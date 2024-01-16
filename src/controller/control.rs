//! The controller in the MVC way of thinking. The brains of the mod.
//!
//! This struct holds all runtime data for the HUD and its implementation
//! along with associated types manage what happens when the user presses keys.
//! It tracks key states, cycle contents (via CycleData), what needs to be drawn
//! in the HUD, and manages a cache for items in use by the HUD.
//! This is by far the most complex logic in the entire application.
//!
//! Functions of note: `handle_key_event()`, `handle_item_equipped()`, and
//! `timer_expired()`.
//!
//! I apologize for what a mess this is. It grew organically and the feature
//! set is itself complex.

use std::collections::HashMap;
use std::sync::Mutex;

use cxx::let_cxx_string;
use once_cell::sync::Lazy;
use strfmt::strfmt;

use super::cycles::*;
use super::keys::*;
use super::settings::{settings, ActivationMethod, UnarmedMethod};
use crate::cycleentries::*;
use crate::data::item_cache::ItemCache;
use crate::data::potion::PotionType;
use crate::data::*;
use crate::layouts::Layout;
use crate::plugin::*;

/// There can be only one. Not public because we want access managed.
static CONTROLLER: Lazy<Mutex<Controller>> = Lazy::new(|| Mutex::new(Controller::new()));

pub fn get() -> std::sync::MutexGuard<'static, Controller> {
    CONTROLLER
        .lock()
        .expect("Unrecoverable runtime problem: cannot acquire controller lock. Exiting.")
}

/// What, model/view/controller? In my UI application? oh no
#[derive(Debug)]
pub struct Controller {
    /// Our currently-active cycles.
    pub cycles: CycleData,
    /// The hud item cache.
    pub cache: ItemCache,
    /// The items the HUD should show right now.
    visible: HashMap<HudElement, HudItem>,
    /// True if we've got a two-handed weapon equipped right now.
    two_hander_equipped: bool,
    /// We cache the form spec of any left-hand item we were holding before a two-hander was equipped.
    left_hand_cached: String,
    /// We cache a right-hand form spec string similarly.
    right_hand_cached: String,
    /// We need to track keystate to implement modifier keys.
    tracked_keys: HashMap<Hotkey, TrackedKey>,
    /// True if we're using CGO's alternative grip.
    cgo_alt_grip: bool,
}

impl Controller {
    /// Make a controller with no information in it.
    pub fn new() -> Self {
        Controller {
            cycles: CycleData::default(),
            cache: ItemCache::new(),
            visible: HashMap::new(),
            two_hander_equipped: false,
            left_hand_cached: "".to_string(),
            right_hand_cached: "".to_string(),
            tracked_keys: HashMap::new(),
            cgo_alt_grip: false,
        }
    }

    /// Called after a save load to initialize state. The validate function logs out cycles.
    pub fn refresh_after_load(&mut self) {
        self.cycles.validate(&mut self.cache);
        self.update_hud();
    }

    /// Called by the MCM cycle clear button.
    pub fn clear_cycles(&mut self) {
        log::info!("Clearing all cycles. Turning off targeting computer.");
        self.cycles.clear();
    }

    /// Get the names of all items in the given cycle. Papyrus support.
    // needs to be mut because the cache might have items added to it when we fetch
    pub fn cycle_names(&mut self, which: i32) -> Vec<String> {
        match which {
            0 => self.cycles.names(&CycleSlot::Power, &mut self.cache),
            1 => self.cycles.names(&CycleSlot::Utility, &mut self.cache),
            2 => self.cycles.names(&CycleSlot::Left, &mut self.cache),
            3 => self.cycles.names(&CycleSlot::Right, &mut self.cache),
            _ => Vec::new(),
        }
    }

    /// Get form IDs for all items in the given cycle. Papyrus support.
    pub fn cycle_formids(&self, which: i32) -> Vec<String> {
        match which {
            0 => self.cycles.formids(&CycleSlot::Power),
            1 => self.cycles.formids(&CycleSlot::Utility),
            2 => self.cycles.formids(&CycleSlot::Left),
            3 => self.cycles.formids(&CycleSlot::Right),
            _ => Vec::new(),
        }
    }

    /// Called after any settings file read to enforce them.
    pub fn apply_settings(&mut self) {
        let settings = settings();

        match settings.unequip_method() {
            UnarmedMethod::AddToCycles => {
                let h2h = HudItem::make_unarmed_proxy();
                self.cache.record(h2h.clone());
                self.cycles.add_item(CycleSlot::Left, &h2h);
                self.cycles.add_item(CycleSlot::Right, &h2h);
            }
            _ => {
                // remove any item with h2h type from cycles
                self.cycles
                    .filter_kind(&CycleSlot::Left, &BaseType::HandToHand, &mut self.cache);
                self.cycles
                    .filter_kind(&CycleSlot::Right, &BaseType::HandToHand, &mut self.cache);
            }
        }

        if settings.group_potions() {
            self.cycles.filter_kind(
                &CycleSlot::Utility,
                &BaseType::Potion(PotionType::Stamina),
                &mut self.cache,
            );
            let proxy = make_stamina_proxy();
            self.cache.record(proxy.clone());
            self.cycles.add_item(CycleSlot::Utility, &proxy);

            self.cycles.filter_kind(
                &CycleSlot::Utility,
                &BaseType::Potion(PotionType::Health),
                &mut self.cache,
            );
            let proxy = make_health_proxy();
            self.cache.record(proxy.clone());
            self.cycles.add_item(CycleSlot::Utility, &proxy);

            self.cycles.filter_kind(
                &CycleSlot::Utility,
                &BaseType::Potion(PotionType::Magicka),
                &mut self.cache,
            );
            let proxy = make_magicka_proxy();
            self.cache.record(proxy.clone());
            self.cycles.add_item(CycleSlot::Utility, &proxy);
        } else {
            let proxy = make_stamina_proxy();
            self.cycles.remove_item(CycleSlot::Utility, &proxy);
            let proxy = make_health_proxy();
            self.cycles.remove_item(CycleSlot::Utility, &proxy);
            let proxy = make_magicka_proxy();
            self.cycles.remove_item(CycleSlot::Utility, &proxy);
        }

        setMaxAlpha(settings.max_alpha());

        if !settings.autofade() {
            if self.cycles.hud_visible() {
                startAlphaTransition(true, 1.0);
            } else {
                startAlphaTransition(false, 0.0);
            }
        }

        // Apply any new anchor relocations to the current layout.
        Layout::refresh();

        self.cache.introspect();
    }

    /// For all visible items, refresh data used by the renderer that might
    /// have changed in the last N draw cycles, where N is a count controlled
    /// by the renderer itself.
    pub fn refresh_hud_items(&mut self) {
        // The only relevant items are shouts, left, and right hand.
        if let Some(power) = self.visible.get_mut(&HudElement::Power) {
            power.refresh_extra_data();
        }
        if let Some(left) = self.visible.get_mut(&HudElement::Left) {
            left.refresh_extra_data();
        }
        if let Some(right) = self.visible.get_mut(&HudElement::Right) {
            right.refresh_extra_data();
        }
    }

