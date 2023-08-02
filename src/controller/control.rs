use std::collections::HashMap;
use std::sync::Mutex;

use once_cell::sync::Lazy;

use super::cycles::*;
use super::itemdata::*;
use super::keys::*;
use super::settings::{user_settings, ActivationMethod, UnarmedMethod};
use crate::plugin::*;

/// There can be only one. Not public because we want access managed.
static CONTROLLER: Lazy<Mutex<Controller>> = Lazy::new(|| Mutex::new(Controller::new()));

pub fn get() -> std::sync::MutexGuard<'static, Controller> {
    CONTROLLER
        .lock()
        .expect("Unrecoverable runtime problem: cannot acquire controller lock. Exiting.")
}

/// What, model/view/controller? In my UI application? oh no
#[derive(Clone, Default, Debug)]
pub struct Controller {
    /// Our currently-active cycles.
    pub cycles: CycleData,
    /// The items the HUD should show right now.
    visible: HashMap<HudElement, ItemData>,
    /// True if we've got a two-handed weapon equipped right now.
    two_hander_equipped: bool,
    /// We cache the left-hand item we had before a two-hander was equipped.
    left_hand_cached: Option<ItemData>,
    /// We cache the right-hand item we had similarly.
    right_hand_cached: Option<ItemData>,
    tracked_keys: HashMap<HotkeyKind, TrackedKey>,
}

impl Controller {
    /// Make a controller. Cycle data is read from disk. Currently-equipped
    /// items are not handled yet.
    pub fn new() -> Self {
        let cycles = CycleData::default();
        Controller {
            cycles,
            ..Default::default()
        }
    }

    pub fn validate_cycles(&mut self) {
        self.cycles.validate();
        // log::info!("after validation, cycles are: {}", self.cycles);
        self.update_hud();
    }

    pub fn apply_settings(&mut self) {
        let settings = user_settings();

        match settings.unarmed_handling() {
            UnarmedMethod::AddToCycles => {
                let h2h = hand2hand_itemdata();
                let h2h = *h2h;
                self.cycles.include_item(CycleSlot::Left, &h2h);
                self.cycles.include_item(CycleSlot::Right, &h2h);
            }
            _ => {
                // remove any item with h2h type from cycles
                self.cycles
                    .filter_kind(&CycleSlot::Left, ItemKind::HandToHand);
                self.cycles
                    .filter_kind(&CycleSlot::Right, ItemKind::HandToHand);
            }
        }
        self.flush_cycle_data();

        if !settings.autofade() {
            if self.cycles.hud_visible() {
                startAlphaTransition(true, 1.0);
            } else {
                startAlphaTransition(false, 0.0);
            }
        }
    }

    /// The player's inventory changed! Act on it if we need to.
    pub fn handle_inventory_changed(
        &mut self,
        #[allow(clippy::boxed_local)] item: Box<ItemData>, // boxed because arriving from C++
        delta: i32,
    ) {
        log::debug!("inventory count changed; {}; count={delta}", item);

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

        if item.kind().is_ammo() {
            if let Some(candidate) = self.visible.get_mut(&HudElement::Ammo) {
                if *candidate.form_string() == *item.form_string() {
                    candidate.set_count(new_count);
                }
            }
        }

        // This function does most of the work. 
        self.cycles.update_count(*item.clone(), new_count);
        if new_count > 0 {
            return;
        }

        if item.kind().is_utility() {
            if let Some(vis) = self.visible.get(&HudElement::Utility) {
                if vis.form_string() == item.form_string() {
                    if let Some(showme) = self.cycles.get_top(&CycleSlot::Utility) {
                        self.update_slot(HudElement::Utility, &showme);
                    }
                }
            }
        }
        if item.kind().left_hand_ok() {
            if let Some(vis) = self.visible.get(&HudElement::Left) {
                if vis.form_string() == item.form_string() {
                    if let Some(equipme) = self.cycles.get_top(&CycleSlot::Left) {
                        self.equip_item(&equipme, Action::Left);
                    }
                }
            }
        }
        if item.kind().right_hand_ok() {
            if let Some(vis) = self.visible.get(&HudElement::Right) {
                if vis.form_string() == item.form_string() {
                    if let Some(equipme) = self.cycles.get_top(&CycleSlot::Right) {
                        self.equip_item(&equipme, Action::Right);
                        // this might race with the left hand. IDEK.
                    }
                }
            }
        }
    }

