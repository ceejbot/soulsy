use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Mutex;

use once_cell::sync::Lazy;

use super::cycles::*;
use super::keys::*;
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
        let mut ctrl = CONTROLLER
            .lock()
            .expect("Unrecoverable runtime problem: cannot acquire controller lock. Exiting.");
        let settings = user_settings();
        log::info!("{settings:?}");
        let _hud = hud_layout();
        ctrl.validate_cycles();
        log::info!("HUD data should be fresh; ready to cycle!")
    }

    /// Function for C++ to call to send a relevant button event to us.
    pub fn handle_key_event(key: u32, button: &ButtonEvent) -> KeyEventResponse {
        CONTROLLER
            .lock()
            .expect("Unrecoverable runtime problem: cannot acquire controller lock. Exiting.")
            .handle_key_event(key, button)
    }

    pub fn show_ui() -> bool {
        CONTROLLER
            .lock()
            .expect("Unrecoverable runtime problem: cannot acquire controller lock. Exiting.")
            .cycles
            .hud_visible()
    }

    /// Function for C++ to call to send a relevant menu button-event to us.
    ///
    /// We get a fully-filled out TesItemData struct to use as we see fit.
    // menu_item is boxed because it's arriving from C++.
    #[allow(clippy::boxed_local)]
    pub fn handle_menu_event(key: u32, #[allow(clippy::boxed_local)] menu_item: Box<TesItemData>) {
        let action = Action::from(key);
        CONTROLLER
            .lock()
            .expect("Unrecoverable runtime problem: cannot acquire controller lock. Exiting.")
            .toggle_item(action, *menu_item)
    }

    /// Get information about the item equipped in a specific slot.
    pub fn entry_to_show_in_slot(element: HudElement) -> Box<TesItemData> {
        CONTROLLER
            .lock()
            .expect("Unrecoverable runtime problem: cannot acquire controller lock. Exiting.")
            .entry_to_show_in_slot(element)
    }

    // Handle an equip delay timer expiring.
    pub fn timer_expired(slot: Action) {
        // Fun time! We get to equip an item now!
        let mut ctrl = CONTROLLER
            .lock()
            .expect("Unrecoverable runtime problem: cannot acquire controller lock. Exiting.");
        ctrl.timer_expired(slot);
    }

    /// Update our view of the player's equipment.
    pub fn update_hud() -> bool {
        let mut ctrl = CONTROLLER
            .lock()
            .expect("Unrecoverable runtime problem: cannot acquire controller lock. Exiting.");
        ctrl.update_hud()
    }

    /// We know for sure the player just equipped this item.
    pub fn handle_item_equipped(equipped: bool, item: Box<TesItemData>) -> bool {
        let mut ctrl = CONTROLLER
            .lock()
            .expect("Unrecoverable runtime problem: cannot acquire controller lock. Exiting.");
        ctrl.handle_item_equipped(equipped, item)
    }

    /// A consumable's count changed. Record if relevant.
    pub fn handle_inventory_changed(item: Box<TesItemData>, count: i32) {
        let mut ctrl = CONTROLLER
            .lock()
            .expect("Unrecoverable runtime problem: cannot acquire controller lock. Exiting.");
        ctrl.handle_inventory_changed(item, count);
        ctrl.update_hud();
    }

    pub fn truncate_cycles(new: u32) {
        let mut ctrl = CONTROLLER
            .lock()
            .expect("Unrecoverable runtime problem: cannot acquire controller lock. Exiting.");
        ctrl.cycles.truncate_if_needed(new as usize);
    }

    pub fn refresh_user_settings() {
        if let Some(e) = UserSettings::refresh().err() {
            log::warn!("Failed to read user settings! using defaults; {e:?}");
            return;
        }
        let mut ctrl = CONTROLLER
            .lock()
            .expect("Unrecoverable runtime problem: cannot acquire controller lock. Exiting.");
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
    /// We cache the left-hand item we had before a two-hander was equipped.
    left_hand_cached: Option<TesItemData>,
    /// We cache the right-hand item we had similarly.
    right_hand_cached: Option<TesItemData>,
    /// State of the cycle modifier hotkey
    cycle_modifier: TrackedKey,
    unequip_modifier: TrackedKey,
    activate_modifier: TrackedKey,
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
    fn handle_inventory_changed(
        &mut self,
        #[allow(clippy::boxed_local)] item: Box<TesItemData>, // boxed because arriving from C++
        delta: i32,
    ) {
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
        } else if self.cycles.update_count(*item, new_count) {
            self.update_hud();
        }
    }

    /// Handle a key-press event that the event system decided we need to know about.
    ///
    /// Returns an enum indicating what we did in response, so that the C++ layer can
    /// start a tick timer for cycle delay.
    fn handle_key_event(&mut self, key: u32, button: &ButtonEvent) -> KeyEventResponse {
        let keyevent = TrackedKey {
            key: HotkeyKind::from(key),
            state: KeyState::from(button),
        };
        if matches!(keyevent.key, HotkeyKind::None) {
            return KeyEventResponse::default();
        }
        if !button.IsUp() && !button.IsDown() {
            return KeyEventResponse::default();
        }

        log::trace!("incoming key={};", keyevent);

        if matches!(keyevent.key, HotkeyKind::CycleModifier) {
            self.cycle_modifier = keyevent;
            return KeyEventResponse::default();
        }
        if matches!(keyevent.key, HotkeyKind::UnequipModifier) {
            self.unequip_modifier = keyevent;
            return KeyEventResponse::default();
        }
        if matches!(keyevent.key, HotkeyKind::ActivateModifier) {
            self.activate_modifier = keyevent;
            return KeyEventResponse::default();
        }

        match keyevent.key {
            HotkeyKind::Power => return self.handle_cycle_power(),
            HotkeyKind::Utility => return self.handle_cycle_utility(),
            HotkeyKind::Left => return self.handle_cycle_hand(keyevent),
            HotkeyKind::Right => return self.handle_cycle_hand(keyevent),
            HotkeyKind::Activate => return self.use_utility_item(),
            HotkeyKind::Refresh => {
                HudLayout::refresh();
                return KeyEventResponse {
                    handled: true,
                    ..Default::default()
                };
            }
            HotkeyKind::ShowHide => {
                self.cycles.toggle_hud();
                return KeyEventResponse {
                    handled: true,
                    ..Default::default()
                };
            }
            HotkeyKind::UnequipModifier => {
                self.unequip_modifier = keyevent;
                return KeyEventResponse::default();
            }
            HotkeyKind::CycleModifier => {
                self.cycle_modifier = keyevent;
                return KeyEventResponse::default();
            }
            HotkeyKind::ActivateModifier => {
                self.activate_modifier = keyevent;
                return KeyEventResponse::default();
            }
            HotkeyKind::None => {
                return KeyEventResponse::default();
            }
        }
    }

    fn handle_cycle_power(&mut self) -> KeyEventResponse {
        if self.cycle_modifier.ignore() || self.cycle_modifier.is_pressed() {
            self.advance_simple_cycle(Action::Power)
        } else {
            log::info!("declining to advance power/shouts cycle without the modifier key down");
            KeyEventResponse::default()
        }
    }

    fn handle_cycle_utility(&mut self) -> KeyEventResponse {
        if self.cycle_modifier.ignore() || self.cycle_modifier.is_pressed() {
            self.advance_simple_cycle(Action::Utility)
        } else {
            log::info!(
                "declining to advance utility/consumables cycle without the modifier key down"
            );
            KeyEventResponse::default()
        }
    }

    fn advance_simple_cycle(&mut self, which: Action) -> KeyEventResponse {
        // Programmer error to call this for left/right.
        if matches!(which, Action::Power | Action::Utility) {
            log::info!("Programmer error! This is not a simple cycle. cycle={which:?}",);
            return KeyEventResponse::default();
        }
        if self.cycles.cycle_len(which) <= 1 {
            // TODO failure sound
            return KeyEventResponse {
                handled: true,
                ..Default::default()
            };
        }

        let candidate = self.cycles.advance(which, 1);
        if let Some(next) = candidate {
            let hud = HudElement::from(which);
            self.update_slot(hud, &next);
            self.flush_cycle_data();
            KeyEventResponse {
                handled: true,
                start_timer: if !matches!(which, Action::Utility) {
                    which
                } else {
                    Action::None
                },
                stop_timer: Action::None,
            }
        } else {
            KeyEventResponse {
                handled: true,
                ..Default::default()
            }
        }
    }

    fn handle_cycle_hand(&mut self, event: TrackedKey) -> KeyEventResponse {
        // We have two modifiers to check
        let unequip_requested =
            self.unequip_modifier.ignore() || self.unequip_modifier.is_pressed();
        let cycle_requested = if self.cycle_modifier.ignore() {
            !unequip_requested
        } else {
            !unequip_requested && self.cycle_modifier.is_pressed()
        };

        // ETOOMANYENUMS
        // The root problem is that shared enums are not sum types, and I want sum types.
        let hudslot = HudElement::from(&event.key);
        let action = Action::from(&event.key);

        if unequip_requested {
            log::info!("unequipping slot {:?} by request!", action);
            let empty_item = *hand_to_hand_item();
            unequipSlot(action);
            self.update_slot(hudslot, &empty_item);
            self.cycles.set_top(action, &empty_item);
            KeyEventResponse {
                handled: true,
                start_timer: Action::None,
                stop_timer: action,
            }
        } else if cycle_requested {
            self.advance_hand_cycle(action)
        } else {
            // TODO honk
            log::info!("you need a modifier key down for {action:?}");
            KeyEventResponse::default()
        }
    }

    fn advance_hand_cycle(&mut self, which: Action) -> KeyEventResponse {
        if self.cycles.cycle_len(which) <= 1 {
            // TODO failure sound
            return KeyEventResponse {
                handled: true,
                ..Default::default()
            };
        }

        let candidate = if matches!(which, Action::Left) {
            if !self.two_hander_equipped {
                let rightie = equippedRightHand();
                log::info!("right equipped count: {};", rightie.count());
                if rightie.has_count() && rightie.count() == 1 {
                    self.cycles.advance_skipping(which, *rightie)
                } else {
                    self.cycles.advance(which, 1)
                }
            } else if let Some(cached_right) = &self.right_hand_cached {
                // We skip whatever the right hand is going to re-equip if we have only 1.
                log::info!("right cached count: {};", cached_right.count());
                if cached_right.has_count() && cached_right.count() == 1 {
                    self.cycles.advance_skipping(which, cached_right.clone())
                } else {
                    self.cycles.advance(which, 1)
                }
            } else {
                self.cycles.advance(which, 1)
            }
        } else {
            // this is the matches!(Right) block
            if !self.two_hander_equipped {
                let leftie = equippedLeftHand();
                log::info!("left equipped count: {};", leftie.count());
                if leftie.has_count() && leftie.count() == 1 {
                    self.cycles.advance_skipping(which, *leftie)
                } else {
                    self.cycles.advance(which, 1)
                }
            } else if let Some(cached_left) = &self.left_hand_cached {
                // We skip whatever the left hand is going to re-equip if there's only 1
                log::info!("left cached count: {};", cached_left.count());
                if cached_left.has_count() && cached_left.count() == 1 {
                    self.cycles.advance_skipping(which, cached_left.clone())
                } else {
                    self.cycles.advance(which, 1)
                }
            } else {
                // Nothing cached, two-hander equipped.
                self.cycles.advance(which, 1)
            }
        };

        if let Some(next) = candidate {
            let hud = HudElement::from(which);
            self.update_slot(hud, &next);
            self.flush_cycle_data();
            KeyEventResponse {
                handled: true,
                start_timer: if !matches!(which, Action::Utility) {
                    which
                } else {
                    Action::None
                },
                stop_timer: Action::None,
            }
        } else {
            KeyEventResponse {
                handled: true,
                ..Default::default()
            }
        }
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
            } else if item.kind().is_ammo() {
                // TODO equip ammo
            }
        }

        // No matter what we did, we stop the timer. Not that a timer should exist.
        KeyEventResponse {
            handled: true,
            start_timer: Action::None,
            stop_timer: Action::Utility,
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
    fn timer_expired(&mut self, which: Action) {
        let hud = HudElement::from(which);

        let Some(item) = &self.visible.get(&hud) else {
            log::warn!(
                "visible item in hud slot was None, which should not happen; slot={:?};",
                hud
            );
            unequipSlot(which);
            return;
        };

        // We equip whatever the HUD is showing right now.
        let kind = item.kind();
        if matches!(kind, TesItemKind::HandToHand) {
            log::info!("melee time! unequipping slot {which:?}");
            if which == Action::Left {
                self.left_hand_cached = Some(*hand_to_hand_item());
            } else {
                self.right_hand_cached = Some(*hand_to_hand_item());
            }
            unequipSlot(which);
            return;
        }

        if matches!(which, Action::Power) {
            // Equip that fus-ro-dah, dovahkin!
            log::info!("Fus-ro-dah!");
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

    /// The game informs us that our equipment has changed. Update.
    ///
    /// The item we're handed was either equipped or UNequipped.
    /// There are some changes we do need to react to, either because
    /// they were done out-of-band of the HUD or because we want to
    /// do more work in reaction to changes we initiated.
    fn handle_item_equipped(
        &mut self,
        equipped: bool,
        #[allow(clippy::boxed_local)] item: Box<TesItemData>, // boxed because arriving from C++
    ) -> bool {
        log::debug!("========== start handle_item_equipped() ==========");
        log::debug!("is-now-equipped={}; name='{}'; item.kind={:?}; 2-hander equipped={}; left_cached='{}'; right_cached='{}';",
            equipped,
            item.name(),
            item.kind(),
            self.two_hander_equipped,
            self.left_hand_cached.clone().map_or("".to_string(), |xs| xs.name()),
            self.right_hand_cached.clone().map_or("".to_string(), |xs| xs.name())
        );

        if item.kind().is_utility() {
            // We do nothing. We are the source of truth on the utility view.
            return false;
        }

        if !equipped {
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
                    self.cycles.set_top(Action::Power, &item);
                    return true;
                } else {
                    return false;
                }
            } else {
                self.update_slot(HudElement::Power, &item);
                return true;
            }
        }

        // ---------- The hard part starts. Left hand vs right hand.

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

        log::debug!(
            "form strings: item={}; right={}; left={}; two-hander-equipped={};",
            item.form_string(),
            rightie.form_string(),
            leftie.form_string(),
            self.two_hander_equipped
        );

        let leftvis = self
            .visible
            .get(&HudElement::Left)
            .map_or("".to_string(), |xs| xs.form_string());
        let rightvis = self
            .visible
            .get(&HudElement::Right)
            .map_or("".to_string(), |xs| xs.form_string());

        // This logic is more complicated than you'd think.
        let is_right_hand = if item.two_handed() == self.two_hander_equipped {
            rightvis == item.form_string()
        } else {
            // if we're swapping from having only 1 item equipped to 2, prev test
            // will always come back as true.
            leftvis != item.form_string()
        };

        log::debug!("====== is right hand={is_right_hand}");

        // First, update the hud as usual.
        let (hudslot, action) = if is_right_hand {
            (HudElement::Right, Action::Right)
        } else {
            (HudElement::Left, Action::Left)
        };

        let changed = if let Some(visible) = self.visible.get(&hudslot) {
            if visible.form_string() != item.form_string() {
                log::debug!(
                    "shown item in hand; name='{}'; is-right-hand={}",
                    item.name(),
                    is_right_hand
                );
                self.cycles.set_top(action, &item);
                self.update_slot(hudslot, &item);
                true
            } else {
                false
            }
        } else {
            // somehow nothing was in there
            self.cycles.set_top(action, &item);
            self.update_slot(hudslot, &item);
            true
        };

        if !item.two_handed() {
            log::info!(
                "updating is-right={} cached item to {}",
                is_right_hand,
                item.name()
            );
            if is_right_hand {
                self.right_hand_cached = Some(item.clone());
            } else {
                self.left_hand_cached = Some(item.clone());
            }
        }

        if item.two_handed() && !self.two_hander_equipped {
            self.two_hander_equipped = true;
            // and show the left hand as empty
            self.update_slot(HudElement::Left, &TesItemData::default());
        }

        if !item.two_handed() && self.two_hander_equipped {
            log::info!("transitioning from 2-hander to 1-hander.");
            self.two_hander_equipped = false;

            if is_right_hand {
                log::info!("right hand transitioning case");
                if let Some(prev_left) = self.left_hand_cached.clone() {
                    // We won't get a good event to let us trigger this later.
                    log::info!("considering re-requipping or unequipping LEFT");
                    if prev_left.kind() == TesItemKind::HandToHand {
                        unequipSlot(Action::Left);
                        self.update_slot(HudElement::Left, &prev_left);
                    } else {
                        log::info!(
                            "!!!!!!!! should be re-equipping left; name='{}';",
                            prev_left.name()
                        );
                        cxx::let_cxx_string!(form_spec = prev_left.form_string());
                        reequipHand(Action::Left, &form_spec);
                        self.update_slot(HudElement::Left, &prev_left);
                    }
                }
            } else {
                if let Some(prev_right) = self.right_hand_cached.clone() {
                    // We won't get a good event to let us trigger this later.
                    log::info!("considering re-requipping or unequipping RIGHT");
                    if prev_right.kind() == TesItemKind::HandToHand {
                        unequipSlot(Action::Right);
                        self.update_slot(HudElement::Right, &prev_right);
                    } else {
                        log::info!(
                            "!!!!!!!! should be re-equipping right; name='{}';",
                            prev_right.name()
                        );
                        cxx::let_cxx_string!(form_spec = prev_right.form_string());
                        reequipHand(Action::Right, &form_spec);
                    }
                }
            };
        }
        self.two_hander_equipped = item.two_handed();
        log::info!("---------- exiting handle_item_equipped");
        changed
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
        if !right_entry.two_handed() {
            self.right_hand_cached = Some(*right_entry.clone());
        }

        let left_entry = boundObjectLeftHand();
        let left_changed = self.update_slot(HudElement::Left, &left_entry);
        if !left_entry.two_handed() {
            self.left_hand_cached = Some(*left_entry.clone());
        }
        self.two_hander_equipped = right_entry.two_handed(); // same item will be in both hands

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
            self.cycles.set_top(Action::Power, &power);
            self.cycles.set_top(Action::Left, &left_entry);
            self.cycles.set_top(Action::Right, &right_entry);
        }

        changed
    }

    fn update_slot(&mut self, slot: HudElement, new_item: &TesItemData) -> bool {
        if let Some(replaced) = self.visible.insert(slot, new_item.clone()) {
            replaced != *new_item
        } else {
            false
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
            stop_timer: Action::None,
            start_timer: Action::None,
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
            Action::None
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
