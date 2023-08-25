use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::Mutex;

use cxx::let_cxx_string;
use once_cell::sync::Lazy;

use super::cycles::*;
use super::keys::*;
use super::settings::{user_settings, ActivationMethod, UnarmedMethod};
use crate::data::item_cache::ItemCache;
use crate::data::potion::PotionType;
use crate::data::*;
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
    /// the hud item cache
    pub cache: ItemCache,
    /// The items the HUD should show right now.
    visible: HashMap<HudElement, HudItem>,
    /// True if we've got a two-handed weapon equipped right now.
    two_hander_equipped: bool,
    /// We cache the form spec of any left-hand item we were holding before a two-hander was equipped.
    left_hand_cached: String,
    /// We cache a right-hand form spec string similarly.
    right_hand_cached: String,
    tracked_keys: HashMap<HotkeyKind, TrackedKey>,
}

impl Controller {
    /// Make a controller with no information in it.
    pub fn new() -> Self {
        let cycles = CycleData::default();
        Controller {
            cycles,
            cache: ItemCache::new(
                NonZeroUsize::new(100)
                    .expect("cats and dogs living together! 100 is not non-zero!"),
            ),
            visible: HashMap::new(),
            two_hander_equipped: false,
            left_hand_cached: "".to_string(),
            right_hand_cached: "".to_string(),
            tracked_keys: HashMap::new(),
        }
    }

    pub fn validate_cycles(&mut self) {
        self.cycles.validate(&mut self.cache);
        // log::info!("after validation, cycles are: {}", self.cycles);
        self.update_hud();
    }

    pub fn clear_cycles(&mut self) {
        log::info!("Clearing all cycles. Turning off targeting computer.");
        self.cycles.clear();
    }

    pub fn cycle_names(&mut self, which: u32) -> Vec<String> {
        match which {
            0 => self.cycles.names(&CycleSlot::Power, &mut self.cache),
            1 => self.cycles.names(&CycleSlot::Utility, &mut self.cache),
            2 => self.cycles.names(&CycleSlot::Left, &mut self.cache),
            3 => self.cycles.names(&CycleSlot::Right, &mut self.cache),
            _ => Vec::new(),
        }
    }

