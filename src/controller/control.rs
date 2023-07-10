use std::sync::Mutex;

use once_cell::sync::Lazy;

use super::cycles::*;
use super::settings::user_settings;
use crate::plugin::{ui_renderer, Action, ButtonEvent, KeyEventResponse, MenuEventResponse};

/// There can be only one. Not public because we want access managed.
static CONTROLLER: Lazy<Mutex<Controller>> = Lazy::new(|| Mutex::new(Controller::new()));

pub fn handle_key_event(key: u32, button: &ButtonEvent) -> KeyEventResponse {
    let action = Action::from(key);
    CONTROLLER.lock().unwrap().handle_key_event(action)
}

pub fn handle_menu_event(key: u32, item: &CycleEntry) -> MenuEventResponse {
    let action = Action::from(key);
    CONTROLLER.lock().unwrap().toggle_item(action, item.clone())
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
        } else {
            Action::Irrelevant
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct Controller {
    /// Our currently-active cycles.
    cycles: CycleData,
}

impl Controller {
    pub fn new() -> Self {
        let cycles = CycleData::read().unwrap_or_default();
        Controller {
            cycles
        }
    }

    pub fn handle_key_event(&mut self, action: Action) -> KeyEventResponse {
        // Sketching out what has to happen on fired timers
        // timer data should include the triggering action so we know what to do
        // de-highlight the button if necessary
        // if utility slot, nothing further to do
        // if left/right/power, equip the item

        // If we're faded out in any way, show ourselves again.
        // The second param to set_fade() is the desired end alpha.
        if !matches!(action, Action::ShowHide) {
            let is_fading: bool = ui_renderer::get_fade();
            if user_settings().fade() && !is_fading {
                ui_renderer::set_fade(true, 1.0);
                return KeyEventResponse {
                    handled: true,
                    ..Default::default()
                };
            }
        }

        // will clippy complain about the C++ method names?
        let is_down: bool = button::IsDown();
        let is_up: bool = button::IsUp();

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
                KeyEventResponse {
                    handled: true,
                    start_timer: Action::Left,
                    stop_timer: Action::Irrelevant,
                }
            }
            Action::Right => {
                // cycle the left selection one forward; start the left timer
                // highlight the button
                KeyEventResponse {
                    handled: true,
                    start_timer: Action::Right,
                    stop_timer: Action::Irrelevant,
                }
            }
            Action::Utility => {
                // cycle the left selection one forward; start the left timer
                // highlight the button
                KeyEventResponse {
                    handled: true,
                    start_timer: Action::Utility,
                    stop_timer: Action::Irrelevant,
                }
            }
            Action::Activate => {
                // stop any timers for the utility cycle
                // finalize the UI look (de-highlight)
                // use the item
                // TODO
                KeyEventResponse {
                    handled: true,
                    start_timer: Action::Irrelevant,
                    stop_timer: Action::Utility,
                }
            }
            Action::ShowHide => {
                // ask if we're visible now
                // set val=0.0 if we are, 1.0 if we're not
                // call set_fade(true, val)
                ui_renderer::toggle_show_ui();
                KeyEventResponse {
                    handled: true,
                    ..Default::default()
                }
            }
            Action::Irrelevant => KeyEventResponse::default(),
        }
    }

    /// This function is called when the user has pressed a hot key while hovering over an
    /// item in a menu. We'll remove the item if it's already in the matching cycle,
    /// or add it if it's an appropriate item. We signal back to the UI layer what we did.
    pub fn toggle_item(&mut self, action: Action, item: CycleEntry) -> MenuEventResponse {
        let result = self.cycles.toggle(action, item);
        if matches!(
            result,
            MenuEventResponse::ItemAdded | MenuEventResponse::ItemRemoved
        ) {
            // the data changed. flush it to disk with char name in it or something
            match self.cycles.write() {
                Ok(_) => todo!(),
                Err(_) => todo!(),
            }
        }

        result
    }

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
