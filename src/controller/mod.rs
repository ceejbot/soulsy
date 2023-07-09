pub mod cycles;
pub mod controller;
pub mod settings;

pub use settings::*;
pub use controller::*;

pub fn handle_key_event(key: u32) -> bool {
    let action = Action::from(key);
    let controller = controller();
    controller.handle_action(action)
}
