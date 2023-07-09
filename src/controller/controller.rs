use std::sync::Mutex;
use std::time::Duration;

use once_cell::sync::Lazy; // 1.3.1

use super::cycles::*;
use super::settings::settings;

/// There can be only one. Not public because we want access managed.
static CONTROLLER: Lazy<Mutex<Controller>> = Lazy::new(|| Mutex::new(Controller::new()));

pub fn handle_key_event(key: u32) -> bool {
    let action = Action::from(key);
    CONTROLLER.lock().unwrap().handle_action(action)
}

/// Turning the key number into an enum is handy.
#[derive(Debug, Clone)]
pub enum Action {
    Power,
    Left,
    Right,
    Utility,
    Activate,
    ShowHide,
    Irrelevant,
}

impl From<u32> for Action {
    /// Turn the key code into an enum for easier processing.
    fn from(value: u32) -> Self {
        let settings = settings();

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
    /// A timer for the power button. When this fires, the item equips.
    _power_timer: Option<()>,
    /// Repeat comment.
    _left_timer: Option<()>,
    /// Repeat comment.
    _right_timer: Option<()>,
    /// Yes, this unrolling thing is tedious. But the cost of abstraction doesn't yet feel worth it.
    _utility_timer: Option<()>,
    /// Storing this on the struct in case it becomes a user setting.
    _delay_ms: Duration,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            cycles: CycleData::default(), // for now; will need to come from mcm data or from a file
            _power_timer: None,
            _left_timer: None,
            _right_timer: None,
            _utility_timer: None,
            _delay_ms: Duration::from_micros(1000),
        }
    }

    pub fn handle_action(&mut self, action: Action) -> bool {
        // Sketching out what has to happen on fired timers
        // timer data should include the triggering action so we know what to do
        // de-highlight the button if necessary
        // if utility slot, nothing further to do
        // if left/right/power, equip the item

        match action {
            Action::Power => {
                let _next = self.cycles.advance(action, 1);
                // tell the ui to show this and highlight this button
                // start or restart the relevant timer
                /*
                if let Some(guard) = self.power_timer {
                    drop(guard);
                }
                let t = Timer::new();
                let guard = t.schedule_with_delay(self.delay_ms, || {
                    self.power_timer = None;
                });
                self.power_timer = Some(guard);
                */
                todo!()
            }
            Action::Left => {
                // cycle the left selection one forward; start the left timer
                // highlight the button
                todo!()
            }
            Action::Right => {
                // cycle the left selection one forward; start the left timer
                // highlight the button
                todo!()
            }
            Action::Utility => {
                // cycle the left selection one forward; start the left timer
                // highlight the button
                todo!()
            }
            Action::Activate => {
                // stop any timers for the utility cycle
                // finalize the UI look (de-highlight)
                // use the item
                todo!()
            }
            Action::ShowHide => {
                // do what it says plox
                todo!()
            }
            Action::Irrelevant => false,
        }
    }

    /// This function is called when the user has pressed a hot key while hovering over an
    /// item in a menu. We'll remove the item if it's already in the matching cycle,
    /// or add it if it's an appropriate item. We signal back to the UI layer what we did.
    pub fn toggle_item(&mut self, action: Action, item: CycleEntry) -> ToggleResults {
        // The trick here is making the CycleEntry item out of UI data in the first place.
        // I need to expose form data and other item data to Rust.
        let result = self.cycles.toggle(action, item);
        if matches!(result, ToggleResults::Added) || matches!(result, ToggleResults::Removed) {
            // the data changed. might want to flush it to disk?
            // or do something depending on how I end up persisting this
        }

        result
    }

    pub fn on_equip_change(&self) {
        // this should be called by a top-level hook that also makes sure UI is updated
        // if item is any lists: rotate list so item is at front
        // else do nothing
        // how do we notice if an item has left our inventory? event registration probably
        todo!();
    }
}
