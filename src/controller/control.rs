use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Mutex;

use once_cell::sync::Lazy;

use super::cycles::*;
use super::settings::{user_settings, UserSettings};
use crate::hud_layout;
use crate::plugin::*;

/// There can be only one. Not public because we want access managed.
static CONTROLLER: Lazy<Mutex<Controller>> = Lazy::new(|| Mutex::new(Controller::new()));

/// This mod bundles up the public-facing interface of the controller for ease
/// of import into the bridge. I do not want to give the C++ side this object.
pub mod public {
    use super::*;

    /// C++ tells us when it's safe to start pulling together the data we need.
    pub fn initialize_hud() {
        log::info!("initializing hud controller");
        let mut ctrl = CONTROLLER.lock().unwrap();
        let settings = user_settings();
        log::info!("{settings:?}");
        let hud = hud_layout();
        log::info!(
            "hud layout: loc={},{}; size={},{};",
            hud.anchor.x,
            hud.anchor.y,
            hud.size.x,
            hud.size.y
        );

        ctrl.validate_cycles();
        showHUD();
        log::info!("HUD data should be fresh; ready to cycle!")
    }

    /// Function for C++ to call to send a relevant button event to us.
    pub fn handle_key_event(key: u32, button: &ButtonEvent) -> KeyEventResponse {
        let action = Action::from(key);
        if matches!(action, Action::Irrelevant) {
            KeyEventResponse::default()
        } else {
            log::trace!("incoming key event; key={key}; action={action:?}");
            CONTROLLER.lock().unwrap().handle_key_event(action, button)
        }
    }

    /// Function for C++ to call to send a relevant menu button-event to us.
    ///
    /// We get a fully-filled out TesItemData struct to use as we see fit.
    pub fn handle_menu_event(key: u32, menu_item: Box<TesItemData>) {
        let action = Action::from(key);
        CONTROLLER.lock().unwrap().toggle_item(action, *menu_item)
    }

    /// Get information about the item equipped in a specific slot.
    pub fn entry_to_show_in_slot(element: HudElement) -> Box<TesItemData> {
        CONTROLLER.lock().unwrap().entry_to_show_in_slot(element)
    }

    // Handle an equip delay timer expiring.
    pub fn timer_expired(slot: Action) {
        // Fun time! We get to equip an item now!
        let ctrl = CONTROLLER.lock().unwrap();
        ctrl.timer_expired(slot);
    }

    /// Update our view of the player's equipment.
    pub fn update_hud() -> bool {
        let mut ctrl = CONTROLLER.lock().unwrap();
        ctrl.update_hud()
    }

    /// We know for sure the player just equipped this item.
    pub fn handle_item_equipped(equipped: bool, item: Box<TesItemData>) -> bool {
        let mut ctrl = CONTROLLER.lock().unwrap();
        ctrl.handle_item_equipped(equipped, item)
    }

    /// A consumable's count changed. Record if relevant.
    pub fn handle_inventory_changed(item: Box<TesItemData>, count: i32) {
        let mut ctrl = CONTROLLER.lock().unwrap();
        ctrl.handle_inventory_changed(item, count);
        ctrl.update_hud();
    }

    pub fn truncate_cycles(new: u32) {
        let mut ctrl = CONTROLLER.lock().unwrap();
        ctrl.cycles.truncate_if_needed(new as usize);
    }

    pub fn refresh_user_settings() {
        if let Some(e) = UserSettings::refresh().err() {
            log::warn!("Failed to read user settings! using defaults; {e:?}");
            return;
        }
        let mut ctrl = CONTROLLER.lock().unwrap();
        let settings = user_settings();
        if settings.include_unarmed() {
            let h2h = hand_to_hand_item();
            let h2h = *h2h;
            ctrl.cycles.include_item(Action::Left, h2h.clone());
            ctrl.cycles.include_item(Action::Right, h2h);
        } else {
            // remove any item with h2h type from cycles
            ctrl.cycles
                .filter_kind(Action::Left, TesItemKind::HandToHand);
            ctrl.cycles
                .filter_kind(Action::Right, TesItemKind::HandToHand);
        }
        ctrl.flush_cycle_data();
        showHUD();
    }
}

/// What, model/view/controller? In my UI application? oh no
#[derive(Clone, Default, Debug)]
pub struct Controller {
    /// Our currently-active cycles.
    cycles: CycleData,
    /// The items the HUD should show right now.
    visible: HashMap<HudElement, TesItemData>,
    /// True if we've got a two-handed weapon equipped right now.
    two_hander_equipped: bool,
    /// We cache the left-hand item we had before a two-hander arrived.
    left_hand_cached: Option<TesItemData>,
}

