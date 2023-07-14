use std::sync::Mutex;
use std::collections::HashMap;

use once_cell::sync::Lazy;

use super::cycles::*;
use super::layout::layout;
use super::settings::user_settings;
use crate::plugin::*;


/// There can be only one. Not public because we want access managed. 
// Does this really need to be a mutex? I think we're single-threaded...
static CONTROLLER: Lazy<Mutex<Controller>> = Lazy::new(|| Mutex::new(Controller::new()));

/// C++ tells us when it's safe to start pulling together the data we need.
pub fn initialize_hud() {
    let _ctrl = CONTROLLER.lock().unwrap();
    let _settings = user_settings();
    let _layout = layout();

    // here we should validate all four cycle entries which might refer to now-missing items
    // player::has_item_or_spell(form) is the function to call

    // now walk through what we should be showing in each slot, whether in the cycle or not
    // These functions are mostly implemented:
	// rust::Box<CycleEntry> equipped_left_hand();
	// rust::Box<CycleEntry> equipped_right_hand();
	// rust::Box<CycleEntry> equipped_power();
	// rust::Box<CycleEntry> equipped_ammo();
    
    // The readied utility item is purely in our control, so we can use whatever we have 
    // top-of-cycle for that one.
}

/// Function for C++ to call to send a relevant button event to us.
pub fn handle_key_event(key: u32, button: &ButtonEvent) -> KeyEventResponse {
    let action = Action::from(key);
    CONTROLLER.lock().unwrap().handle_key_event(action, button)
}

/// Function for C++ to call to send a relevant menu button-event to us.
///
/// We get a fully-filled out CycleEntry struct to use as we see fit.
pub fn handle_menu_event(key: u32, menu_item: Box<CycleEntry>) -> MenuEventResponse {
    let action = Action::from(key);
    CONTROLLER.lock().unwrap().toggle_item(action, *menu_item)
}

/// Get information about the item equipped in a specific slot.
pub fn equipped_in_slot(element: HudElement) -> Box<CycleEntry> {
    let slot = match element {
        HudElement::Power => Action::Power,
        HudElement::Utility => Action::Utility,
        HudElement::Left => Action::Left,
        HudElement::Right => Action::Right,
        _ => Action::Irrelevant,
    };
    CONTROLLER.lock().unwrap().equipped_in_slot(slot)
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

/// What, model/view/controller? In my UI application? oh no
#[derive(Clone, Default, Debug)]
pub struct Controller {
    /// Our currently-active cycles.
    cycles: CycleData,
    // speculative: I think this is how we'll handle tracking equipped thingies
    equipped: HashMap<Action, CycleEntry>,
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

    // TODO refs instead of cloning
    /// Get the item equipped in a specific slot. I'd like to return an option but I can't.
    pub fn equipped_in_slot(&self, slot: Action) -> Box<CycleEntry> {
        let Some(candidate) = self.equipped.get(&slot) else {
            return Box::new(CycleEntry::default());
        };

        Box::new(candidate.clone())
    }

    /// Handle a key-press event that the event system decided we need to know about.
    ///
    /// Returns an enum indicating what we did in response, in case one of the calling
    /// layers wants to show UI or play sounds in response.
    pub fn handle_key_event(&mut self, action: Action, button: &ButtonEvent) -> KeyEventResponse {
        // Sketching out what has to happen on fired timers
        // timer data should include the triggering action so we know what to do
        // de-highlight the button if necessary
        // if utility slot, nothing further to do
        // if left/right/power, equip the item

        // If we're faded out in any way, show ourselves again.
        if !matches!(action, Action::ShowHide) {
            let is_fading: bool = get_is_transitioning();
            if user_settings().fade() && !is_fading {
                set_alpha_transition(true, 1.0);
                return KeyEventResponse {
                    handled: true,
                    ..Default::default()
                };
            }
        }

        // will clippy complain about the C++ method names?
        let _is_down: bool = button.IsDown();
        let _is_up: bool = button.IsUp();

        // TODO implement!
        match action {
            Action::Power => {
                let _next = self.cycles.advance(action, 1);
                // tell the ui to show this and highlight this button
                // start or restart the relevant timer
                KeyEventResponse {
                    handled: true,
                    start_timer: Action::Power,
                    stop_timer: Action::Irrelevant,
                }
            }
            Action::Left => {
                // cycle the left selection one forward; start the left timer
                // highlight the button
                let _next = self.cycles.advance(action, 1);
                KeyEventResponse {
                    handled: true,
                    start_timer: Action::Left,
                    stop_timer: Action::Irrelevant,
                }
            }
            Action::Right => {
                // start the ready delay timer for the right hand
                // highlight the right hud slot
                let _next = self.cycles.advance(action, 1);
                KeyEventResponse {
                    handled: true,
                    start_timer: Action::Right,
                    stop_timer: Action::Irrelevant,
                }
            }
            Action::Utility => {
                // start the ready delay timer for the utility slot
                // highlight the utility hud slot
                let _next = self.cycles.advance(action, 1);
                KeyEventResponse {
                    handled: true,
                    start_timer: Action::Utility,
                    stop_timer: Action::Irrelevant,
                }
            }
            Action::Activate => {
                // TODO
                // stop any timers for the utility slot;
                // mark the current item as the top-of-cycle
                // finalize the UI look (de-highlight)
                // use the item
                KeyEventResponse {
                    handled: true,
                    start_timer: Action::Irrelevant,
                    stop_timer: Action::Utility,
                }
            }
            Action::ShowHide => {
                // handled by the C++ side for now
                toggle_hud_visibility();
                KeyEventResponse {
                    handled: true,
                    ..Default::default()
                }
            }
            Action::RefreshLayout => {
                // TODO tell C++ to redraw
                HudLayout::refresh();
                KeyEventResponse {
                    handled: true,
                    ..Default::default()
                }
            }
            _ => KeyEventResponse::default(),
        }
    }

    /// This function is called when the user has pressed a hot key while hovering over an
    /// item in a menu. We'll remove the item if it's already in the matching cycle,
    /// or add it if it's an appropriate item. We signal back to the UI layer what we did.
    pub fn toggle_item(&mut self, action: Action, item: CycleEntry) -> MenuEventResponse {
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
                Ok(_) => log::info!("successfully wrote cycle data"),
                Err(e) => {
                    log::warn!("failed to write cycle data, but gamely continuing; {e:?}");
                }
            }
        }

        result
    }

    /// TO BE CALLED when the player's equipped items change.
    /// API surface tbd.
    pub fn on_equip_change(&self) {
        // this should be called by a top-level hook that also makes sure UI is updated
        // if item is any lists: rotate list so item is at front
        // else do nothing
        // we have another hook set up for inventory changes that will also need a handler
        todo!();
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