    pub fn apply_settings(&mut self) {
        let settings = user_settings();

        match settings.unarmed_handling() {
            UnarmedMethod::AddToCycles => {
                let h2h = HudItem::make_unarmed_proxy();
                self.cache.record(h2h.clone());
                self.cycles.include_item(CycleSlot::Left, &h2h);
                self.cycles.include_item(CycleSlot::Right, &h2h);
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
            self.cycles.include_item(CycleSlot::Utility, &proxy);

            self.cycles.filter_kind(
                &CycleSlot::Utility,
                &BaseType::Potion(PotionType::Health),
                &mut self.cache,
            );
            let proxy = make_health_proxy();
            self.cache.record(proxy.clone());
            self.cycles.include_item(CycleSlot::Utility, &proxy);

            self.cycles.filter_kind(
                &CycleSlot::Utility,
                &BaseType::Potion(PotionType::Magicka),
                &mut self.cache,
            );
            let proxy = make_magicka_proxy();
            self.cache.record(proxy.clone());
            self.cycles.include_item(CycleSlot::Utility, &proxy);
        } else {
            let proxy = make_stamina_proxy();
            self.cycles.remove_item(CycleSlot::Utility, &proxy);
            let proxy = make_health_proxy();
            self.cycles.remove_item(CycleSlot::Utility, &proxy);
            let proxy = make_magicka_proxy();
            self.cycles.remove_item(CycleSlot::Utility, &proxy);
        }

        if !settings.autofade() {
            if self.cycles.hud_visible() {
                startAlphaTransition(true, 1.0);
            } else {
                startAlphaTransition(false, 0.0);
            }
        }

        self.cache.introspect();
    }

    /// The player's inventory changed! Act on it if we need to.
    pub fn handle_inventory_changed(&mut self, form_spec: &String, delta: i32) {
        let Some(item) = self.cache.update_count(form_spec.as_str(), delta) else {
            return;
        };

        let kind = item.kind().clone();
        let new_count = item.count();
        log::debug!("inventory count update: name='{}'; delta={delta}; count={new_count}", item.name());

        if kind.is_ammo() {
            if let Some(candidate) = self.visible.get_mut(&HudElement::Ammo) {
                if candidate.form_string() == *form_spec {
                    candidate.set_count(new_count);
                }
            }
        } else if kind.is_utility() {
            if let Some(candidate) = self.visible.get_mut(&HudElement::Utility) {
                if candidate.form_string() == *form_spec {
                    candidate.set_count(new_count);
                }
            }
        } else {
            if let Some(candidate) = self.visible.get_mut(&HudElement::Left) {
                if candidate.form_string() == *form_spec {
                    candidate.set_count(new_count);
                }
            }
            if let Some(candidate) = self.visible.get_mut(&HudElement::Right) {
                if candidate.form_string() == *form_spec {
                    candidate.set_count(new_count);
                }
            }
        }

        self.cycles
            .remove_zero_count_items(form_spec.as_str(), &kind, new_count);

        // update count of magicka, health, or stamina potions if we're grouped
        // TODO magic strings
        if kind.is_potion() && user_settings().group_potions() {
            if matches!(kind, BaseType::Potion(PotionType::Health)) {
                let count = healthPotionCount();
                self.cache.update_count("health_proxy", delta);
                self.cycles.remove_zero_count_items(
                    "health_proxy",
                    &BaseType::PotionProxy(Proxy::Health),
                    count,
                );
            }
            if matches!(kind, BaseType::Potion(PotionType::Magicka)) {
                let count = magickaPotionCount();
                self.cache.update_count("magicka_proxy", delta);
                self.cycles.remove_zero_count_items(
                    "magicka_proxy",
                    &BaseType::PotionProxy(Proxy::Magicka),
                    count,
                );
            }
            if matches!(kind, BaseType::Potion(PotionType::Stamina)) {
                self.cache.update_count("stamina_proxy", delta);
                let count = staminaPotionCount();
                self.cycles.remove_zero_count_items(
                    "stamina_proxy",
                    &BaseType::PotionProxy(Proxy::Stamina),
                    count,
                );
            }
        }

        if new_count > 0 {
            return;
        }

        if kind.is_utility() {
            if let Some(vis) = self.visible.get(&HudElement::Utility) {
                if vis.form_string() == *form_spec {
                    if let Some(formspec) = self.cycles.get_top(&CycleSlot::Utility) {
                        let item = self.cache.get(&formspec);
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

        let settings = user_settings();
        if user_settings().autofade() {
            show_briefly();
        }

        match hotkey {
            HotkeyKind::Power => self.handle_cycle_power(),
            HotkeyKind::Utility => self.handle_cycle_utility(),
            HotkeyKind::Left => self.handle_cycle_left(&hotkey),
            HotkeyKind::Right => self.handle_cycle_right(&hotkey),
            HotkeyKind::Activate => {
                let activation_method = settings.how_to_activate();
                if matches!(activation_method, ActivationMethod::Hotkey) {
                    self.use_utility_item()
                } else {
                    KeyEventResponse::default()
                }
            }
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
        let hotkey = self.get_tracked_key(&HotkeyKind::Utility);
        let is_long_press = hotkey.is_long_press();

        match activation_method {
            ActivationMethod::LongPress => {
                if is_long_press {
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
            _ => {}
        }

        match cycle_method {
            ActivationMethod::Hotkey => {
                log::debug!("cycling utilities/consumables");
                return self.advance_simple_cycle(&CycleSlot::Utility);
            }
            ActivationMethod::LongPress => {
                if is_long_press {
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
            let item = self.cache.get(&next);
            self.update_slot(hud, &item);
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

    fn handle_cycle_left(&mut self, hotkey: &HotkeyKind) -> KeyEventResponse {
        log::debug!("Left hand advance requested");

        // Left hand logic:
        // Do we have a bow equipped, and if so, is the "handle ammo" boolean set?
        // In that case, we switch ammo. Ammo is ordered least damaging -> most damaging.
        // Otherwise, it's the same as the right hand.

        let settings = user_settings();
        let tracked = self.get_tracked_key(hotkey);
        let cycle_ammo = settings.cycle_ammo();

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

        if unequip_requested {
            log::info!("unequipping left hand by request");
            let empty_item = HudItem::make_unarmed_proxy();
            unequipSlot(Action::Left);
            self.update_slot(HudElement::from(&CycleSlot::Left), &empty_item);
            self.cycles
                .set_top(&CycleSlot::Left, &empty_item.form_string());
            KeyEventResponse {
                handled: true,
                start_timer: Action::None,
                stop_timer: Action::Left,
            }
        } else if cycle_requested {
            if hasRangedEquipped() && cycle_ammo {
                self.advance_ammo()
            } else {
                self.advance_hand_cycle(&CycleSlot::Left)
            }
        } else {
            log::trace!("you need a modifier key down to advance the left hand");
            KeyEventResponse::default()
        }
    }

    fn handle_cycle_right(&mut self, hotkey: &HotkeyKind) -> KeyEventResponse {
        log::debug!("Right hand advance requested");

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
            log::info!("unequipping right hand by request");
            let empty_item = HudItem::make_unarmed_proxy();
            unequipSlot(action);
            self.update_slot(HudElement::from(&CycleSlot::Right), &empty_item);
            self.cycles
                .set_top(&CycleSlot::Right, &empty_item.form_string());
            KeyEventResponse {
                handled: true,
                start_timer: Action::None,
                stop_timer: action,
            }
        } else if cycle_requested {
            self.advance_hand_cycle(&CycleSlot::Right)
        } else {
            log::trace!("you need a modifier key down to advance the right hand");
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
            let Some(form_string) = self.cycles.peek_next(which) else {
                return KeyEventResponse {
                    handled: true,
                    ..Default::default()
                };
            };

            let candidate = self.cache.get(&form_string);

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
            if !candidate.kind().count_matters() || candidate.count() > 1 {
                self.cycles.advance(which, 1);
                let _changed = &self.update_slot(other_hud, &return_to.clone());
                self.cycles.set_top(&other_hand, &return_to.form_string());
                return self.update_and_record(which, &candidate);
            }

            // The worst case! Somebody's got to lose the battle for the single item,
            // and in this case it's the hand trying to cycle forward.
            let Some(form_string) = self.cycles.advance_skipping(which, return_to.clone()) else {
                honk();
                return KeyEventResponse {
                    handled: true,
                    ..Default::default()
                };
            };

            let candidate = self.cache.get(&form_string);
            if candidate.two_handed() {
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
                if !other_equipped.kind().count_matters() || other_equipped.count() > 1 {
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
        KeyEventResponse {
            handled: true,
            ..Default::default()
        }
    }

    fn advance_ammo(&mut self) -> KeyEventResponse {
        log::debug!("selecting next ammo");
        let form_string = specEquippedAmmo();
        let ammotypes = getAmmoInventory();
        if ammotypes.len() < 2 {
            // do nothing
            KeyEventResponse::default()
        } else {
            // this array is sorted by damage type. so we find ourselves then choose next
            let next = if let Some(pos) = ammotypes.iter().position(|xs| *xs == form_string) {
                if pos < ammotypes.len() - 2 {
                    ammotypes[pos + 1].to_string()
                } else {
                    ammotypes[0].to_string()
                }
            } else {
                ammotypes[ammotypes.len() - 1].to_string()
            };
            let_cxx_string!(form_spec = next);
            equipAmmo(&form_spec);
            KeyEventResponse {
                handled: true,
                ..Default::default()
            }
        }
    }

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
        log::debug!("using utility item");

        if let Some(form_string) = self.cycles.get_top(&CycleSlot::Utility) {
            let item = self.cache.get(&form_string);
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
            } else if item.kind().is_potion() {
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
        if matches!(kind, BaseType::HandToHand) {
            log::info!("melee time! unequipping slot {which:?}");
            if which == Action::Left {
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
            log::info!("Fus-ro-dah!");
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

    /// Convenience function for equipping any equippable.
    fn equip_item(&self, item: &HudItem, which: Action) {
        if !matches!(which, Action::Right | Action::Left | Action::Utility) {
            return;
        }

        let kind = item.kind();
        cxx::let_cxx_string!(form_spec = item.form_string());
        log::info!("about to equip this item: slot={:?}; {}", which, item);

        if kind.is_magic() || kind.left_hand_ok() || kind.right_hand_ok() {
            equipWeapon(&form_spec, which);
        } else if kind.is_armor() {
            equipArmor(&form_spec);
        } else if matches!(kind, BaseType::Ammo(_)) {
            equipAmmo(&form_spec);
        } else {
            log::info!("we did nothing with item {}", item);
        }
    }

    pub fn handle_item_unequipped(&mut self, form_spec: &String, right: bool, left: bool) -> bool {
        // Here we only care about updating the HUD. We let the rest fall where may.
        log::debug!("item UNequipped; right={right}; left={left}; form_spec={form_spec};");

        let right_vis = self.visible.get(&HudElement::Right);
        let left_vis = self.visible.get(&HudElement::Left);
        if right {
            if let Some(visible) = right_vis {
                if visible.form_string() == *form_spec {
                    let empty = HudItem::default();
                    return self.update_slot(HudElement::Right, &empty);
                }
            }
        } else if left {
            if let Some(visible) = left_vis {
                if visible.form_string() == *form_spec {
                    let empty = HudItem::default();
                    return self.update_slot(HudElement::Left, &empty);
                }
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
        right: bool,
        left: bool,
    ) -> bool {
        if !equipped {
            return self.handle_item_unequipped(form_spec, right, left);
        }
        log::debug!("item equipped; right={right}; left={left}; form_spec={form_spec};");
        let item = self.cache.get(form_spec);

        if item.kind().is_ammo() {
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

        if item.kind().is_utility() {
            // We do nothing. We are the source of truth for non-ammo on the utility view.
            return false;
        }

        if item.kind().is_power() {
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
            self.left_hand_cached,
            self.right_hand_cached
        );

        if item.two_handed() {
            log::trace!("we have a two-hander. we should exit after this block.");
            let changed = self.update_slot(HudElement::Right, &item);
            if changed {
                // Change was out of band. We need to react.
                self.cycles.set_top(&CycleSlot::Right, &item.form_string());
            }
            self.update_slot(HudElement::Left, &HudItem::default());
            self.two_hander_equipped = true;
            return changed;
        }

        // It's a one-hander. Does it match an earlier decision?

        let rightie = specEquippedRight();
        let leftie = specEquippedLeft();
        log::trace!(
            "form strings: item={}; equipped-right={}; requipped-left={}; two-hander-equipped={}; two-handed={}; name='{}';",
            item.form_string(),
            rightie,
            leftie,
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

        let right_unexpected = rightvis != rightie;
        let left_unexpected = leftvis != leftie;

        if right && right_unexpected {
            self.update_slot(HudElement::Right, &item);
        } else if left && left_unexpected {
            self.update_slot(HudElement::Left, &item);
        }

        if !self.two_hander_equipped {
            // We are done. Phew.
            return left_unexpected || right_unexpected;
        }

        // We just swapped from a two-hander to a one-hander.
        // Now we earn our keep.

        let unarmed = HudItem::make_unarmed_proxy();

        if right {
            if !self.left_hand_cached.is_empty() {
                let prev_left = self.left_hand_cached.clone();
                log::debug!("considering re-requipping LEFT; {}", prev_left);
                if prev_left == unarmed.form_string() {
                    unequipSlot(Action::Left);
                    self.update_slot(HudElement::Left, &unarmed);
                } else {
                    let item = self.cache.get(&prev_left);
                    self.update_slot(HudElement::Left, &item);
                    cxx::let_cxx_string!(form_spec = prev_left.clone());
                    reequipHand(Action::Left, &form_spec);
                }
            } else if let Some(left_next) = self.cycles.get_top(&CycleSlot::Left) {
                let item = self.cache.get(&left_next);
                self.left_hand_cached = left_next.clone();
                self.update_slot(HudElement::Left, &item);
                cxx::let_cxx_string!(form_spec = left_next);
                reequipHand(Action::Left, &form_spec);
            }
        }

        if left {
            if !self.right_hand_cached.is_empty() {
                let prev_right = self.right_hand_cached.clone();
                log::debug!("considering re-requipping RIGHT; {prev_right}");
                if prev_right == unarmed.form_string() {
                    unequipSlot(Action::Right);
                    self.update_slot(HudElement::Right, &unarmed);
                } else {
                    let item = self.cache.get(&prev_right);
                    self.update_slot(HudElement::Right, &item);
                    cxx::let_cxx_string!(form_spec = prev_right);
                    reequipHand(Action::Right, &form_spec);
                }
            } else if let Some(right_next) = self.cycles.get_top(&CycleSlot::Right) {
                self.right_hand_cached = right_next.clone();
                let item = self.cache.get(&right_next);
                cxx::let_cxx_string!(form_spec = right_next);
                reequipHand(Action::Right, &form_spec);
                self.update_slot(HudElement::Right, &item);
            }
        }

        self.two_hander_equipped = item.two_handed();
        left_unexpected || right_unexpected
    }

    /// Get the item equipped in a specific slot.
    /// Called by the HUD rendering loop in the ImGui code.
    pub fn entry_to_show_in_slot(&self, slot: HudElement) -> Box<HudItem> {
        let Some(candidate) = self.visible.get(&slot) else {
            return Box::<HudItem>::default();
        };

        Box::new(candidate.clone())
    }

    /// Call when loading or otherwise needing to reinitialize the HUD.
    ///
    /// Updates will only happen here if the player changed equipment
    /// out of band, e.g., by using a menu, and only then if we screwed
    /// up an equip event.
    pub fn update_hud(&mut self) -> bool {
        let right_spec = specEquippedRight();
        let right_entry = self.cache.get(&right_spec);

        let right_changed = self.update_slot(HudElement::Right, &right_entry);
        if !right_entry.two_handed() {
            self.right_hand_cached = right_entry.form_string();
        }

        let left_spec = specEquippedLeft();
        let left_entry = self.cache.get(&left_spec);

        let left_unexpected = if !left_entry.two_handed() {
            self.left_hand_cached = left_entry.form_string();
            self.update_slot(HudElement::Left, &left_entry)
        } else {
            // it's a two-handed item in the left hand.
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
        let ammo_changed = self.update_slot(HudElement::Ammo, &ammo);

        let utility_changed = if self.visible.get(&HudElement::Utility).is_none() {
            if let Some(utility) = self.cycles.get_top(&CycleSlot::Utility) {
                let item = self.cache.get(&utility);
                self.update_slot(HudElement::Utility, &item)
            } else {
                false
            }
        } else {
            false
        };

        let changed =
            right_changed || left_unexpected || power_changed || ammo_changed || utility_changed;

        if changed {
            log::info!(
                "HUD updated. Now showing: power='{}'; left='{}'; right='{}'; ammo='{}';",
                power.name(),
                left_entry.name(),
                right_entry.name(),
                ammo.name(),
            );

            // If any of our equipped items is in a cycle, make that item the top item
            // so advancing the cycles works as expected.
            self.cycles.set_top(&CycleSlot::Power, &power.form_string());
            self.cycles
                .set_top(&CycleSlot::Left, &left_entry.form_string());
            self.cycles
                .set_top(&CycleSlot::Right, &right_entry.form_string());
        }

        changed
    }

    /// Update the displayed slot for the specified HUD element.
    fn update_slot(&mut self, slot: HudElement, new_item: &HudItem) -> bool {
        log::debug!("updating hud slot {slot}; visible: {new_item}");
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
        item: HudItem,
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
                if self.cycles.add_item(CycleSlot::Utility, &item) {
                    Some(format!("{} added to utilities cycle.", item.name()))
                } else {
                    None
                }
            } else if item.kind().is_power() {
                if self.cycles.add_item(CycleSlot::Power, &item) {
                    Some(format!("{} added to powers cycle.", item.name()))
                } else {
                    None
                }
            } else if item.two_handed() || matches!(item.kind(), BaseType::Scroll(_)) {
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
        } else {
            log::info!("Favoriting or unfavoriting didn't change cycles.");
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
        log::info!("{}; kind={:?};", message, item.kind());
        cxx::let_cxx_string!(msg = message);
        notifyPlayer(&msg);
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

impl Default for Controller {
    fn default() -> Self {
        Controller::new()
    }
}