impl Controller {
    /// Make a controller. Cycle data is read from disk. Currently-equipped
    /// items are not handled yet.
    pub fn new() -> Self {
        let cycles = CycleData::read().unwrap_or_default();
        Controller {
            cycles,
            ..Default::default()
        }
    }

    pub fn validate_cycles(&mut self) {
        self.cycles.validate();
        log::info!("after validation, cycles are: {}", self.cycles);
        self.update_hud();
    }

    /// The player's inventory changed! Act on it if we need to.
    fn handle_inventory_changed(&mut self, item: Box<TesItemData>, delta: i32) {
        log::info!(
            "inventory count changed; formID={}; count={delta}",
            item.form_string()
        );

        let current = item.count();
        let new_count = if delta.is_negative() {
            if delta > current as i32 {
                0
            } else {
                current - delta.unsigned_abs()
            }
        } else {
            current + delta as u32
        };

        if item.kind() == TesItemKind::Arrow {
            if let Some(candidate) = self.visible.get_mut(&HudElement::Ammo) {
                if *candidate == *item {
                    candidate.set_count(new_count);
                }
            }
        } else {
            if self.cycles.update_count(*item, new_count) {
                self.update_hud();
            }
        }
    }

    /// When the equip delay for a cycle expires, equip the item at the top.
    ///
    /// This function implements a critical function in the mod: equipping
    /// items. When the delay timer expires, we're notified to act on the
    /// player's changes to the cycle rotation. The delay exists to let the
    /// player tap a hotkey repeatedly to look at the items in a cycle without
    /// equipping each one of them as they go. Instead we wait for a little bit,
    /// and if we've had no more hotkey events, we act.
    ///
    /// We do not act here on cascading changes. Instead, we let the equipped-change
    /// callback decide what to do when, e.g., a two-handed item is equipped.
    fn timer_expired(&self, which: Action) {
        let hud = HudElement::from(which);

        if matches!(which, Action::Left) && self.two_hander_equipped {
            // The left hand is blocked because the right hand is equipping a two-hander.
            // TODO honk
            return;
        }

        let Some(item) = &self.visible.get(&hud) else {
            log::warn!("visible item in hud slot was None, which should not happen; slot={:?};", hud);
            unequipSlot(which);
            return;
        };

        // We equip whatever the HUD is showing right now.
        let kind = item.kind();
        if matches!(kind, TesItemKind::HandToHand) {
            log::info!("melee time! unequipping slot {which:?}");
            unequipSlot(which);
            return;
        }

        if matches!(which, Action::Power) {
            // Equip that fus-ro-dah, dovahkin!
            cxx::let_cxx_string!(form_spec = item.form_string());
            equipShout(&form_spec);
            return;
        }

        self.equip_item(item, which);
    }

    /// Convenience function for equipping any equippable.
    fn equip_item(&self, item: &TesItemData, which: Action) {
        if !matches!(which, Action::Right | Action::Left | Action::Utility) {
            return;
        }
        let kind = item.kind();
        cxx::let_cxx_string!(form_spec = item.form_string());
        log::trace!(
            "equip_item: which={:?}; form_spec={}; name='{}'",
            which,
            item.form_string(),
            item.name()
        );

        // These are all different because the game API is a bit of an evolved thing.
        if kind.is_magic() {
            // My name is John Wellington Wells / I'm a dealer in...
            equipMagic(&form_spec, which);
        } else if kind.left_hand_ok() || kind.right_hand_ok() {
            equipWeapon(&form_spec, which);
        } else if kind.is_armor() {
            equipArmor(&form_spec);
        } else if kind == TesItemKind::Arrow {
            equipAmmo(&form_spec);
        } else {
            log::info!(
                "we did nothing with item name='{}'; kind={kind:?};",
                item.name()
            );
        }
    }

    /// Get the item equipped in a specific slot.
    /// Called by the HUD rendering loop in the ImGui code.
    fn entry_to_show_in_slot(&self, slot: HudElement) -> Box<TesItemData> {
        let Some(candidate) = self.visible.get(&slot) else {
            return Box::<TesItemData>::default();
        };

        Box::new(candidate.clone())
    }