    /// The player's inventory changed! Act on it if we need to.
    pub fn handle_inventory_changed(&mut self, form_spec: &String, new_count: u32) {
        let Some(item) = self.cache.update_count(form_spec.as_str(), new_count) else {
            return;
        };

        // If I were smart enough to have the hud use the same object that the cache
        // holds this would be a lot less work, but I am stupid.

        let kind = item.kind().clone();
        log::trace!(
            "inventory count update: name='{}'; count={new_count}",
            item.name()
        );

        if kind.is_ammo() {
            if let Some(candidate) = self.visible.get_mut(&HudElement::Ammo) {
                if candidate.form_string() == *form_spec {
                    candidate.set_count(new_count);
                }
            }
        } else if kind.is_utility() {
            // update count of magicka, health, or stamina potions if we're grouped
            if kind.is_potion() && settings().group_potions() {
                if matches!(kind, BaseType::Potion(PotionType::Health)) {
                    self.cache.set_count("health_proxy", healthPotionCount());
                }
                if matches!(kind, BaseType::Potion(PotionType::Magicka)) {
                    self.cache.set_count("magicka_proxy", magickaPotionCount());
                }
                if matches!(kind, BaseType::Potion(PotionType::Stamina)) {
                    self.cache.set_count("stamina_proxy", staminaPotionCount());
                }
            }

            if let Some(candidate) = self.visible.get_mut(&HudElement::Utility) {
                let visible_spec = candidate.form_string();
                if visible_spec == *form_spec {
                    candidate.set_count(new_count);
                } else if visible_spec == "health_proxy" {
                    candidate.set_count(healthPotionCount());
                } else if visible_spec == "magicka_proxy" {
                    candidate.set_count(magickaPotionCount());
                } else if visible_spec == "stamina_proxy" {
                    candidate.set_count(staminaPotionCount());
                }
            }
        } else {
            // This entire code block is unlikely to execute because we are
            // consistently getting the unequip message first. Unfortunately
            // we have no idea at that time *why* the unequip event happened.
            if let Some(candidate) = self.visible.get_mut(&HudElement::Left) {
                if candidate.form_string() == *form_spec {
                    candidate.set_count(new_count);
                    if new_count == 0 {
                        self.advance_hand_cycle(&CycleSlot::Left);
                    }
                }
            }
            if let Some(candidate) = self.visible.get_mut(&HudElement::Right) {
                if candidate.form_string() == *form_spec {
                    candidate.set_count(new_count);
                    if new_count == 0 {
                        self.advance_hand_cycle(&CycleSlot::Right);
                    }
                }
            }
        }
        if new_count > 0 {
            return;
        }

        self.cycles
            .remove_zero_count_items(form_spec.as_str(), &kind);

        // The count of the inventory item went to zero. We need to check
        // if we must equip/ready something else now.

        if kind.is_utility() {
            if let Some(vis) = self.visible.get(&HudElement::Utility) {
                if vis.form_string() == *form_spec {
                    if let Some(formspec) = self.cycles.get_top(&CycleSlot::Utility) {
                        let item = self.cache.get_with_refresh(&formspec);
                        self.update_slot(HudElement::Utility, &item);
                    }
                }
            }
        }
        if kind.left_hand_ok() {
            if let Some(vis) = self.visible.get(&HudElement::Left) {
                if vis.form_string() == *form_spec {
                    if let Some(formspec) = self.cycles.get_top(&CycleSlot::Left) {
                        let item = self.cache.get(&formspec);
                        self.equip_item(&item, Action::Left);
                    }
                }
            }
        }
        if kind.right_hand_ok() {
            if let Some(vis) = self.visible.get(&HudElement::Right) {
                if vis.form_string() == *form_spec {
                    if let Some(formspec) = self.cycles.get_top(&CycleSlot::Right) {
                        let item = self.cache.get(&formspec);
                        self.equip_item(&item, Action::Right);
                        // this might race with the left hand. IDEK.
                    }
                }
            }
        }
    }

    /// Handle a gameplay key-press event that the event system decided we need to know about.
    ///
    /// Returns an enum indicating what we did in response, so that the C++ layer can
    /// start a tick timer for cycle delay.
    pub fn handle_key_event(&mut self, key: u32, button: &ButtonEvent) -> KeyEventResponse {
        let hotkey = Hotkey::from(key);
        let state = KeyState::from(button);
        // log::info!(
        //     "{key} {} {hotkey}  {state}",
        //     char::from_u32(key).unwrap_or('X')
        // );
        if matches!(hotkey, Hotkey::None) {
            return KeyEventResponse::default();
        }

        // log::trace!("incoming key={}; state={};", hotkey, state);

        // We want all updates so we can track mod keys & long presses.
        // This call starts and stops long-press timers as well.
        let keep_handling = self.update_tracked_key(&hotkey, button, false);

        // For mod keys, we're done.
        if hotkey.is_modifier_key() || !keep_handling {
            return KeyEventResponse::default();
        }

        // From here on, we only care if the key has gone up.
        if state != KeyState::Up {
            return KeyEventResponse::handled();
        }

        let le_options = settings();
        showBriefly();

        match hotkey {
            Hotkey::Power => self.handle_cycle_power(),
            Hotkey::Utility => self.handle_cycle_utility(),
            Hotkey::Left => self.handle_cycle_left(&hotkey),
            Hotkey::Right => self.handle_cycle_right(&hotkey),
            Hotkey::Equipment => self.handle_cycle_equipset(&hotkey),
            Hotkey::Activate => {
                let activation_method = le_options.utility_activation_method();
                if matches!(activation_method, ActivationMethod::Hotkey) {
                    self.use_utility_item()
                } else {
                    KeyEventResponse::default()
                }
            }
            Hotkey::UnequipHands => {
                let unarmed_method = le_options.unequip_method();
                if matches!(unarmed_method, UnarmedMethod::Hotkey) {
                    self.disarm_player()
                } else {
                    KeyEventResponse::default()
                }
            }
            Hotkey::Refresh => {
                Layout::refresh();
                KeyEventResponse::handled()
            }
            Hotkey::ShowHide => {
                if !le_options.autofade() {
                    self.cycles.toggle_hud();
                }
                KeyEventResponse::handled()
            }
            _ => KeyEventResponse::default(),
        }
    }

    /// Handle the power/shouts key being pressed.
    fn handle_cycle_power(&mut self) -> KeyEventResponse {
        // We don't need to worry about long presses here: those are handled by timers.
        let settings = settings();
        let cycle_method = settings.cycle_advance_method();
        if matches!(cycle_method, ActivationMethod::Hotkey) {
            self.advance_cycle_power()
        } else if matches!(cycle_method, ActivationMethod::Modifier) {
            let hotkey = self.get_tracked_key(&Hotkey::CycleModifier);
            if hotkey.is_pressed() {
                self.advance_cycle_power()
            } else {
                KeyEventResponse::default()
            }
        } else {
            KeyEventResponse::default()
        }
    }

    /// The power/shouts keypress resulted in advancing the cycle.
    fn advance_cycle_power(&mut self) -> KeyEventResponse {
        let current_not_in_cycle = if let Some(visible) = self.visible.get(&HudElement::Power) {
            !self.cycles.includes(&CycleSlot::Power, visible)
        } else {
            false
        };
        let candidate = if current_not_in_cycle {
            self.cycles.get_top(&CycleSlot::Power)
        } else {
            self.cycles.advance(&CycleSlot::Power, 1)
        };

        if let Some(next) = candidate {
            let item = self.cache.get_with_refresh(&next);
            self.update_slot(HudElement::Power, &item);
            KeyEventResponse {
                handled: true,
                start_timer: Action::from(CycleSlot::Power),
                stop_timer: Action::None,
            }
        } else {
            KeyEventResponse::handled()
        }
    }

    /// Hande the utilities/consumable key being pressed.
    fn handle_cycle_utility(&mut self) -> KeyEventResponse {
        // Same comment about long presses.
        let settings = settings();

        if matches!(
            settings.utility_activation_method(),
            ActivationMethod::Modifier
        ) {
            let modifier = self.get_tracked_key(&Hotkey::ActivateModifier);
            if modifier.is_pressed() {
                log::debug!("activating utilities/consumables");
                return self.use_utility_item();
            }
        }

        let cycle_method = settings.cycle_advance_method();
        if matches!(cycle_method, ActivationMethod::Hotkey) {
            log::debug!("cycling utilities/consumables");
            return self.advance_cycle_utilities();
        }
        if matches!(cycle_method, ActivationMethod::Modifier) {
            let modifier = self.get_tracked_key(&Hotkey::CycleModifier);
            if modifier.is_pressed() {
                log::debug!("cycling utilities/consumables");
                return self.advance_cycle_utilities();
            }
        }

        KeyEventResponse::default()
    }

