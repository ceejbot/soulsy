use std::collections::HashMap;
use std::sync::Mutex;

use once_cell::sync::Lazy;

use super::cycles::*;
use super::settings::user_settings;
use crate::layout;
use crate::plugin::*;

/// There can be only one. Not public because we want access managed.
// Does this really need to be a mutex? I think we're single-threaded...
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
        let hud = layout();
        log::info!(
            "hud layout: loc={},{}; size={},{};",
            hud.anchor.x,
            hud.anchor.y,
            hud.size.x,
            hud.size.y
        );

        ctrl.validate_cycles();
        if ctrl.update_hud() {
            show_hud();
        }
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
    pub fn handle_menu_event(key: u32, menu_item: Box<TesItemData>) -> MenuEventResponse {
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
    pub fn handle_item_equipped(item: Box<TesItemData>) -> bool {
        let mut ctrl = CONTROLLER.lock().unwrap();
        ctrl.handle_item_equipped(item)
    }

    /// A consumable's count changed. Record if relevant.
    pub fn handle_inventory_changed(item: Box<TesItemData>, count: usize) {
        let mut ctrl = CONTROLLER.lock().unwrap();
        ctrl.handle_inventory_changed(item, count);
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
    }

    /// The player's inventory changed! Act on it if we need to.
    fn handle_inventory_changed(&mut self, item: Box<TesItemData>, count: usize) {
        log::info!(
            "inventory count changed; formid={}; count={count}",
            item.form_string()
        );

        if item.kind() == TesItemKind::Arrow {
            if let Some(candidate) = self.visible.get_mut(&HudElement::Ammo) {
                if *candidate == *item {
                    candidate.set_count(item.count());
                }
            }
        } else {
            self.cycles.update_count(*item, count);
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

        // We equip whatever the HUD is showing right now.
        let Some(item) = &self.visible.get(&hud) else {
            return;
        };

        let kind = item.kind();
        if matches!(kind, TesItemKind::Empty) && which != Action::Utility {
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
                "we did nothing with item name={}; kind={kind:?};",
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
        let right_entry = equippedRightHand();
        let right_changed = self.update_slot(HudElement::Right, &right_entry);

        let left_entry = equippedLeftHand();
        let left_changed = self.update_slot(HudElement::Left, &left_entry);

        let power = equippedPower();
        let power_changed = self.update_slot(HudElement::Power, &power);

        let ammo = equippedAmmo();
        let ammo_changed = self.update_slot(HudElement::Ammo, &ammo);

        let changed = right_changed || left_changed || power_changed || ammo_changed;

        if changed {
            log::info!(
                "visible items changed: power='{}'; left='{}'; right='{}'; ammo='{}';",
                power.name(),
                left_entry.name(),
                right_entry.name(),
                ammo.name()
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
    /// The item we're handed was either equipped or UNequipped. We are not
    /// told which. There are some changes we do need to react to.
    fn handle_item_equipped(&mut self, item: Box<TesItemData>) -> bool {
        log::info!(
            "entering handle_item_equipped with obj; name='{}'; form_spec={};",
            item.name(),
            item.form_string()
        );

        if item.kind() == TesItemKind::Arrow {
            let ammo = equippedAmmo();
            log::info!(
                "three kinds of arrow walk into a bar; equipped={}; item={};",
                ammo.name(),
                item.name()
            );
            if ammo.form_string() == item.form_string() {
                // we are equipping this.
                self.visible.insert(HudElement::Power, *item);
                return true;
            } else {
                // We are unequipping it, and will wait for the equipping update.
                return false;
            }
        }

        if item.kind().is_utility() {
            // We do nothing. We are the source of truth on the utility view.
            return false;
        }

        if item.kind().is_power() {
            if let Some(previous) = self.visible.get(&HudElement::Power) {
                if previous.form_string() != item.form_string() {
                    // Trying the other logic thingie for powers.
                    self.visible.insert(HudElement::Power, *item.clone());
                    self.cycles.set_top(Action::Power, *item);
                    return true;
                } else {
                    return false;
                }
            } else {
                self.visible.insert(HudElement::Power, *item);
                return true;
            }
        }

        if !item.kind().left_hand_ok() && !item.kind().right_hand_ok() {
            return false;
        }

        let rightie = equippedRightHand();
        let leftie = equippedLeftHand();

        if (item.form_string() != rightie.form_string())
            && (item.form_string() != leftie.form_string())
        {
            // We are unequipping this item. We do nothing.
            return false;
        }

        if rightie.form_string() == item.form_string() {
            // We are equipping this item in the right hand.
            let right_prev = self.visible.get(&HudElement::Right);
            if let Some(visible) = right_prev {
                if visible.form_string() != item.form_string() {
                    self.visible.insert(HudElement::Right, *item.clone());
                    self.cycles.set_top(Action::Right, *item);
                    return true;
                } else {
                    return false;
                }
            } else {
                self.visible.insert(HudElement::Right, *item.clone());
                self.cycles.set_top(Action::Right, *item);
                return true;
            }
        } else if leftie.form_string() == item.form_string() {
            // We are equipping this item in the left hand.
            let left_prev = self.visible.get(&HudElement::Left);

            // We do not show two-handed items in the left hand, though.
            if item.two_handed() {
                if let Some(visible) = left_prev {
                    if visible.form_string().is_empty() {
                        return false; // no change from previous state
                    }
                }
                let insert_this = TesItemData::default();
                self.visible.insert(HudElement::Left, insert_this);
                return true;
            }
            self.two_hander_equipped = item.two_handed();

            if let Some(visible) = left_prev {
                if visible.form_string() != item.form_string() {
                    self.visible.insert(HudElement::Left, *item.clone());
                    self.cycles.set_top(Action::Left, *item);
                    return true;
                } else {
                    return false;
                }
            } else {
                self.visible.insert(HudElement::Left, *item.clone());
                self.cycles.set_top(Action::Left, *item);
                return true;
            }
        }
        false
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
        log::debug!("entering handle_key_event(); action={which:?}");

        // It's not really tidier rewritten as a match.

        if matches!(which, Action::ShowHide) {
            log::debug!("doing Action:ShowHide");
            toggle_hud_visibility();
            return KeyEventResponse {
                handled: true,
                ..Default::default()
            };
        } else {
            // If we're faded out in any way, show ourselves again, because we're about to do something.
            if user_settings().fade() && get_is_transitioning() {
                show_hud();
            }
        }

        if matches!(
            which,
            Action::Power | Action::Left | Action::Right | Action::Utility
        ) {
            let hud = HudElement::from(which);
            if self.cycles.cycle_len(which) > 1 {
                if let Some(next) = self.cycles.advance(which, 1) {
                    self.update_slot(hud, &next);
                    show_hud();
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
            show_hud();
            return KeyEventResponse {
                handled: true,
                ..Default::default()
            };
        }

        // unreachable tbh
        return KeyEventResponse::default();
    }

    /// Activate whatever we have in the utility slot.
    fn use_utility_item(&mut self) -> KeyEventResponse {
        log::debug!("using utility item (possibly crashy)");
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
    fn toggle_item(&mut self, action: Action, item: TesItemData) -> MenuEventResponse {
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
        notify_player(&msg);

        if matches!(
            result,
            MenuEventResponse::ItemAdded | MenuEventResponse::ItemRemoved
        ) {
            // the data changed. flush it to disk with char name in it or something
            match self.cycles.write() {
                Ok(_) => log::info!(
                    "persisted cycle data after change; verb='{}'; item='{}';",
                    verb,
                    item.name()
                ),
                Err(e) => {
                    log::warn!("failed to persist cycle data, but gamely continuing; {e:?}");
                }
            }
        }

        result
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