    /// Call when loading or otherwise needing to reinitialize the HUD.
    ///
    /// Updates will only happen here if the player changed equipment
    /// out of band, e.g., by using a menu, and only then if we screwed
    /// up an equip event.
    fn update_hud(&mut self) -> bool {
        let right_entry = boundObjectRightHand();
        let right_changed = self.update_slot(HudElement::Right, &right_entry);

        let left_entry = boundObjectLeftHand();
        let left_changed = self.update_slot(HudElement::Left, &left_entry);
        if !left_entry.two_handed() {
            self.left_hand_cached = Some(*left_entry.clone());
        }
        self.two_hander_equipped = left_entry.two_handed();

        let power = equippedPower();
        let power_changed = self.update_slot(HudElement::Power, &power);

        let ammo = equippedAmmo();
        let ammo_changed = self.update_slot(HudElement::Ammo, &ammo);

        let utility_changed = if let Some(utility) = self.cycles.get_top(Action::Utility) {
            log::debug!("utility item starts at name='{}';", utility.name());
            self.update_slot(HudElement::Utility, &utility);
            true
        } else {
            false
        };

        let changed =
            right_changed || left_changed || power_changed || ammo_changed || utility_changed;

        if changed {
            log::info!(
                "visible items changed: power='{}'; left='{}'; right='{}'; ammo='{}';",
                power.name(),
                left_entry.name(),
                right_entry.name(),
                ammo.name(),
            );

            // If any of our equipped items is in a cycle, make that item the top item
            // so advancing the cycles works as expected.
            self.cycles.set_top(Action::Power, *power);
            self.cycles.set_top(Action::Left, *left_entry);
            self.cycles.set_top(Action::Right, *right_entry);
        }

        changed
    }

    /// The game informs us that our equipment has changed. Update.
    ///
    /// The item we're handed was either equipped or UNequipped.
    /// There are some changes we do need to react to, either because
    /// they were done out-of-band of the HUD or because we want to
    /// do more work in reaction to changes we initiated.
    fn handle_item_equipped(&mut self, equipped: bool, item: Box<TesItemData>) -> bool {
        log::info!(
            "item equip status changed; we don't know which hand yet; equipped={}; name='{}'; item.kind={:?}; 2-hander equipped={}; cached={:?}",
            equipped,
            item.name(),
            item.kind(),
            self.two_hander_equipped,
            self.left_hand_cached
        );

        if item.kind().is_utility() {
            // We do nothing. We are the source of truth on the utility view.
            return false;
        }

        let item = *item; // insert unboxing video

        if matches!(item.kind(), TesItemKind::Arrow) {
            log::debug!("handling ammo");
            if let Some(visible) = self.visible.get(&HudElement::Ammo) {
                if visible.form_string() != item.form_string() {
                    log::debug!("updating visible ammo; name='{}';", item.name());
                    self.update_slot(HudElement::Ammo, &item);
                    return true;
                } else {
                    return false;
                }
            } else {
                self.update_slot(HudElement::Ammo, &item);
                return true;
            }
        }

        if item.kind().is_power() {
            log::debug!("handling power/shout");
            if let Some(visible) = self.visible.get(&HudElement::Power) {
                if visible.form_string() != item.form_string() {
                    log::debug!("updating visible power; name='{}';", item.name());
                    self.update_slot(HudElement::Power, &item);
                    self.cycles.set_top(Action::Power, item.clone());
                    return true;
                } else {
                    return false;
                }
            } else {
                self.update_slot(HudElement::Power, &item);
                return true;
            }
        }

        if !item.kind().left_hand_ok() && !item.kind().right_hand_ok() {
            return false;
        }

        let rightie = if !item.kind().is_weapon() {
            equippedRightHand()
        } else {
            boundObjectRightHand()
        };

        let leftie = if !item.kind().is_weapon() {
            equippedLeftHand()
        } else {
            boundObjectLeftHand()
        };

        log::trace!(
            "form strings: item={}; right={}; left={}; two-hander-equipped={};",
            item.form_string(),
            rightie.form_string(),
            leftie.form_string(),
            self.two_hander_equipped
        );

        if rightie.form_string() == item.form_string() {
            // We are equipping this item in the right hand.
            let right_changed = self.handle_right_hand_event(item.clone());
            if self.two_hander_equipped && !item.two_handed() {
                if let Some(prev_left) = self.left_hand_cached.clone() {
                    log::debug!(
                        "contemplating forcing re-equip here; name='{}';",
                        prev_left.name()
                    );
                    // cxx::let_cxx_string!(form_spec = prev_left.form_string());
                    // reequipLeftHand(&form_spec);
                }
            }
            return right_changed;
        }

        if leftie.form_string() == item.form_string() {
            // We are equipping this item in the left hand.
            return self.handle_left_hand_event(item);
        }

        false // we really shouldn't reach this, but
    }