    /// Advance the utilities/consumables cycle.
    fn advance_cycle_utilities(&mut self) -> KeyEventResponse {
        let current_not_in_cycle = if let Some(visible) = self.visible.get(&HudElement::Utility) {
            !self.cycles.includes(&CycleSlot::Utility, visible)
        } else {
            false
        };
        let candidate = if current_not_in_cycle {
            self.cycles.get_top(&CycleSlot::Utility)
        } else {
            self.cycles.advance(&CycleSlot::Utility, 1)
        };

        if let Some(next) = candidate {
            let item = self.cache.get_with_refresh(&next);
            self.update_slot(HudElement::Utility, &item);
            KeyEventResponse {
                handled: true,
                start_timer: Action::None,
                stop_timer: Action::None,
            }
        } else {
            KeyEventResponse::handled()
        }
    }

    /// Figure out what is supposed to happen on a key up, given settings and keystate.
    /// Only used for right and left hand.
    fn requested_keyup_action(&self, hotkey: &Hotkey) -> RequestedAction {
        let settings = settings();
        let tracked = self.get_tracked_key(hotkey);
        let is_long_press = tracked.is_long_press();

        let unequip_requested = match settings.unequip_method() {
            UnarmedMethod::None => false,
            UnarmedMethod::LongPress => tracked.is_long_press(),
            UnarmedMethod::Modifier => {
                let unequipmod = self.get_tracked_key(&Hotkey::UnequipModifier);
                unequipmod.is_pressed()
            }
            UnarmedMethod::AddToCycles => false,
            UnarmedMethod::Hotkey => false, // this hotkey has its own handler
        };

        if unequip_requested {
            RequestedAction::Unequip
        } else if is_long_press && settings.long_press_to_dual_wield() {
            RequestedAction::Match
        } else if match settings.cycle_advance_method() {
            ActivationMethod::Hotkey => true,
            ActivationMethod::LongPress => is_long_press,
            ActivationMethod::Modifier => {
                let cyclemod = self.get_tracked_key(&Hotkey::CycleModifier);
                cyclemod.is_pressed()
            }
        } {
            RequestedAction::Advance
        } else {
            RequestedAction::None
        }
    }

    /// Handle the right hand hotkey.
    fn handle_cycle_right(&mut self, hotkey: &Hotkey) -> KeyEventResponse {
        let requested_action = self.requested_keyup_action(hotkey);
        // The left hand needs these two steps separated. See next function.
        self.do_hand_action(requested_action, Action::Right, CycleSlot::Right)
    }

    /// Hande the left hand hotkey.
    fn handle_cycle_left(&mut self, hotkey: &Hotkey) -> KeyEventResponse {
        let settings = settings();
        let cycle_ammo = settings.cycle_ammo();
        let requested_action = self.requested_keyup_action(hotkey);

        // Here's our different left-hand decision.
        // Do we have a bow equipped, and if so, is the "handle ammo" boolean set?
        // In that case, we switch ammo. Ammo is ordered least damaging -> most damaging.
        if matches!(requested_action, RequestedAction::Advance) && hasRangedEquipped() && cycle_ammo
        {
            self.advance_ammo()
        } else {
            self.do_hand_action(requested_action, Action::Left, CycleSlot::Left)
        }
    }

    /// Do what the user requested for the given hand.
    fn do_hand_action(
        &mut self,
        requested: RequestedAction,
        hand: Action,
        slot: CycleSlot,
    ) -> KeyEventResponse {
        match requested {
            RequestedAction::Unequip => {
                log::info!("unequipping {hand:?} hand by request");
                let unarmed = HudItem::make_unarmed_proxy();
                unequipSlot(hand);
                self.update_slot(HudElement::from(&slot), &unarmed);
                self.cycles.set_top(&slot, &unarmed.form_string());
                KeyEventResponse {
                    handled: true,
                    start_timer: Action::None,
                    stop_timer: hand,
                }
            }
            RequestedAction::Advance => self.advance_hand_cycle(&slot),
            RequestedAction::AdvanceAmmo => self.advance_ammo(), // pretty sure we never hit this
            RequestedAction::Match => self.match_hands(hand),
            RequestedAction::Consume => KeyEventResponse::default(),
            RequestedAction::None => KeyEventResponse::default(),
        }
    }

    fn disarm_player(&mut self) -> KeyEventResponse {
        self.do_hand_action(RequestedAction::Unequip, Action::Left, CycleSlot::Left);
        self.do_hand_action(RequestedAction::Unequip, Action::Right, CycleSlot::Right)
    }

    /// Match hands to each other; that is, dual-wield whatever is in the hand we
    /// were asked to match, if possible.
    fn match_hands(&mut self, action: Action) -> KeyEventResponse {
        let (equipped, other_hand) = if matches!(action, Action::Left) {
            (specEquippedLeft(), Action::Right)
        } else {
            (specEquippedRight(), Action::Left)
        };
        let item = self.cache.get(&equipped);

        if item.left_hand_ok() && item.right_hand_ok() {
            log::info!("Attempting to dual-wield '{}' by request.", item.name());
            if item.form_string().as_str() == "unarmed_proxy" {
                unequipSlot(other_hand);
                self.update_slot(HudElement::from(other_hand), &item);
            } else {
                self.equip_item(&item, other_hand);
            }
            KeyEventResponse {
                handled: true,
                start_timer: Action::None,
                stop_timer: action,
            }
        } else {
            log::info!("Can't dual-wield '{}' item!", item.name());
            KeyEventResponse {
                handled: true,
                start_timer: Action::None,
                stop_timer: action,
            }
        }
    }

    /// Advance the left or right hand cycle.
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
            let Some(form_string) = self.cycles.peek_next(which) else {
                return KeyEventResponse::handled();
            };

            let candidate = self.cache.get(&form_string);
            if self.treat_as_two_handed(&candidate) {
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

            if other_cached.is_empty() {
                // The other hand has no opinions. Advance without fear.
                self.cycles.advance(which, 1);
                return self.update_and_record(which, &candidate);
            };
            let return_to = self.cache.get(&other_cached);

            // What do we want to return to? If it's completely different from us,
            // we are golden. We update both HUD slots and start a timer.
            if candidate.form_string() != return_to.form_string() {
                self.cycles.advance(which, 1);

                // are we bouncing back to something in a cycle or not? This is fun.
                if self.cycles.includes(&other_hand, &return_to) {
                    let _changed = self.update_slot(other_hud, &return_to);
                    self.cycles.set_top(&other_hand, &return_to.form_string());
                } else {
                    // The return to item was removed from the cycle at some point. This is
                    // a question of design now. We can either select the next *single-handed*
                    // item in the cycle or bounce back anyway.
                    // We choose to bounce back anyway.
                    let _changed = self.update_slot(other_hud, &return_to);
                    /* if we chose to advance, it would look like this.
                    log::debug!("return-to item is no longer in the other hand's cycle.");
                    if matches!(other_hand, CycleSlot::Right) {
                        if let Some(advance_to) = self.cycles.advance_skipping_twohanders() {
                            let _changed = self.update_slot(other_hud, &advance_to.clone());
                            self.cycles.set_top(&other_hand, &advance_to);
                            self.right_hand_cached = Some(advance_to);
                        } else {
                            self.right_hand_cached = Some(*hand2hand_HudItem());
                        }
                    }
                    */
                }

                return self.update_and_record(which, &candidate);
            }

            // They are the same. Do we have more than one? If so, we're good.
            if !candidate.count_matters() || candidate.count() > 1 {
                self.cycles.advance(which, 1);
                let _changed = &self.update_slot(other_hud, &return_to.clone());
                self.cycles.set_top(&other_hand, &return_to.form_string());
                return self.update_and_record(which, &candidate);
            }

            // The worst case! Somebody's got to lose the battle for the single item,
            // and in this case it's the hand trying to cycle forward.
            let Some(form_string) = self.cycles.advance_skipping(which, return_to.clone()) else {
                honk();
                return KeyEventResponse::handled();
            };

            let candidate = self.cache.get(&form_string);
            if self.treat_as_two_handed(&candidate) {
                // How lucky we are. We equip it and don't fuss.
                return self.update_and_record(which, &candidate);
            } else {
                let _changed = &self.update_slot(other_hud, &return_to.clone());
                self.cycles.set_top(&other_hand, &return_to.form_string());
                return self.update_and_record(which, &candidate);
            }
        } else {
            // Phew. Okay. Now we're on to the one-handers equipped cases. These are easier.
            let maybe_candidate = if let Some(other_equipped) = self.visible.get(&other_hud) {
                // Are we dual-wielding? If so, do we have at least two?
                if !other_equipped.count_matters() || other_equipped.count() > 1 {
                    self.cycles.advance(which, 1)
                } else {
                    self.cycles.advance_skipping(which, other_equipped.clone())
                }
            } else {
                self.cycles.advance(which, 1)
            };

            if let Some(candidate) = maybe_candidate {
                let item = self.cache.get(&candidate);
                return self.update_and_record(which, &item);
            }
        }

