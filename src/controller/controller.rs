use once_cell::sync::OnceCell;

use super::settings::settings;
use super::cycles::*;

/// There can be only one. Not public because we want access managed.
static CONTROLLER: OnceCell<Controller> = OnceCell::new();

pub fn controller() -> &'static Controller {
    if CONTROLLER.get().is_none() {
        let c = Controller::new();
        CONTROLLER.set(c).unwrap();
    }

    CONTROLLER.get().unwrap()
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
    Irrelevant
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

#[derive(Debug, Clone, Default)]
pub struct Controller {
    // timers
    // any other runtime state
    cycles: CycleData
}

impl Controller {

    pub fn new() -> Self {
        Controller {
            cycles: CycleData::default() // for now; will need to come from mcm data or from a file
        }
    }

    fn timer_elapsed() {
        // Sketching out what has to happen here
        // timer data should include the triggering action so we know what to do
        // de-highlight the button if necessary
        // if utility slot, nothing further to do
        // if left/right/power, equip the item
    }

    pub fn handle_action(&self, action: Action) -> bool {
        match action {
            Action::Power => {
                // cycle the power selection one forward; start the power timer
                // highlight the button
                todo!()
            },
            Action::Left => {
                // cycle the left selection one forward; start the left timer
                // highlight the button
                todo!()
            },
            Action::Right => {
                // cycle the left selection one forward; start the left timer
                // highlight the button
                todo!()
            },
            Action::Utility => {
                // cycle the left selection one forward; start the left timer
                // highlight the button
                todo!()
            },
            Action::Activate => {
                // stop any timers for the utility cycle
                // finalize the UI look (de-highlight)
                // use the item
                todo!()
            },
            Action::ShowHide => {
                // cycle the left selection one forward; start the left timer
                // highlight the button
                todo!()
            },
            Action::Irrelevant => false,
        }
    }

    pub fn toggle_item(&self, action: Action, item: String) -> bool {
        // string is almost certainly NOT it.

        // select the correct list.
        // remove from list if present in list; return false
        // apply filter logic to make sure item is equippable in that slot
        // if not okay, return false
        // if okay, add item to list behind current position, return true
    
        // might want to save HudState on any change tbh
        match action {
            Action::Power => self.cycles.toggle_power(item),
            Action::Left => self.cycles.toggle_left(item),
            Action::Right => self.cycles.toggle_right(item),
            Action::Utility => self.cycles.toggle_utility(item),
            _ => return false,
        }
    }

    pub fn on_equip_change(&self) {
        // this should be called by a top-level hook that also makes sure UI is updated
        // if item is any lists: rotate list so item is at front
        // else do nothing
        // how do we notice if an item has left our inventory?
        todo!();
    }

}