    fn handle_right_hand_event(&mut self, item: TesItemData) -> bool {
        log::debug!(
            "entering RIGHT hand event; name='{}'; item is two-handed={}; two_hander_equipped={}",
            item.name(),
            item.two_handed(),
            self.two_hander_equipped
        );

        // Tracking two-handed/one-handed transitions.
        // We get 2-handed equip events for the right hand only, so we
        // do that bookkeeping here.
        if item.two_handed() {
            // only set; do not unset.
            self.two_hander_equipped = true;
            self.update_slot(HudElement::Left, &TesItemData::default());
        }

        if let Some(visible) = self.visible.get(&HudElement::Right) {
            if visible.form_string() != item.form_string() {
                log::debug!("updating visible right-hand item; name='{}';", item.name());
                self.cycles.set_top(Action::Right, item.clone());
                self.update_slot(HudElement::Right, &item);
                return true;
            } else {
                return false;
            }
        }

        // somehow nothing was in there
        self.cycles.set_top(Action::Right, item.clone());
        self.update_slot(HudElement::Right, &item);
        true
    }

    fn handle_left_hand_event(&mut self, item: TesItemData) -> bool {
        let left_prev = self.visible.get(&HudElement::Left);
        log::debug!(
            "entering LEFT hand event; name='{}'; item is two-handed={}; two_hander_equipped={}",
            item.name(),
            item.two_handed(),
            self.two_hander_equipped
        );

        // We don't care if it's visible or not. If we've equipped
        // a one-hander in the left hand, we record it.
        if !item.two_handed() && self.two_hander_equipped {
            log::trace!("forcing left-hand re-equip");
            self.two_hander_equipped = false;
            cxx::let_cxx_string!(form_spec = item.form_string());
            reequipLeftHand(&form_spec);
            log::trace!("updating cached left hand item");
            self.left_hand_cached = Some(item.clone());
        }

        // We do not show two-handed items in the left hand, though.
        if item.two_handed() {
            if let Some(visible) = left_prev {
                if visible.form_string().is_empty() {
                    return false; // no change from previous state
                }
            }
            let insert_this = TesItemData::default();
            self.update_slot(HudElement::Left, &insert_this);
            return true;
        }

        let update = if let Some(visible) = left_prev {
            if visible.form_string() != item.form_string() {
                log::debug!("equipped left-hand item; name='{}';", item.name());
                true
            } else {
                false
            }
        } else {
            true
        };

        if update {
            self.cycles.set_top(Action::Left, item.clone());
            self.update_slot(HudElement::Left, &item);
        }
        update
    }

    fn update_slot(&mut self, slot: HudElement, new_item: &TesItemData) -> bool {
        if let Some(replaced) = self.visible.insert(slot, new_item.clone()) {
            replaced != *new_item
        } else {
            false
        }
    }

    /// Handle a key-press event that the event system decided we need to know about.
    ///
    /// Returns an enum indicating what we did in response, in case one of the calling
    /// layers wants to show UI or play sounds in response.
    fn handle_key_event(&mut self, which: Action, _button: &ButtonEvent) -> KeyEventResponse {
        if matches!(which, Action::Irrelevant) {
            return KeyEventResponse::default();
        }
        // log::trace!("entering handle_key_event(); action={which:?}");

        // It's not really tidier rewritten as a match.

        if matches!(which, Action::ShowHide) {
            log::trace!("doing Action:ShowHide");
            toggleHUD();
            return KeyEventResponse {
                handled: true,
                ..Default::default()
            };
        } else {
            // If we're faded out in any way, show ourselves again, because we're about to do something.
            if user_settings().fade() && getIsFading() {
                showHUD();
            }
        }

        if matches!(
            which,
            Action::Power | Action::Left | Action::Right | Action::Utility
        ) {
            if matches!(which, Action::Left) && self.two_hander_equipped {
                log::info!("declining to advance left-hand cycle while two-hander equipped");
                return KeyEventResponse {
                    handled: true,
                    ..Default::default()
                };
            }

            let hud = HudElement::from(which);
            if self.cycles.cycle_len(which) > 1 {
                if let Some(next) = self.cycles.advance(which, 1) {
                    self.update_slot(hud, &next);
                    self.flush_cycle_data();
                    showHUD();
                }
                return KeyEventResponse {
                    handled: true,
                    start_timer: if !matches!(which, Action::Utility) {
                        which
                    } else {
                        Action::Irrelevant
                    },
                    stop_timer: Action::Irrelevant,
                };
            } else {
                return KeyEventResponse {
                    handled: true,
                    ..Default::default()
                };
            }
        } else if matches!(which, Action::Activate) {
            return self.use_utility_item();
        } else if matches!(which, Action::RefreshLayout) {
            HudLayout::refresh();
            showHUD();
            return KeyEventResponse {
                handled: true,
                ..Default::default()
            };
        }

        // unreachable tbh
        KeyEventResponse::default()
    }