        // If we got here, we got nothin'.
        KeyEventResponse::handled()
    }

    fn advance_ammo(&mut self) -> KeyEventResponse {
        let form_string = specEquippedAmmo();
        let mut ammotypes = getAmmoInventory();
        if ammotypes.len() < 2 {
            // do nothing
            log::info!("You don't have any ammo options to advance to. Doing nothing.");
            KeyEventResponse::default()
        } else {
            // This array is sorted by damage type. so we find ourselves then choose next
            let maybe_next = if let Some(idx) = ammotypes.iter().position(|xs| *xs == form_string) {
                ammotypes.rotate_left(idx + 1);
                ammotypes.first()
            } else {
                ammotypes.last()
            };
            if let Some(next) = maybe_next {
                let_cxx_string!(form_spec = next);
                equipAmmo(&form_spec);
                KeyEventResponse::handled()
            } else {
                log::warn!("Something very strange just happened. Ammo types: {ammotypes:?}");
                honk();
                KeyEventResponse::handled()
            }
        }
    }

    /// Update a HUD slot for one of the two hands, and return that we handled it.
    fn update_and_record(&mut self, which: &CycleSlot, next: &HudItem) -> KeyEventResponse {
        let hud = HudElement::from(which);
        self.update_slot(hud, next);

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
        if let Some(form_string) = self.cycles.get_top(&CycleSlot::Utility) {
            let item = self.cache.get(&form_string);
            log::info!("Activating utility item: name='{}';", item.name());
            if matches!(
                item.kind(),
                BaseType::Potion(PotionType::Poison) | BaseType::Food(_)
            ) {
                cxx::let_cxx_string!(form_spec = item.form_string());
                consumePotion(&form_spec);
            } else if item.form_string() == "health_proxy" {
                chooseHealthPotion();
            } else if item.form_string() == "magicka_proxy" {
                chooseMagickaPotion();
            } else if item.form_string() == "stamina_proxy" {
                chooseStaminaPotion();
            } else if item.is_potion() {
                cxx::let_cxx_string!(form_spec = item.form_string());
                consumePotion(&form_spec);
            } else if item.is_armor() {
                cxx::let_cxx_string!(form_spec = item.form_string());
                cxx::let_cxx_string!(name = item.name());
                toggleArmor(&form_spec, &name);
            } else if item.is_ammo() {
                cxx::let_cxx_string!(form_spec = item.form_string());
                equipAmmo(&form_spec)
            }
        } else {
            log::debug!("No item at top of utility cycle to use.");
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
    /// This function implements a critical behavior in the mod: equipping
    /// items. When the delay timer expires, we're notified to act on the
    /// player's changes to the cycle rotation. The delay exists to let the
    /// player tap a hotkey repeatedly to look at the items in a cycle without
    /// equipping each one of them as they go. Instead we wait for a little bit,
    /// and if we've had no more hotkey events, we act.
    ///
    /// We do not act here on cascading changes. Instead, we let the equipped-change
    /// callback decide what to do when, e.g., a two-handed item is equipped.
    pub fn timer_expired(&mut self, which: Action) {
        // Has a long press action timer fired? If so, we do the long press action
        // for this key. We know there's one because we would not have started a
        // timer if there wasn't.
        if matches!(
            which,
            Action::LongPressLeft
                | Action::LongPressPower
                | Action::LongPressRight
                | Action::LongPressUtility
        ) {
            self.handle_long_press(which);
            return;
        }

        let tracked = self.get_tracked_key(&Hotkey::from(&which));
        if tracked.is_pressed() {
            // Here's the reasoning. The player might be mid-long-press, in
            // which case we do not want to interrupt by equipping. The player
            // might be mid-short-tap, in which case the timer will get started
            // again on key up.
            return;
        }

        let hud = HudElement::from(which);
        if matches!(which, Action::Equipment) {
            self.equip_selected_set();
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
        if matches!(kind, BaseType::HandToHand) {
            log::info!("Melee time! Unequipping slot {which:?} so you can go punch a dragon.");
            if matches!(which, Action::Left) {
                // TODO wasteful but better than a magic string?
                self.left_hand_cached = HudItem::make_unarmed_proxy().form_string();
            } else {
                self.right_hand_cached = HudItem::make_unarmed_proxy().form_string();
            }
            unequipSlot(which);
            return;
        }

        if matches!(which, Action::Power) {
            // Equip that fus-ro-dah, dovahkin!
            if let BaseType::Shout(t) = kind {
                log::info!("{}", t.translation());
            }
            cxx::let_cxx_string!(form_spec = item.form_string());
            equipShout(&form_spec);
            return;
        }

        if !item.two_handed() {
            if which == Action::Left {
                self.left_hand_cached = item.form_string();
            } else {
                self.right_hand_cached = item.form_string();
            }
        }
        self.equip_item(item, which);
    }

    /// Handle a long-press timer firing.
    fn handle_long_press(&mut self, which: Action) {
        match which {
            Action::LongPressLeft => {
                let do_this = Hotkey::Left.long_press_action();
                if !matches!(do_this, RequestedAction::None) {
                    self.do_hand_action(do_this, Action::Left, CycleSlot::Left);
                    stopTimer(Action::Left);
                }
            }
            Action::LongPressRight => {
                let do_this = Hotkey::Right.long_press_action();
                if !matches!(do_this, RequestedAction::None) {
                    self.do_hand_action(do_this, Action::Right, CycleSlot::Right);
                    stopTimer(Action::Right);
                }
            }
            Action::LongPressPower => match Hotkey::Power.long_press_action() {
                RequestedAction::Advance => {
                    self.advance_cycle_power();
                    stopTimer(Action::Power);
                }
                RequestedAction::Unequip => {
                    unequipSlot(Action::Power);
                    stopTimer(Action::Power);
                }
                _ => {}
            },
            Action::LongPressUtility => match Hotkey::Utility.long_press_action() {
                RequestedAction::Advance => {
                    self.handle_cycle_utility();
                    stopTimer(Action::Utility);
                }
                RequestedAction::Consume => {
                    self.use_utility_item();
                    stopTimer(Action::Utility);
                }
                _ => {}
            },
            _ => {}
        }
    }

    /// Convenience function for equipping any equippable.
    fn equip_item(&self, item: &HudItem, which: Action) {
        if !matches!(which, Action::Right | Action::Left | Action::Utility) {
            return;
        }

        let kind = item.kind();
        cxx::let_cxx_string!(form_spec = item.form_string());
        cxx::let_cxx_string!(name = item.name());
        log::debug!("about to equip this item: slot={:?}; {}", which, item);

        if kind.is_magic() || kind.left_hand_ok() || kind.right_hand_ok() {
            equipWeapon(&form_spec, which, &name);
        } else if kind.is_armor() {
            toggleArmor(&form_spec, &name);
        } else if matches!(kind, BaseType::Ammo(_)) {
            equipAmmo(&form_spec);
        } else {
            log::info!(
                "We did nothing with item {}. Probably a missing feature!",
                item
            );
        }
    }

    /// We get this event when the player is using CGO and has switched grip mode.
    pub fn handle_grip_change(&mut self, using_alt_grip: bool) {
        // Record this in a local var so we can respect it when we equip new things.
        log::info!("CGO grip change observed; alt-grip={using_alt_grip};");
        self.cgo_alt_grip = using_alt_grip;

        let spec = specEquippedRight();
        let item = self.cache.get(&spec);
        if hasRangedEquipped() || !item.is_weapon() {
            // We have nothing to update in this case.
            return;
        }

        if using_alt_grip && item.two_handed() {
            // IFF we had been equipping a two-hander, we're now equipping a one-hander.
            self.switch_to_one_hander();
        } else if using_alt_grip && !item.two_handed() {
            // This is weird stuff. CGO will make you hold this in two hands.
            let left = specEquippedLeft();
            self.switch_to_two_hander();
            self.left_hand_cached = left;
        } else if item.two_handed() {
            // Alt grip is now OFF and we are holding what is normally a two-hander.
            self.switch_to_two_hander();
        } else {
            // Alt grip is now OFF and we are holding what is normally a one-hander.
            self.switch_to_one_hander();
        }
    }

    /// The very tiny shared logic for switching from a two-hander to a one-hander.
    fn switch_to_two_hander(&mut self) {
        self.update_slot(HudElement::Left, &HudItem::default());
        unequipSlot(Action::Left);
    }

    /// Shared logic for handling the switch to a one-handed weapon from a two-hander.
    fn switch_to_one_hander(&mut self) {
        if !self.left_hand_cached.is_empty() {
            let unarmed = HudItem::make_unarmed_proxy();
            let prev_left = self.left_hand_cached.clone();
            log::trace!(
                "re-requipping what we previously had in the LEFT hand; spec={};",
                prev_left
            );
            if prev_left == unarmed.form_string() {
                unequipSlot(Action::Left);
                self.update_slot(HudElement::Left, &unarmed);
            } else {
                let item = self.cache.get(&prev_left);
                self.update_slot(HudElement::Left, &item);
                cxx::let_cxx_string!(form_spec = prev_left.clone());
                cxx::let_cxx_string!(name = item.name());
                reequipHand(Action::Left, &form_spec, &name);
            }
        } else if let Some(left_next) = self.cycles.get_top(&CycleSlot::Left) {
            let item = self.cache.get(&left_next);
            self.left_hand_cached = left_next.clone();
            self.update_slot(HudElement::Left, &item);
            cxx::let_cxx_string!(form_spec = left_next);
            cxx::let_cxx_string!(name = item.name());
            reequipHand(Action::Left, &form_spec, &name);
        }
    }

    /// Helper functions for deciding if an item is two-handed in practice or
    /// not. If you're NOT using CGO, this is the same as asking if an item is
    /// two-handed or not. If you are using CGO, it's more complicated.
    fn treat_as_two_handed(&self, item: &HudItem) -> bool {
        (self.cgo_alt_grip && !item.two_handed()) || (!self.cgo_alt_grip && item.two_handed())
    }

    /// An item that was equipped is no longer equipped. Empty out a HUD slot if
    /// necessary. We take no other actions.
    pub fn handle_item_unequipped(
        &mut self,
        unequipped_spec: &String,
        equipped_right: &String,
        equipped_left: &String,
    ) -> bool {
        // Here we only care about updating the HUD. We let the rest fall where may.
        // We ONLY ever empty a visible slot here.
        log::trace!("item UNequipped; right={equipped_right}; left={equipped_left}; unequipped_spec={unequipped_spec};");
        let right_vis = self.visible.get(&HudElement::Right);
        let left_vis = self.visible.get(&HudElement::Left);
        let empty = HudItem::default();
        let item = self.cache.get(unequipped_spec);

        match item.kind() {
            BaseType::Ammo(_) => return self.update_slot(HudElement::Ammo, &empty),
            BaseType::Light(_) => return self.update_slot(HudElement::Left, &empty),
            BaseType::Power(_) => return self.update_slot(HudElement::Power, &empty),
            BaseType::Shout(_) => return self.update_slot(HudElement::Power, &empty),
            _ => {}
        }

        // This works for scrolls, spells, weapons, torches, and shields.
        if let Some(visible) = right_vis {
            if (equipped_right != unequipped_spec) && *unequipped_spec == visible.form_string() {
                return self.update_slot(HudElement::Right, &empty);
            }
        }
        if let Some(visible) = left_vis {
            if (equipped_left != unequipped_spec) && *unequipped_spec == visible.form_string() {
                return self.update_slot(HudElement::Left, &empty);
            }
        }
        false
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
        form_spec: &String,
        equipped_right: &String,
        equipped_left: &String,
    ) -> bool {
        if !equipped {
            return self.handle_item_unequipped(form_spec, equipped_right, equipped_left);
        }
        let item = self.cache.get_with_refresh(form_spec);
        let right = form_spec == equipped_right;
        let left = form_spec == equipped_left;
        let prefix = if right && left {
            "Item equipped in both hands"
        } else if right {
            "Item equipped in right hand"
        } else if left {
            "Item equipped in left hand"
        } else {
            "Item equipped"
        };
        log::info!(
            "{prefix}: name='{}'; icon='{}'; form_spec='{form_spec}'",
            item.name(),
            item.icon()
        );

        if item.is_ammo() {
            if let Some(visible) = self.visible.get(&HudElement::Ammo) {
                if visible.form_string() != *form_spec {
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

        if item.is_utility() {
            // We do nothing. We are the source of truth for non-ammo in the utility view.
            return false;
        }

        if item.is_power() {
            if let Some(visible) = self.visible.get(&HudElement::Power) {
                if visible.form_string() != *form_spec {
                    self.update_slot(HudElement::Power, &item);
                    self.cycles.set_top(&CycleSlot::Power, &item.form_string());
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
        // The hard part starts. Earlier, we did our best to set up the HUD to
        // show what we want in each hand. So we look at the item equipped: does
        // it match an earlier decision? If so, make the other decision happen
        // as well. If the equip event was NOT driven by the HUD, we have some
        // more work to do.
        //
        // CGO throws a wrinkle into all of this. If we're using alt-grip mode,
        // two-hander *melee* weapons are treated like one-handers and
        // one-handers are used with both hands. Cats living with dogs. Real
        // end-of-the-world type stuff.

        let treat_as_two_hander = self.treat_as_two_handed(&item);
        let switching = item.two_handed() != self.two_hander_equipped;
        log::trace!("weapon grip normally={}; alt-grip={}; we are treating it like: 2-hander={treat_as_two_hander}; switching={switching};",
            item.two_handed(), self.cgo_alt_grip);
        self.two_hander_equipped = item.two_handed();

        if treat_as_two_hander && right {
            let changed = self.update_slot(HudElement::Right, &item);
            if changed {
                // Change was out of band. We need to react by spinning the cycle around if possible.
                self.cycles.set_top(&CycleSlot::Right, &item.form_string());
            }
            self.update_slot(HudElement::Left, &HudItem::default());
            return changed;
        } else if treat_as_two_hander && left {
            log::debug!("treat_as_two_hander + left detected; item={item}");
            // TODO The left hud slot should be cleared and the left hand unequipped.
            // but I'm also not sure we ever get here.
            return false;
        }

        // It's a one-hander (effectively). Does it match an earlier decision?

        let rightie = specEquippedRight();
        let leftie = specEquippedLeft();
        // log::trace!(
        //     "form strings: item={}; equipped-right={}; equipped-left={}; two-hander-equipped={}; two-handed={}; name='{}';",
        //     item.form_string(),
        //     rightie,
        //     leftie,
        //     self.two_hander_equipped,
        //     item.two_handed(),
        //     item.name(),
        // );
        let leftvis = self
            .visible
            .get(&HudElement::Left)
            .map_or("".to_string(), |xs| xs.form_string());
        let rightvis = self
            .visible
            .get(&HudElement::Right)
            .map_or("".to_string(), |xs| xs.form_string());

        let right_unexpected = rightvis != rightie;
        let left_unexpected = leftvis != leftie;

        if right && right_unexpected {
            self.right_hand_cached = item.form_string().clone();
            self.update_slot(HudElement::Right, &item);
        } else if left && left_unexpected {
            self.left_hand_cached = item.form_string().clone();
            self.update_slot(HudElement::Left, &item);
        }

        // If the player is now a werewolf or a vampire, we do not
        // attempt to equip anything ourselves.
        if isVampireLord() || isWerewolf() {
            return false; // I forget what this means. do we even use it?
        }

        if !switching {
            // We are not switching from two-hander to one-hander. Phew.
            return left_unexpected || right_unexpected;
        }

        // We just swapped from a two-hander to a one-hander.
        // Now we earn our keep.

        let unarmed = HudItem::make_unarmed_proxy();

        if right {
            // This function is re-used for the right hand.
            self.switch_to_one_hander();
        }

        if left {
            // The item is effectively a one-hander, and it's now in our left hand.
            if !self.right_hand_cached.is_empty() {
                let prev_right = self.right_hand_cached.clone();
                log::debug!(
                    "re-requipping what we previously had in the right hand; spec={};",
                    prev_right
                );
                if prev_right == unarmed.form_string() {
                    unequipSlot(Action::Right);
                    self.update_slot(HudElement::Right, &unarmed);
                } else {
                    let item = self.cache.get(&prev_right);
                    self.update_slot(HudElement::Right, &item);
                    cxx::let_cxx_string!(form_spec = prev_right);
                    cxx::let_cxx_string!(name = item.name());
                    reequipHand(Action::Right, &form_spec, &name);
                }
            } else if let Some(right_next) = self.cycles.get_top(&CycleSlot::Right) {
                self.right_hand_cached = right_next.clone();
                let item = self.cache.get(&right_next);
                cxx::let_cxx_string!(form_spec = right_next);
                cxx::let_cxx_string!(name = item.name());
                reequipHand(Action::Right, &form_spec, &name);
                self.update_slot(HudElement::Right, &item);
            }
        }

        left_unexpected || right_unexpected
    }

    /// Get the item equipped in a specific slot.
    /// Called by the HUD rendering loop in the ImGui code.
    pub fn entry_to_show_in_slot(&self, slot: HudElement) -> Box<HudItem> {
        let Some(candidate) = self.visible.get(&slot) else {
            // log::debug!("nothing to draw in slot {slot:?}");
            return Box::<HudItem>::default();
        };

        Box::new(candidate.clone()) // this clone is in a hot path
    }

    /// Call when loading or otherwise needing to reinitialize the HUD.
    ///
    /// Updates will only happen here if the player changed equipment
    /// out of band, e.g., by using a menu, and only then if we screwed
    /// up an equip event. So don't call it except at initialization.
    fn update_hud(&mut self) {
        self.cgo_alt_grip = useCGOAltGrip();
        let right_spec = specEquippedRight();
        let right_entry = self.cache.get(&right_spec);

        let treat_right_as_2h = self.treat_as_two_handed(&right_entry);
        let right_changed = self.update_slot(HudElement::Right, &right_entry);
        if !treat_right_as_2h {
            self.right_hand_cached = right_entry.form_string();
        }

        let left_spec = specEquippedLeft();
        let left_entry = self.cache.get(&left_spec);
        let treat_left_as_2h = self.treat_as_two_handed(&left_entry);

        let left_unexpected = if !treat_left_as_2h {
            self.left_hand_cached = left_entry.form_string();
            self.update_slot(HudElement::Left, &left_entry)
        } else {
            // Two-handed item in the left hand, which means we show it as empty.
            self.left_hand_cached = self
                .cycles
                .get_top(&CycleSlot::Left)
                .map_or("".to_string(), |xs| xs);
            self.update_slot(HudElement::Left, &HudItem::default())
        };
        self.two_hander_equipped = right_entry.two_handed(); // same item will be in both hands

        let power_form = specEquippedPower();
        let power = self.cache.get(&power_form);
        let power_changed = self.update_slot(HudElement::Power, &power);

        let ammo_form = specEquippedAmmo();
        let ammo = self.cache.get(&ammo_form);
        self.update_slot(HudElement::Ammo, &ammo);

        if let Some(utility) = self.cycles.get_top(&CycleSlot::Utility) {
            let item = self.cache.get(&utility);
            self.update_slot(HudElement::Utility, &item);
        } else {
            self.update_slot(HudElement::Utility, &HudItem::default());
        }

        log::info!(
            "HUD initialized. Now showing: power='{}'; left='{}'; right='{}'; ammo='{}';",
            power.name(),
            left_entry.name(),
            right_entry.name(),
            ammo.name(),
        );

        // If any of our equipped items is in a cycle, make that item the top item
        // so advancing the cycles works as expected.
        if power_changed {
            self.cycles.set_top(&CycleSlot::Power, &power.form_string());
        }
        if left_unexpected {
            self.cycles
                .set_top(&CycleSlot::Left, &left_entry.form_string());
        }
        if right_changed {
            self.cycles
                .set_top(&CycleSlot::Right, &right_entry.form_string());
        }
    }

    /// Update the displayed slot for the specified HUD element.
    fn update_slot(&mut self, slot: HudElement, new_item: &HudItem) -> bool {
        log::trace!("updating hud slot '{slot}'; visible: {new_item}");
        if let Some(replaced) = self.visible.insert(slot, new_item.clone()) {
            replaced != *new_item
        } else {
            false
        }
    }

    /// The player has toggled a favorite. If our settings instruct us to link favorites
    /// to cycle entries, we do so now. We do our best to assign items to the cycles where
    /// they make the most sense. 90% of the work is contructing a useful feedback message
    /// for the player about what we did.
    pub fn handle_favorite_event(
        &mut self,
        _button: &ButtonEvent,
        is_favorite: bool,
        item: HudItem,
    ) {
        if !settings().link_to_favorites() {
            return;
        }

        log::debug!("handle_favorite_event(); is_favorite={is_favorite};");
        log::debug!("    {item}; two-handed={};", item.two_handed());

        let maybe_message = if !is_favorite {
            // This formerly-favorite item is now disliked.
            let format = translated_key(FMT_ITEM_REMOVED);
            let mut vars = HashMap::new();
            vars.insert("item".to_string(), item.name());

            let maybe_cycle = if item.is_utility() {
                if self.cycles.remove_item(CycleSlot::Utility, &item) {
                    Some(translated_key(FMT_ITEM_UTILITIES_CYCLE))
                } else {
                    None
                }
            } else if item.is_power() {
                if self.cycles.remove_item(CycleSlot::Power, &item) {
                    Some(translated_key(FMT_ITEM_POWERS_CYCLE))
                } else {
                    None
                }
            } else if item.two_handed() {
                if self.cycles.remove_item(CycleSlot::Right, &item) {
                    Some(translated_key(FMT_ITEM_RIGHT_CYCLE))
                } else {
                    None
                }
            } else {
                let removed_right = self.cycles.remove_item(CycleSlot::Right, &item);
                let removed_left = self.cycles.remove_item(CycleSlot::Left, &item);
                if removed_right && removed_left {
                    Some(translated_key(FMT_ITEM_BOTH_HANDS))
                } else if removed_left {
                    Some(translated_key(FMT_ITEM_LEFT_CYCLE))
                } else if removed_right {
                    Some(translated_key(FMT_ITEM_RIGHT_CYCLE))
                } else {
                    None
                }
            };
            if let Some(cycle) = maybe_cycle {
                vars.insert("cycle".to_string(), cycle);
                strfmt(&format, &vars).ok()
            } else {
                None
            }
        } else {
            // This item is a new fave! Add to whatever the appropriate cycle is.
            let format = translated_key(FMT_ITEM_ADDED);
            let mut vars = HashMap::new();
            vars.insert("item".to_string(), item.name());

            let maybe_cycle = if item.is_utility() {
                if self.cycles.add_item(CycleSlot::Utility, &item) {
                    Some(translated_key(FMT_ITEM_UTILITIES_CYCLE))
                } else {
                    None
                }
            } else if item.is_power() {
                if self.cycles.add_item(CycleSlot::Power, &item) {
                    Some(translated_key(FMT_ITEM_POWERS_CYCLE))
                } else {
                    None
                }
            } else if item.two_handed() || matches!(item.kind(), BaseType::Scroll(_)) {
                if self.cycles.add_item(CycleSlot::Right, &item) {
                    Some(translated_key(FMT_ITEM_RIGHT_CYCLE))
                } else {
                    None
                }
            } else if item.is_spell() || (item.right_hand_ok() && item.count() > 1) {
                let added_right = self.cycles.add_item(CycleSlot::Right, &item);
                let added_left = self.cycles.add_item(CycleSlot::Left, &item);
                if added_right && added_left {
                    Some(translated_key(FMT_ITEM_BOTH_HANDS))
                } else if added_left {
                    Some(translated_key(FMT_ITEM_LEFT_CYCLE))
                } else if added_right {
                    Some(translated_key(FMT_ITEM_RIGHT_CYCLE))
                } else {
                    None
                }
            } else if item.right_hand_ok() {
                if self.cycles.add_item(CycleSlot::Right, &item) {
                    Some(translated_key(FMT_ITEM_RIGHT_CYCLE))
                } else {
                    None
                }
            } else if self.cycles.add_item(CycleSlot::Left, &item) {
                Some(translated_key(FMT_ITEM_LEFT_CYCLE))
            } else {
                None
            };
            if let Some(cycle) = maybe_cycle {
                vars.insert("cycle".to_string(), cycle);
                strfmt(&format, &vars).ok()
            } else {
                None
            }
        };

        if let Some(msg) = maybe_message {
            log::info!("{msg}");
            notify(&msg);
            self.assign_keyword(&item);
        } else {
            log::info!("Favoriting or unfavoriting didn't change cycles.");
        }
    }

    pub fn handle_menu_event(&mut self, key: u32, button: &ButtonEvent) -> bool {
        // Much simpler than the cycle loop. We care if the cycle modifier key
        // is down (if one is set), and we care if the cycle button itself has
        // been pressed.
        let hotkey = Hotkey::from(key);
        if matches!(hotkey, Hotkey::None) {
            return false;
        }

        // You want a fun bug? I'll give you a fun bug. If these two keys are the
        // same, which they might be, we suddenly have to become context-aware.
        let hotkey = if matches!(hotkey, Hotkey::ActivateModifier) {
            Hotkey::MenuModifier
        } else {
            hotkey
        };

        self.update_tracked_key(&hotkey, button, true);
        if !hotkey.is_cycle_key() || !button.IsDown() {
            return false;
        }

        let options = settings();
        let menu_method = options.how_to_toggle();

        match menu_method {
            ActivationMethod::Hotkey => true,
            ActivationMethod::LongPress => {
                log::debug!("checking for long press in menu");
                // if it's not found, will never be a long press
                self.get_tracked_key(&hotkey).is_long_press()
            }
            ActivationMethod::Modifier => {
                let modkey = self.get_tracked_key(&Hotkey::MenuModifier);
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
    pub fn handle_toggle_item(&mut self, action: Action, item: HudItem) {
        let Ok(cycle_slot) = CycleSlot::try_from(action) else {
            return;
        };

        let result = self.cycles.toggle(&cycle_slot, item.clone());

        if matches!(result, MenuEventResponse::ItemRemoved) && matches!(action, Action::Utility) {
            if let Some(topmost) = self.cycles.get_top(&CycleSlot::Utility) {
                let item = self.cache.get(&topmost);
                self.update_slot(HudElement::Utility, &item);
            } else {
                self.update_slot(HudElement::Utility, &HudItem::default());
            }
        }

        // notify the player what happened...
        let verb = match result {
            MenuEventResponse::ItemAdded => translated_key(FMT_ITEM_ADDED),
            MenuEventResponse::ItemRemoved => translated_key(FMT_ITEM_REMOVED),
            MenuEventResponse::ItemInappropriate => translated_key(FMT_ITEM_REJECTED),
            MenuEventResponse::TooManyItems => translated_key(FMT_ITEM_TOOMANY),
            _ => translated_key(FMT_ITEM_NOCHANGE),
        };
        let cyclename = match action {
            Action::Power => translated_key(FMT_ITEM_POWERS_CYCLE),
            Action::Left => translated_key(FMT_ITEM_LEFT_CYCLE),
            Action::Right => translated_key(FMT_ITEM_RIGHT_CYCLE),
            Action::Utility => translated_key(FMT_ITEM_UTILITIES_CYCLE),
            _ => "any".to_string(), // should be unreachable
        };

        let mut vars = HashMap::new();
        vars.insert("item".to_string(), item.name());
        vars.insert("cycle".to_string(), cyclename);
        if let Ok(message) = strfmt(&verb, &vars) {
            log::info!("{}; kind={:?};", message, item.kind());
            notify(&message);
            self.assign_keyword(&item);
        } else {
            log::debug!("No notification sent to player because message couldn't be formatted");
        }
    }

    // Update the state of a tracked key so we can handle modifier keys and long-presses.
    // Returns whether the calling level should continue handling this key.
    fn update_tracked_key(&mut self, hotkey: &Hotkey, button: &ButtonEvent, in_menu: bool) -> bool {
        let mut retval = true;
        let tracking_long_presses = !in_menu && settings().start_long_press_timer(hotkey);
        let tracked = if let Some(previous) = self.tracked_keys.get_mut(hotkey) {
            // We have seen this key before.
            // Did this key just have a long-press event? if so, ignore a key-up.
            // We ask this question before we update the tracking data.
            if matches!(previous.state, KeyState::Pressed)
                && previous.is_long_press()
                && tracking_long_presses
            {
                retval = false;
            }

            previous.update(button);
            previous.clone()
        } else {
            let mut tracked = TrackedKey {
                key: hotkey.clone(),
                state: KeyState::default(),
                press_start: None,
            };
            tracked.update(button);
            self.tracked_keys.insert(hotkey.clone(), tracked.clone());
            tracked
        };

        // long press timers; not started if we're in a menu
        if tracking_long_presses {
            if matches!(tracked.state, KeyState::Down) {
                let duration = settings().long_press_ms();
                match tracked.key {
                    Hotkey::Left => startTimer(Action::LongPressLeft, duration),
                    Hotkey::Right => startTimer(Action::LongPressRight, duration),
                    Hotkey::Power => startTimer(Action::LongPressPower, duration),
                    Hotkey::Utility => startTimer(Action::LongPressUtility, duration),
                    _ => {}
                }
            } else if matches!(tracked.state, KeyState::Up) {
                match tracked.key {
                    Hotkey::Left => stopTimer(Action::LongPressLeft),
                    Hotkey::Right => stopTimer(Action::LongPressRight),
                    Hotkey::Power => stopTimer(Action::LongPressPower),
                    Hotkey::Utility => stopTimer(Action::LongPressUtility),
                    _ => {}
                }
            }
        }
        retval
    }

    fn get_tracked_key(&self, hotkey: &Hotkey) -> TrackedKey {
        if let Some(tracked) = self.tracked_keys.get(hotkey) {
            tracked.clone()
        } else {
            TrackedKey {
                key: Hotkey::None,
                state: KeyState::Up,
                press_start: None,
            }
        }
    }

    // ----------- equipment set functions

    /// Handle the power/shouts key being pressed.
    fn handle_cycle_equipset(&mut self, _hotkey: &Hotkey) -> KeyEventResponse {
        let le_options = settings();
        let cycle_method = le_options.cycle_advance_method();

        if matches!(cycle_method, ActivationMethod::Hotkey) {
            self.advance_cycle_equipset()
        } else if matches!(cycle_method, ActivationMethod::Modifier) {
            let hotkey = self.get_tracked_key(&Hotkey::CycleModifier);
            if hotkey.is_pressed() {
                self.advance_cycle_equipset()
            } else {
                KeyEventResponse::default()
            }
        } else {
            KeyEventResponse::default()
        }
    }

    /// Rotate to the next equipment set in the cycle and start the timer.
    fn advance_cycle_equipset(&mut self) -> KeyEventResponse {
        let candidate = self.cycles.advance_equipset(1);
        if let Some(_next) = candidate {
            KeyEventResponse {
                handled: true,
                start_timer: Action::Equipment,
                stop_timer: Action::None,
            }
        } else {
            KeyEventResponse {
                handled: true,
                ..Default::default()
            }
        }
    }

    /// Called when equipment timer expires: equip that equip set!
    fn equip_selected_set(&mut self) {
        let Some(equipset) = self.cycles.get_top_equipset() else {
            return;
        };
        log::debug!("Switching to equipment set '{}'.", equipset.name());
        if settings().equip_sets_unequip() {
            equipset.empty_slots().iter().for_each(|shift| {
                unequipSlotByShift(*shift);
            });
        }
        equipset.items().iter().for_each(|item| {
            let cached = self.cache.get(item);
            let_cxx_string!(form_spec = item.identifier());
            let_cxx_string!(name = cached.name());
            equipArmor(&form_spec, &name);
        });

        let set = HudItem::for_equip_set(equipset.name(), equipset.id(), equipset.icon.clone());
        self.update_slot(HudElement::EquipSet, &set);
    }

    /// Called by the MCM code when it is showing a list of all items in an equipment set.
    pub fn get_equipset_item_names(&mut self, id: u32) -> Vec<String> {
        if let Some(set) = self.cycles.equipset_by_id(id) {
            set.items
                .iter()
                .map(|xs| {
                    let item = self.cache.get(xs);
                    item.name()
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Use the icon assigned to a specific item as the icon to represent this
    /// equipment set. The named item should be in the given equip set, but we
    /// don't enforce that.
    pub fn set_equipset_icon(&mut self, id: u32, itemname: String) -> bool {
        let Some(set) = self.cycles.equipset_by_id(id) else {
            return false;
        };

        let found: Option<HudItem> = set.items.iter().find_map(|xs| {
            let item = self.cache.get(xs);
            if item.name() == itemname {
                Some(item)
            } else {
                None
            }
        });
        let Some(source) = found else {
            return false;
        };
        let icon = source.icon().clone();
        self.cycles.set_icon_by_id(id, icon)
    }

    /// Call this after any add/remove actions are fully complete to
    /// assign the appropriate keyword to an item to update the menu.
    fn assign_keyword(&self, item: &HudItem) {
        if item.is_power() {
            if self.cycles.includes(&CycleSlot::Power, item) {
                keywords::set_keyword(item, keywords::IN_CYCLE);
            } else {
                keywords::clear_keywords(item);
            }
        } else if item.is_utility() {
            if self.cycles.includes(&CycleSlot::Utility, item) {
                keywords::set_keyword(item, keywords::IN_CYCLE);
            } else {
                keywords::clear_keywords(item);
            }
        } else {
            let in_left = self.cycles.includes(&CycleSlot::Left, item);
            let in_right = self.cycles.includes(&CycleSlot::Right, item);
            if in_left && in_right {
                keywords::set_keyword(item, keywords::IN_CYCLE_BOTHHANDS);
            } else if in_right {
                keywords::set_keyword(item, keywords::IN_CYCLE_RIGHT);
            } else if in_left {
                keywords::set_keyword(item, keywords::IN_CYCLE_LEFT);
            } else {
                keywords::clear_keywords(item);
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

impl KeyEventResponse {
    pub fn handled() -> Self {
        Self {
            handled: true,
            stop_timer: Action::None,
            start_timer: Action::None,
        }
    }
}

impl From<u32> for Action {
    /// Turn the key code into an enum for easier processing.
    fn from(value: u32) -> Self {
        let settings = settings();

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
        } else if value == settings.equipset() as u32 {
            Action::Equipment
        } else if value == settings.unequip_hotkey() as u32 {
            Action::UnequipHands
        } else {
            Action::None
        }
    }
}

/// Keywords and functions for settings and removing them.
pub mod keywords {
    #[cfg(not(test))]
    use crate::plugin::{clearCycleKeywords, setCycleKeyword};
    use crate::HudItem;

    /// shouts, powers, utility items
    pub const IN_CYCLE: &str = "InSoulsyCycle";
    /// anything in right hand cycle (weapons, spells)
    pub const IN_CYCLE_RIGHT: &str = "InSoulsyCycleRight";
    /// anything in left hand cycle (weapons, spells, torches, shields)
    pub const IN_CYCLE_LEFT: &str = "InSoulsyCycleLeft";
    /// anything in both hand cycles (weapons, spells)
    pub const IN_CYCLE_BOTHHANDS: &str = "InSoulsyCycleBothHands";

    /// Add a keyword to an item.
    #[cfg(not(test))]
    pub fn set_keyword(item: &HudItem, keyword: &str) {
        log::trace!("adding keyword to '{}': {keyword}", item.name());
        cxx::let_cxx_string!(form_spec = item.form_string());
        cxx::let_cxx_string!(kwd = keyword);
        setCycleKeyword(&form_spec, &kwd);
    }

    #[cfg(test)]
    pub fn set_keyword(item: &HudItem, keyword: &str) {
        log::debug!("would be adding soulsy keyword {keyword} to {item}");
    }

    #[cfg(not(test))]
    pub fn clear_keywords(item: &HudItem) {
        log::trace!("removing all soulsy keywords from '{}';", item.name());
        cxx::let_cxx_string!(form_spec = item.form_string());
        clearCycleKeywords(&form_spec);
    }

    #[cfg(test)]
    pub fn clear_keywords(item: &HudItem) {
        log::debug!("would be removing all soulsy keywords from {item}");
    }
}

/// What the controller did with a specific menu press event.
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

/// TODO: derivable?
impl Default for Controller {
    fn default() -> Self {
        Controller::new()
    }
}

#[cfg(not(test))]
pub fn notify(msg: &str) {
    cxx::let_cxx_string!(message = msg);
    notifyPlayer(&message);
}

#[cfg(test)]
pub fn notify(_msg: &str) {}

/// Convenience function for doing the cxx macro boilerplate before
/// calling C++ with a string.
#[cfg(not(test))]
pub fn translated_key(key: &str) -> String {
    let_cxx_string!(cxxkey = key);
    lookupTranslation(&cxxkey)
}

#[cfg(test)]
pub fn translated_key(key: &str) -> String {
    format!("translation of {key}")
}

const FMT_ITEM_REMOVED: &str = "$SoulsyHUD_fmt_ItemRemoved";
const FMT_ITEM_ADDED: &str = "$SoulsyHUD_fmt_ItemAdded";
const FMT_ITEM_REJECTED: &str = "$SoulsyHUD_fmt_ItemRejected";
const FMT_ITEM_TOOMANY: &str = "$SoulsyHUD_fmt_TooMany";
const FMT_ITEM_NOCHANGE: &str = "$SoulsyHUD_fmt_NoChange";
const FMT_ITEM_POWERS_CYCLE: &str = "$SoulsyHUD_fmt_PowersCycle";
const FMT_ITEM_UTILITIES_CYCLE: &str = "$SoulsyHUD_fmt_UtilitiesCycle";
const FMT_ITEM_LEFT_CYCLE: &str = "$SoulsyHUD_fmt_LeftHandCycle";
const FMT_ITEM_RIGHT_CYCLE: &str = "$SoulsyHUD_fmt_RightHandCycle";
const FMT_ITEM_BOTH_HANDS: &str = "$SoulsyHUD_fmt_BothHands";

/// Possible actions requested when a user presses a cycle key.
/// The action is determined using the key pressed, the presence of modifiers,
/// and various user settings.
pub enum RequestedAction {
    Advance,
    AdvanceAmmo,
    Consume,
    Match,
    Unequip,
    None,
}