    /// Handle a key-press event that the event system decided we need to know about.
    ///
    /// Returns an enum indicating what we did in response, so that the C++ layer can
    /// start a tick timer for cycle delay.
    pub fn handle_key_event(&mut self, key: u32, button: &ButtonEvent) -> KeyEventResponse {
        let hotkey = HotkeyKind::from(key);
        let state = KeyState::from(button);
        if matches!(hotkey, HotkeyKind::None) {
            return KeyEventResponse::default();
        }

        // log::trace!("incoming key={}; state={};", hotkey, state);

        // We want all updates so we can track long presses.
        self.update_tracked_key(&hotkey, button);

        // For mod keys, we're done.
        if hotkey.is_modifier_key() {
            return KeyEventResponse::default();
        }

        // From here on, we only care if the key has gone up.
        if state != KeyState::Up {
            return KeyEventResponse {
                handled: true,
                ..Default::default()
            };
        }

        match hotkey {
            HotkeyKind::Power => self.handle_cycle_power(),
            HotkeyKind::Utility => self.handle_cycle_utility(),
            HotkeyKind::Left => self.handle_cycle_hand(CycleSlot::Left, &hotkey),
            HotkeyKind::Right => self.handle_cycle_hand(CycleSlot::Right, &hotkey),
            HotkeyKind::Activate => self.use_utility_item(),
            HotkeyKind::Refresh => {
                HudLayout::refresh();
                KeyEventResponse {
                    handled: true,
                    ..Default::default()
                }
            }
            HotkeyKind::ShowHide => {
                if !user_settings().autofade() {
                    self.cycles.toggle_hud();
                }
                KeyEventResponse {
                    handled: true,
                    ..Default::default()
                }
            }
            _ => KeyEventResponse::default(),
        }
    }

    // Just implementing these without worrying about generalizations yet.
    fn handle_cycle_power(&mut self) -> KeyEventResponse {
        let settings = user_settings();
        let cycle_method = settings.how_to_cycle();
        match cycle_method {
            ActivationMethod::Hotkey => self.advance_simple_cycle(&CycleSlot::Power),
            ActivationMethod::LongPress => {
                let hotkey = self.get_tracked_key(&HotkeyKind::Power);
                if hotkey.is_long_press() {
                    self.advance_simple_cycle(&CycleSlot::Power)
                } else {
                    log::info!("declining to advance power/shouts cycle without a long press");
                    KeyEventResponse::default()
                }
            }
            ActivationMethod::Modifier => {
                let hotkey = self.get_tracked_key(&HotkeyKind::CycleModifier);
                if hotkey.is_pressed() {
                    self.advance_simple_cycle(&CycleSlot::Power)
                } else {
                    log::info!(
                        "declining to advance power/shouts cycle without the cycle modifier key down"
                    );
                    KeyEventResponse::default()
                }
            }
        }
    }

    fn handle_cycle_utility(&mut self) -> KeyEventResponse {
        let settings = user_settings();
        let cycle_method = settings.how_to_cycle();
        let activation_method = settings.how_to_activate();

        match cycle_method {
            ActivationMethod::Hotkey => {
                log::debug!("cycling utilities/consumables");
                return self.advance_simple_cycle(&CycleSlot::Utility);
            }
            ActivationMethod::LongPress => {
                let hotkey = self.get_tracked_key(&HotkeyKind::Utility);
                if hotkey.is_long_press() {
                    return self.advance_simple_cycle(&CycleSlot::Utility);
                }
            }
            ActivationMethod::Modifier => {
                let hotkey = self.get_tracked_key(&HotkeyKind::CycleModifier);
                if hotkey.is_pressed() {
                    log::debug!("cycling utilities/consumables");
                    return self.advance_simple_cycle(&CycleSlot::Utility);
                }
            }
        }

        match activation_method {
            ActivationMethod::Hotkey => {
                // should be unreachable-- this is its own key handler
                return KeyEventResponse::default();
            }
            ActivationMethod::LongPress => {
                let hotkey = self.get_tracked_key(&HotkeyKind::Utility);
                if hotkey.is_long_press() {
                    return self.use_utility_item();
                }
            }
            ActivationMethod::Modifier => {
                let hotkey = self.get_tracked_key(&HotkeyKind::ActivateModifier);
                if hotkey.is_pressed() {
                    log::debug!("activating utilities/consumables");
                    return self.use_utility_item();
                }
            }
        }

        KeyEventResponse::default()
    }