    /// Activate whatever we have in the utility slot.
    fn use_utility_item(&mut self) -> KeyEventResponse {
        log::trace!("using utility item");
        if let Some(item) = self.cycles.get_top(Action::Utility) {
            if item.kind().is_potion()
                || matches!(item.kind(), TesItemKind::PoisonDefault | TesItemKind::Food)
            {
                cxx::let_cxx_string!(form_spec = item.form_string());
                consumePotion(&form_spec);
            } else if item.kind().is_armor() {
                cxx::let_cxx_string!(form_spec = item.form_string());
                equipArmor(&form_spec);
            }
        }

        // No matter what we did, we stop the timer. Not that a timer should exist.
        KeyEventResponse {
            handled: true,
            start_timer: Action::Irrelevant,
            stop_timer: Action::Utility,
        }
    }

    /// This function is called when the player has pressed a hot key while hovering over an
    /// item in a menu. We'll remove the item if it's already in the matching cycle,
    /// or add it if it's an appropriate item. We signal back to the UI layer what we did.
    fn toggle_item(&mut self, action: Action, item: TesItemData) {
        let result = self.cycles.toggle(action, item.clone());

        // notify the player what happened...
        let verb = match result {
            MenuEventResponse::ItemAdded => "added to",
            MenuEventResponse::ItemRemoved => "removed from",
            _ => "not changed in",
        };
        let cyclename = match action {
            Action::Power => "powers",
            Action::Left => "left-hand",
            Action::Right => "right-hand",
            Action::Utility => "utility items",
            _ => "any",
        };
        let message = format!("{} {} {} cycle", item.name(), verb, cyclename);
        cxx::let_cxx_string!(msg = message);
        notifyPlayer(&msg);

        if matches!(
            result,
            MenuEventResponse::ItemAdded | MenuEventResponse::ItemRemoved
        ) {
            // the data changed. flush it to disk
            log::debug!(
                "persisted cycle data after change; verb='{}'; item='{}';",
                verb,
                item.name()
            );
            self.flush_cycle_data();
        }
    }

    fn flush_cycle_data(&self) {
        match self.cycles.write() {
            Ok(_) => {}
            Err(e) => {
                log::warn!("failed to persist cycle data, but gamely continuing; {e:?}");
            }
        }
    }
}

impl Default for KeyEventResponse {
    fn default() -> Self {
        Self {
            handled: false,
            stop_timer: Action::Irrelevant,
            start_timer: Action::Irrelevant,
        }
    }
}

/// All this converting makes me suspect the abstraction is wrong.
impl From<Action> for HudElement {
    fn from(value: Action) -> Self {
        if value == Action::Power {
            HudElement::Power
        } else if value == Action::Utility {
            HudElement::Utility
        } else if value == Action::Left {
            HudElement::Left
        } else if value == Action::Right {
            HudElement::Right
        } else {
            HudElement::Ammo
        }
    }
}

impl Display for HudElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            HudElement::Ammo => write!(f, "Ammo"),
            HudElement::Left => write!(f, "Left"),
            HudElement::Power => write!(f, "Power"),
            HudElement::Right => write!(f, "Right"),
            HudElement::Utility => write!(f, "Utility"),
            _ => write!(f, "unknown"),
        }
    }
}

impl From<u32> for Action {
    /// Turn the key code into an enum for easier processing.
    fn from(value: u32) -> Self {
        let settings = user_settings();

        if value == settings.left {
            Action::Left
        } else if value == settings.right {
            Action::Right
        } else if value == settings.power {
            Action::Power
        } else if value == settings.utility {
            Action::Utility
        } else if value == settings.activate {
            Action::Activate
        } else if value == settings.showhide {
            Action::ShowHide
        } else if value == settings.refresh_layout {
            Action::RefreshLayout
        } else {
            Action::Irrelevant
        }
    }
}

/// What the controller did with a specific menu
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum MenuEventResponse {
    Okay,
    #[default]
    Unhandled,
    Error,
    ItemAdded,
    ItemRemoved,
    ItemInappropriate,
    TooManyItems,
}
