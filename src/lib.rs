pub mod controller;
use controller::*;

/// Rust defines the bridge between it and C++ in this file, using the
/// affordances of the `cxx` crate. At build time `cxx_build` will generate the
/// header files required by the C++ side.
#[cxx::bridge]
pub mod plugin {
    /// This voluminous struct exposes its fields because the UI implementation
    /// frequently refers to them. It is read-only for the C++ side. The values
    /// are filled out by lazily reading the layout toml file.
    #[derive(Deserialize, Serialize, Debug, Clone, Default)]
    pub struct HudLayout {
        /// Enable debug logging.
        debug: bool,
    }

    /// Turning the key number into an enum is handy.
    #[derive(Debug, Clone)]
    pub enum Action {
        /// We do not need to do anything, possibly because the key was not one of our hotkeys.
        Irrelevant,
        /// We're acting on the power/shouts hotkey.
        Power,
        /// The left-hand cycle hotkey.
        Left,
        /// The right-hand cycle hotkey.
        Right,
        /// The utility-item cycle hotkey.
        Utility,
        /// The activate-utility-item hotkey.
        Activate,
        /// The HUD toggle hotkey.
        ShowHide,
    }

    /// I would rather not use exceptions for normal flow control.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum MenuEventResponse {
        Okay,
        Unhandled,
        Error,
        ItemAdded,
        ItemRemoved,
        ItemInappropriate,
        TooManyItems,
        // other responses/errors?
    }

    /// This struct passes data from controller to C++ to signal if it should
    /// start or stop a timer, and if so which timer. For complicated reasons,
    /// timers on the Rust side are impractical (they need to be async) and so I
    /// am doing them on the  C++ side. A better solution would be nice.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct KeyEventResponse {
        /// Did we handle this keypress?
        handled: bool,
        /// Do we need to start a timer?
        start_timer: Action,
        /// Do we need to stop a timer?
        stop_timer: Action,
    }

    extern "Rust" {
        // Zero or more opaque types which both languages can pass around
        // but only Rust can see the fields.

        /// Give access to the settings to the C++ side.
        type UserSettings;
        /// This is an entry in the cycle. The UI will ask questions of it.
        type CycleEntry;

        /// Managed access to the layout object, so we can lazy-load if necessary.
        fn layout() -> &'static HudLayout;
        /// Managed access to the settings object, so we can lazy-load if necessary.
        fn user_settings() -> Box<UserSettings>;
        /// After an MCM-managed change, re-read our .ini file.
        fn refresh_user_settings() -> Result<()>;
        /// Handle an incoming key press event from the game. Returns true if handled.
        fn handle_key_event(key: u32, button: &ButtonEvent) -> KeyEventResponse;
        /// Handle an in-menu event (which adds/removes items) from the game.
        fn handle_menu_event(key: u32, item: &TESForm) -> MenuEventResponse;
    }

    unsafe extern "C++" {
        /// Advertise to CXX what our C++ is supposed to look like.
        include!("PCH.h");
        include!("HeadersForRust.hxx"); // we don't want this handed to CMake, so we name it oddly.

        // everything in the RE namespace is from CommonLibSE

        #[namespace = "RE"]
        type TESForm;
        #[namespace = "RE"]
        type BGSEquipSlot;
        #[namespace = "RE"]
        type ButtonEvent;

        // the UI renderer
        #[namespace = "ui"]
        type ui_renderer;

        #[namespace = "helpers"]
        type slot_type; // this is an enum; TODO make it shared

        #[namespace = "helpers"]
        fn is_two_handed(item: &TESForm) -> bool;

    }
}