    fn advance_simple_cycle(&mut self, which: &CycleSlot) -> KeyEventResponse {
        // Programmer error to call this for left/right.
        if !matches!(which, CycleSlot::Power | CycleSlot::Utility) {
            log::info!("Programmer error! This is not a simple cycle. cycle={which:?}",);
            return KeyEventResponse::default();
        }

        let hud = HudElement::from(which);
        let current_not_in_cycle = if let Some(visible) = self.visible.get(&hud) {
            !self.cycles.includes(which, visible)
        } else {
            false
        };
        let candidate = if current_not_in_cycle {
            self.cycles.get_top(which)
        } else {
            self.cycles.advance(which, 1)
        };

        if let Some(next) = candidate {
            self.update_slot(hud, &next);
            KeyEventResponse {
                handled: true,
                start_timer: if !matches!(which, CycleSlot::Utility) {
                    Action::from(which.clone())
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

    fn handle_cycle_hand(&mut self, cycleslot: CycleSlot, hotkey: &HotkeyKind) -> KeyEventResponse {
        if !matches!(cycleslot, CycleSlot::Left | CycleSlot::Right) {
            return KeyEventResponse::default();
        }

        log::debug!("user requested to advance the {} hand", cycleslot);

        // We have two states to check
        let settings = user_settings();
        let tracked = self.get_tracked_key(hotkey);

        let unequip_requested = match settings.unarmed_handling() {
            UnarmedMethod::None => false,
            UnarmedMethod::LongPress => tracked.is_long_press(),
            UnarmedMethod::Modifier => {
                let unequipmod = self.get_tracked_key(&HotkeyKind::UnequipModifier);
                unequipmod.is_pressed()
            }
            UnarmedMethod::AddToCycles => false,
        };

        let cycle_requested = !unequip_requested
            && match settings.how_to_cycle() {
                ActivationMethod::Hotkey => true,
                ActivationMethod::LongPress => tracked.is_long_press(),
                ActivationMethod::Modifier => {
                    let cyclemod = self.get_tracked_key(&HotkeyKind::CycleModifier);
                    cyclemod.is_pressed()
                }
            };

        // ETOOMANYENUMS
        // The root problem is that shared enums are not sum types, and I want sum types.
        let action = Action::from(hotkey);

        if unequip_requested {
            log::info!("unequipping slot {cycleslot} by request");
            let empty_item = *hand2hand_itemdata(); // unbox
            unequipSlot(action);
            self.update_slot(HudElement::from(&cycleslot), &empty_item);
            self.cycles.set_top(&cycleslot, &empty_item);
            KeyEventResponse {
                handled: true,
                start_timer: Action::None,
                stop_timer: action,
            }
        } else if cycle_requested {
            self.advance_hand_cycle(&cycleslot)
        } else {
            // TODO honk
            log::info!("you need a modifier key down for {cycleslot}");
            KeyEventResponse::default()
        }
    }

    fn advance_hand_cycle(&mut self, which: &CycleSlot) -> KeyEventResponse {
        // This is one of two tricky decision points in the mod. (The other
        // is when timers expire and we have to act on decisions made here.)
        // We have decided we want to advance the left or right cycle.
        // What is an allowed next choice in the spot? This code deliberately
        // repeats itself to make the logic clear.

        let other_hud = if matches!(which, CycleSlot::Left) {
            HudElement::Right
        } else {
            HudElement::Left
        };

        if self.two_hander_equipped {
            // Here either hand may cycle, and the other hand must bounce back
            // to what was previously equipped. We update both slots in the HUD.

            // this should not be None given the first check, but we need to check anyway
            let Some(candidate) = self.cycles.peek_next(which) else {
                return KeyEventResponse {
                    handled: true,
                    ..Default::default()
                };
            };

            if candidate.two_handed() {
                // no problem. just cycle to it.
                self.cycles.advance(which, 1);
                return self.update_and_record(which, &candidate);
            }

            // Now we got fun. Do we have something to bounce back to in the other hand?
            let (other_cached, other_hand) = if matches!(which, CycleSlot::Left) {
                (self.right_hand_cached.clone(), CycleSlot::Right)
            } else {
                (self.left_hand_cached.clone(), CycleSlot::Left)
            };

            let Some(return_to) = other_cached else {
                // The other hand has no opinions. Advance without fear.
                self.cycles.advance(which, 1);
                return self.update_and_record(which, &candidate);
            };

            // What do we want to return to? If it's completely different from us,
            // we are golden. We update both HUD slots and start a timer.
            if candidate.form_string() != return_to.form_string() {
                self.cycles.advance(which, 1);
                let _changed = self.update_slot(other_hud, &return_to.clone());
                self.cycles.set_top(&other_hand, &return_to.clone());
                return self.update_and_record(which, &candidate);
            }

            // They are the same. Do we have more than one? If so, we're good.
            if !candidate.kind().count_matters() || candidate.count() > 1 {
                self.cycles.advance(which, 1);
                let _changed = &self.update_slot(other_hud, &return_to.clone());
                self.cycles.set_top(&other_hand, &return_to.clone());
                return self.update_and_record(which, &candidate);
            }

            // The worst case! Somebody's got to lose the battle for the single item,
            // and in this case it's the hand trying to cycle forward.
            let Some(candidate) = self.cycles.advance_skipping(which, return_to.clone()) else {
                // We have no good options. TODO honk
                return KeyEventResponse {
                    handled: true,
                    ..Default::default()
                };
            };

            if candidate.two_handed() {
                // How lucky we are. We equip it and don't fuss.
                return self.update_and_record(which, &candidate);
            } else {
                let _changed = &self.update_slot(other_hud, &return_to.clone());
                self.cycles.set_top(&other_hand, &return_to);
                return self.update_and_record(which, &candidate);
            }
        } else {
            // Phew. Okay. Now we're on to the one-handers equipped cases. These are easier.
            let maybe_candidate = if let Some(other_equipped) = self.visible.get(&other_hud) {
                // Are we dual-wielding? If so, do we have at least two?
                if !other_equipped.kind().count_matters() || other_equipped.count() > 1 {
                    self.cycles.advance(which, 1)
                } else {
                    self.cycles.advance_skipping(which, other_equipped.clone())
                }
            } else {
                self.cycles.advance(which, 1)
            };

            if let Some(candidate) = maybe_candidate {
                return self.update_and_record(which, &candidate);
            }
        }

        // If we got here, we got nothin'.
        KeyEventResponse {
            handled: true,
            ..Default::default()
        }
    }

    fn update_and_record(&mut self, which: &CycleSlot, next: &ItemData) -> KeyEventResponse {
        let hud = HudElement::from(which);
        self.update_slot(hud, next);
        // self.flush_cycle_data();

        if user_settings().autofade() {
            show_briefly();
        }

        KeyEventResponse {
            handled: true,
            start_timer: if !matches!(which, CycleSlot::Utility) {
                Action::from(which.clone())
            } else {
                Action::None
            },
            stop_timer: Action::None,
        }
    }

    /// Activate whatever we have in the utility slot.
    fn use_utility_item(&mut self) -> KeyEventResponse {
        log::debug!("using utility item");

        if let Some(item) = self.cycles.get_top(&CycleSlot::Utility) {
            if item.kind().is_potion()
                || matches!(item.kind(), ItemKind::PoisonDefault | ItemKind::Food)
            {
                cxx::let_cxx_string!(form_spec = item.form_string());
                consumePotion(&form_spec);
            } else if item.kind().is_armor() {
                cxx::let_cxx_string!(form_spec = item.form_string());
                equipArmor(&form_spec);
            } else if item.kind().is_ammo() {
                cxx::let_cxx_string!(form_spec = item.form_string());
                equipAmmo(&form_spec)
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
    pub fn timer_expired(&mut self, which: Action) {
        let hud = HudElement::from(which);
        let hotkey = self.get_tracked_key(&HotkeyKind::from(&which));
        if hotkey.is_pressed() {
            // Here's the reasoning. The player might be mid-long-press, in
            // which case we do not want to interrupt by equipping. The player
            // might be mid-short-tap, in which case the timer will get started
            // again on key up.
            return;
        }

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
        if matches!(kind, ItemKind::HandToHand) {
            log::info!("melee time! unequipping slot {which:?}");
            if which == Action::Left {
                self.left_hand_cached = Some(*hand2hand_itemdata());
            } else {
                self.right_hand_cached = Some(*hand2hand_itemdata());
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

        if !item.two_handed() {
            if which == Action::Left {
                self.left_hand_cached = Some(*item).cloned();
            } else {
                self.right_hand_cached = Some(*item).cloned();
            }
        }
        self.equip_item(item, which);
    }

    /// Convenience function for equipping any equippable.
    fn equip_item(&self, item: &ItemData, which: Action) {
        if !matches!(which, Action::Right | Action::Left | Action::Utility) {
            return;
        }

        let kind = item.kind();
        cxx::let_cxx_string!(form_spec = item.form_string());
        log::trace!("about to equip this item: slot={:?}; {}", which, item);

        if kind.is_magic() || kind.left_hand_ok() || kind.right_hand_ok() {
            equipWeapon(&form_spec, which);
        } else if kind.is_armor() {
            equipArmor(&form_spec);
        } else if kind == ItemKind::Arrow {
            equipAmmo(&form_spec);
        } else {
            log::info!("we did nothing with item {}", item);
        }
    }

    /// The game informs us that our equipment has changed. Update.
    ///
    /// The item we're handed was either equipped or UNequipped. There are some
    /// changes we do need to react to, either because they were done
    /// out-of-band of the HUD or because we want to do more work in reaction to
    /// changes we initiated.
    pub fn handle_item_equipped(
        &mut self,
        equipped: bool,
        #[allow(clippy::boxed_local)] item: Box<ItemData>, // boxed because arriving from C++
        right: bool,
        left: bool,
    ) -> bool {
        if !equipped {
            return false;
        }

        let item = *item; // insert unboxing video

        if item.kind().is_ammo() {
            if let Some(visible) = self.visible.get(&HudElement::Ammo) {
                if visible.form_string() != item.form_string() {
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

        if item.kind().is_utility() {
            // We do nothing. We are the source of truth for non-ammo on the utility view.
            return false;
        }

        if item.kind().is_power() {
            if let Some(visible) = self.visible.get(&HudElement::Power) {
                if visible.form_string() != item.form_string() {
                    self.update_slot(HudElement::Power, &item);
                    self.cycles.set_top(&CycleSlot::Power, &item);
                    return true;
                } else {
                    return false;
                }
            } else {
                self.update_slot(HudElement::Power, &item);
                return true;
            }
        }

        if !left && !right {
            return false;
        }

        // ----------
        // The hard part starts. Left hand vs right hand. Earlier, we did our
        // best to set up the HUD to show what we want in each hand. So we look
        // at the item equipped: does it match an earlier decision? If so, make
        // the other decision happen as well. If the equip event was NOT driven
        // by the HUD, we have some more work to do.

        log::trace!("is-now-equipped={}; allegedly-right={}; allegedly-left: {}; name='{}'; item.kind={:?}; item 2-handed={}; 2-hander equipped={}; left_cached='{}'; right_cached='{}';",
            equipped,
            right,
            left,
            item.name(),
            item.kind(),
            item.two_handed(),
            self.two_hander_equipped,
            self.left_hand_cached.clone().map_or("".to_string(), |xs| xs.name()),
            self.right_hand_cached.clone().map_or("".to_string(), |xs| xs.name())
        );

        if item.two_handed() {
            let changed = self.update_slot(HudElement::Right, &item);
            if changed {
                // Change was out of band. We need to react.
                self.cycles.set_top(&CycleSlot::Right, &item);
            }
            self.update_slot(HudElement::Left, &ItemData::default());
            self.two_hander_equipped = true;
            log::info!("finished handling two-hander being equipped. all good; changed={changed}");
            return changed;
        }

        // It's a one-hander. Does it match an earlier decision?

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
            "form strings: item={}; equipped-right={}; requipped-left={}; two-hander-equipped={}; two-handed={}; name='{}';",
            item.form_string(),
            rightie.form_string(),
            leftie.form_string(),
            self.two_hander_equipped,
            item.two_handed(),
            item.name(),
        );
        let leftvis = self
            .visible
            .get(&HudElement::Left)
            .map_or("".to_string(), |xs| xs.form_string());
        let rightvis = self
            .visible
            .get(&HudElement::Right)
            .map_or("".to_string(), |xs| xs.form_string());

        let left_unexpected = leftvis != leftie.form_string();
        let right_unexpected = rightvis != rightie.form_string();

        if right {
            // update was out of band
            if right_unexpected {
                self.update_slot(HudElement::Right, &item);
            } else {
                // we expected the right hand change. Force the left hand to show what we wanted.
                cxx::let_cxx_string!(form_spec = leftvis);
                reequipHand(Action::Left, &form_spec);
            }
        }

        if left {
            // update was out of band
            if left_unexpected {
                self.update_slot(HudElement::Left, &item);
            } else {
                // we expected the left hand change. Force the right hand to show what we wanted.
                cxx::let_cxx_string!(form_spec = rightvis);
                reequipHand(Action::Right, &form_spec);
            }
        }

        if !self.two_hander_equipped {
            // We are done. Phew.
            return left_unexpected || right_unexpected;
        }

        // We just swapped from a two-hander to a one-hander.
        // Now we earn our keep.

        if right {
            if let Some(prev_left) = self.left_hand_cached.clone() {
                log::debug!("considering re-requipping or unequipping LEFT");
                if prev_left.kind() == ItemKind::HandToHand {
                    unequipSlot(Action::Left);
                } else {
                    cxx::let_cxx_string!(form_spec = prev_left.form_string());
                    reequipHand(Action::Left, &form_spec);
                }
                self.update_slot(HudElement::Left, &prev_left);
            }
        }

        if left {
            if let Some(prev_right) = self.right_hand_cached.clone() {
                log::debug!("considering re-requipping or unequipping RIGHT");
                if prev_right.kind() == ItemKind::HandToHand {
                    unequipSlot(Action::Right);
                } else {
                    cxx::let_cxx_string!(form_spec = prev_right.form_string());
                    reequipHand(Action::Right, &form_spec);
                }
                self.update_slot(HudElement::Right, &prev_right);
            }
        }

        self.two_hander_equipped = item.two_handed();
        left_unexpected || right_unexpected
    }

    /// Get the item equipped in a specific slot.
    /// Called by the HUD rendering loop in the ImGui code.
    pub fn entry_to_show_in_slot(&self, slot: HudElement) -> Box<ItemData> {
        let Some(candidate) = self.visible.get(&slot) else {
            return Box::<ItemData>::default();
        };

        Box::new(candidate.clone())
    }

    /// Call when loading or otherwise needing to reinitialize the HUD.
    ///
    /// Updates will only happen here if the player changed equipment
    /// out of band, e.g., by using a menu, and only then if we screwed
    /// up an equip event.
    pub fn update_hud(&mut self) -> bool {
        let right_entry = boundObjectRightHand();
        let right_changed = self.update_slot(HudElement::Right, &right_entry);
        if !right_entry.two_handed() {
            self.right_hand_cached = Some(*right_entry.clone());
        }

        let left_entry = boundObjectLeftHand();
        let left_unexpected = self.update_slot(HudElement::Left, &left_entry);
        if !left_entry.two_handed() {
            self.left_hand_cached = Some(*left_entry.clone());
        }
        self.two_hander_equipped = right_entry.two_handed(); // same item will be in both hands

        let power = equippedPower();
        let power_changed = self.update_slot(HudElement::Power, &power);

        let ammo = equippedAmmo();
        let ammo_changed = self.update_slot(HudElement::Ammo, &ammo);

        let utility_changed = if let Some(utility) = self.cycles.get_top(&CycleSlot::Utility) {
            self.update_slot(HudElement::Utility, &utility);
            true
        } else {
            false
        };

        let changed =
            right_changed || left_unexpected || power_changed || ammo_changed || utility_changed;

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
            self.cycles.set_top(&CycleSlot::Power, &power);
            self.cycles.set_top(&CycleSlot::Left, &left_entry);
            self.cycles.set_top(&CycleSlot::Right, &right_entry);
        }

        changed
    }

    /// Update the displayed slot for the specified HUD element.
    fn update_slot(&mut self, slot: HudElement, new_item: &ItemData) -> bool {
        if let Some(replaced) = self.visible.insert(slot, new_item.clone()) {
            replaced != *new_item
        } else {
            false
        }
    }

    pub fn handle_favorite_event(
        &mut self,
        _button: &ButtonEvent,
        is_favorite: bool,
        item: ItemData,
    ) {
        let settings = user_settings();
        if !settings.link_to_favorites() {
            return;
        }

        log::info!("handle_favorite_event(); is_favorite={is_favorite};");
        log::info!("    {item}; two-handed={};", item.two_handed());

        let maybe_message = if !is_favorite {
            // This formerly-favorite item is now disliked.
            if item.kind().is_utility() {
                if self.cycles.remove_item(CycleSlot::Utility, &item) {
                    Some(format!("{} removed from utilities cycle.", item.name()))
                } else {
                    None
                }
            } else if item.kind().is_power() {
                if self.cycles.remove_item(CycleSlot::Power, &item) {
                    Some(format!("{} removed from powers cycle.", item.name()))
                } else {
                    None
                }
            } else if item.two_handed() {
                if self.cycles.remove_item(CycleSlot::Right, &item) {
                    Some(format!("{} removed from right hand.", item.name()))
                } else {
                    None
                }
            } else {
                let removed_right = self.cycles.remove_item(CycleSlot::Right, &item);
                let removed_left = self.cycles.remove_item(CycleSlot::Left, &item);
                if removed_right && removed_left {
                    Some(format!("{} removed from both hands.", item.name()))
                } else if removed_left {
                    Some(format!("{} removed from left hand.", item.name()))
                } else if removed_right {
                    Some(format!("{} removed from right hand.", item.name()))
                } else {
                    None
                }
            }
        } else {
            // This item is a new fave! Add to whatever the appropriate cycle is.
            if item.kind().is_utility() {
                log::info!("attempting to add to utilities cycle...");
                if self.cycles.add_item(CycleSlot::Utility, &item) {
                    Some(format!("{} added to utilities cycle.", item.name()))
                } else {
                    None
                }
            } else if item.kind().is_power() {
                log::info!("attempting to add to powers cycle...");
                if self.cycles.add_item(CycleSlot::Power, &item) {
                    Some(format!("{} added to powers cycle.", item.name()))
                } else {
                    None
                }
            } else if item.two_handed() || matches!(item.kind(), ItemKind::Scroll) {
                if self.cycles.add_item(CycleSlot::Right, &item) {
                    Some(format!("{} added to right hand.", item.name()))
                } else {
                    None
                }
            } else if item.kind().is_spell() || (item.kind().right_hand_ok() && item.count() > 1) {
                let added_right = self.cycles.add_item(CycleSlot::Right, &item);
                let added_left = self.cycles.add_item(CycleSlot::Left, &item);
                if added_right && added_left {
                    Some(format!("{} added to both hands.", item.name()))
                } else if added_left {
                    Some(format!("{} added to left hand.", item.name()))
                } else if added_right {
                    Some(format!("{} added to right hand.", item.name()))
                } else {
                    None
                }
            } else if item.kind().right_hand_ok() {
                if self.cycles.add_item(CycleSlot::Right, &item) {
                    Some(format!("{} added to right hand.", item.name()))
                } else {
                    None
                }
            } else if self.cycles.add_item(CycleSlot::Left, &item) {
                Some(format!("{} added to left hand.", item.name()))
            } else {
                None
            }
        };

        if let Some(msg) = maybe_message {
            log::info!("{msg}");
            cxx::let_cxx_string!(message = msg);
            notifyPlayer(&message);
            self.flush_cycle_data();
        } else {
            log::info!("no changes made.");
        }
    }

    pub fn handle_menu_event(&mut self, key: u32, button: &ButtonEvent) -> bool {
        // Much simpler than the cycle loop. We care if the cycle modifier key
        // is down (if one is set), and we care if the cycle button itself has
        // been pressed.
        let hotkey = HotkeyKind::from(key);
        if matches!(hotkey, HotkeyKind::None) {
            return false;
        }

        self.update_tracked_key(&hotkey, button);
        if !hotkey.is_cycle_key() || !button.IsUp() {
            return false;
        }

        let settings = user_settings();
        let menu_method = settings.how_to_toggle();

        match menu_method {
            ActivationMethod::Hotkey => true,
            ActivationMethod::LongPress => {
                log::debug!("checking for long press in menu");
                // if it's not found, will never be a long press
                self.get_tracked_key(&hotkey).is_long_press()
            }
            ActivationMethod::Modifier => {
                let modkey = self.get_tracked_key(&HotkeyKind::MenuModifier);
                log::debug!(
                    "checking for menu modifier key pressed in menu; {modkey:?} => {}",
                    modkey.is_pressed()
                );
                modkey.is_pressed()
            }
        }
    }

    /// This function is called when the player has pressed a hot key while hovering over an
    /// item in a menu. We'll remove the item if it's already in the matching cycle,
    /// or add it if it's an appropriate item. We signal back to the UI layer what we did.
    pub fn handle_toggle_item(&mut self, action: Action, item: ItemData) {
        let Ok(cycle_slot) = CycleSlot::try_from(action) else {
            return;
        };

        let result = self.cycles.toggle(&cycle_slot, item.clone());

        if matches!(result, MenuEventResponse::ItemRemoved) && matches!(action, Action::Utility) {
            if let Some(topmost) = self.cycles.get_top(&CycleSlot::Utility) {
                self.update_slot(HudElement::Utility, &topmost);
            } else {
                self.update_slot(HudElement::Utility, &ItemData::default());
            }
        }

        // notify the player what happened...
        let verb = match result {
            MenuEventResponse::ItemAdded => "added to",
            MenuEventResponse::ItemRemoved => "removed from",
            MenuEventResponse::ItemInappropriate => "can't go into the",
            MenuEventResponse::TooManyItems => "would overflow the",
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

    // watching the keys
    fn update_tracked_key(&mut self, hotkey: &HotkeyKind, button: &ButtonEvent) {
        if let Some(tracked) = self.tracked_keys.get_mut(hotkey) {
            tracked.update(button);
        } else {
            let mut tracked = TrackedKey {
                key: hotkey.clone(),
                state: KeyState::default(),
                press_start: None,
            };
            tracked.update(button);
            self.tracked_keys.insert(hotkey.clone(), tracked);
        }
    }

    fn get_tracked_key(&self, hotkey: &HotkeyKind) -> TrackedKey {
        if let Some(tracked) = self.tracked_keys.get(hotkey) {
            tracked.clone()
        } else {
            TrackedKey {
                key: HotkeyKind::None,
                state: KeyState::Up,
                press_start: None,
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

impl From<u32> for Action {
    /// Turn the key code into an enum for easier processing.
    fn from(value: u32) -> Self {
        let settings = user_settings();

        if value == settings.left() {
            Action::Left
        } else if value == settings.right() {
            Action::Right
        } else if value == settings.power() {
            Action::Power
        } else if value == settings.utility() {
            Action::Utility
        } else if value == settings.activate() {
            Action::Activate
        } else if value == settings.showhide() {
            Action::ShowHide
        } else if value == settings.refresh_layout() {
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
