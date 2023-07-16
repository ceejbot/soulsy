#![deny(future_incompatible)]
#![warn(rust_2018_idioms, trivial_casts)]

pub mod controller;
use controller::*;

/// Rust defines the bridge between it and C++ in this mod, using the
/// affordances of the `cxx` crate. At build time `cxx_build` will generate the
/// header files required by the C++ side. The macros expand in-line to generate
/// the matching Rust code.
#[cxx::bridge]
pub mod plugin {

    /// Hud elements to draw.
    #[derive(Deserialize, Serialize, Debug, Clone, Hash)]
    enum HudElement {
        Power,
        Utility,
        Left,
        Right,
        Ammo,
    }

    /// Where to arrange the HUD elements and what color to draw them in.
    ///
    /// This data is serialized to the SoulsyHUD_HudLayout.toml file.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    struct HudLayout {
        /// Where to draw the HUD; an offset from the top left corner.
        anchor: Point,
        /// The dimensions of a bounding box for the HUD.
        size: Point,
        /// The color to draw the HUD bg image with; if zero will not be drawn.
        bg_color: Color,
        /// One slot layout for each element. This wants to be map, not a vec,
        /// but the map types are not shareable.
        layouts: Vec<SlotLayout>,
        /// How intense the slot-change animation is.
        animation_alpha: u8,
        /// How long the slot-change animation runs.
        animation_duration: f32,
        /// The font file to load to use for all text.
        font: String,
        /// The font size for most things.
        font_size: f32,

        /// Enable debug logging for the plugin.
        debug: bool,
    }

    /// An x,y coordinate used to indicate size or an offset.
    #[derive(Deserialize, Serialize, Debug, Clone, Default)]
    struct Point {
        /// Width or side-to-side offset. Negative values move left.
        x: f32,
        /// Height or top-to-bottom offset. Negative values move up.
        y: f32,
    }

    /// Color as rgba between 0 and 255. The default is white at full alpha.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    struct Color {
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    }

    /// Layout variables for a single HUD slot, e.g, the power slot.
    ///
    /// This has all the same data as the previous slot settings struct, but rearranges
    /// it into more subtypes. The current logic uses the alpha level of an item to decide
    /// if it should be drawn or not. I might go for an explict boolean for that.
    /// TODO: make this more concise!
    #[derive(Deserialize, Serialize, Debug, Clone)]
    struct SlotLayout {
        /// The hud element this layout is for.
        element: HudElement,
        /// The name of the hud element this layout is for. For humans.
        name: String,
        /// An offset from the overall hud anchor point to draw this element at.
        offset: Point,
        /// The size of this element, to scale everything to. Partially TODO.
        size: Point,
        /// The color of any background for this element. If its alpha is 0, the bg is not drawn.
        bg_color: Color,
        /// The color of any icon for this element. If its alpha is 0, the icon is not drawn.
        icon_color: Color,
        /// The size of the icon to draw in this slot.
        icon_size: Point,

        /// The color to use for this element's hotkey, if it has one. If alpha is zero, it's not drawn.
        hotkey_color: Color,
        /// Where to draw this hotkey, relative to the anchor point.
        hotkey_offset: Point,
        /// Scale for any hotkey icon.
        hotkey_size: Point,
        /// The color to use to draw the key. Not drawn if the alpha is zero.
        hotkey_bg_color: Color,

        /// If text is drawn in this element, where to draw it.
        count_offset: Point,
        /// If this element has to show a count, the font size to use.
        count_font_size: f32,
        /// The color of any count size text; 0 alpha means not to draw it at all.
        count_color: Color,

        /// The color of any item name text; 0 alpha means not to draw it at all.
        name_color: Color,
        /// Where to draw the item name.
        name_offset: Point,
    }

    /// The type of an item stored in a cycle.
    ///
    /// This lets us determine the icon as well as which cycle slot an item can
    /// be added to.
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
    enum EntryKind {
        Empty,
        Alteration,
        ArmorClothing,
        ArmorHeavy,
        ArmorLight,
        Arrow,
        AxeOneHanded,
        AxeTwoHanded,
        Bow,
        Claw,
        Conjuration,
        Crossbow,
        Dagger,
        DestructionFire,
        DestructionFrost,
        DestructionShock,
        Destruction,
        Food,
        Halberd,
        HandToHand,
        IconDefault,
        Illusion,
        Katana,
        Lantern,
        Mace,
        Mask,
        Pike,
        PoisonDefault,
        PotionDefault,
        PotionFireResist,
        PotionFrostResist,
        PotionHealth,
        PotionMagicka,
        PotionMagicResist,
        PotionShockResist,
        PotionStamina,
        Power,
        QuarterStaff,
        Rapier,
        Restoration,
        Scroll,
        Shield,
        Shout,
        SpellDefault,
        Staff,
        SwordOneHanded,
        SwordTwoHanded,
        Torch,
        WeaponDefault,
        Whip,
    }

    /// Turning the key number into an enum is handy.
    #[derive(Debug, Clone, Hash)]
    enum Action {
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
        /// Refresh the layout by re-reading the toml file.
        RefreshLayout,
    }

    /// I would rather not use exceptions for normal flow control.
    #[derive(Debug, Clone, PartialEq, Eq)]
    enum MenuEventResponse {
        Okay,
        Unhandled,
        Error,
        ItemAdded,
        ItemRemoved,
        ItemInappropriate,
        TooManyItems,
        // other responses/errors?
    }

    /// What Rust did with a key event, so the C++ caller can present UI.
    ///
    /// This struct passes data from controller to C++ to signal if it should
    /// start or stop a timer, and if so which timer. For complicated reasons,
    /// timers on the Rust side are impractical (they need to be async) and so I
    /// am doing them on the  C++ side. A better solution would be nice.
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct KeyEventResponse {
        /// Did we handle this keypress?
        handled: bool,
        /// Do we need to start a timer?
        start_timer: Action,
        /// Do we need to stop a timer?
        stop_timer: Action,
    }

    extern "Rust" {
        /// Trigger the rust side to start logging.
        fn initialize_rust_logging(logdir: &CxxString);
        /// Trigger rust to read config, figure out what the player has equipped,
        /// and figure out what it should draw.
        fn initialize_hud();

        /// Give access to the settings to the C++ side.
        type UserSettings;
        fn equip_delay(self: &UserSettings) -> u32;
        fn fade_delay(self: &UserSettings) -> u32;
        fn fade(self: &UserSettings) -> bool;
        fn is_cycle_button(self: &UserSettings, key: u32) -> bool;
        fn maxlen(self: &UserSettings) -> u32;
        fn hotkey_for(self: &UserSettings, action: HudElement) -> u32;
        fn controller_kind(self: &UserSettings) -> u32;

        /// Managed access to the settings object, so we can lazy-load if necessary.
        fn user_settings() -> Box<UserSettings>;
        /// After an MCM-managed change, re-read our .ini file.
        fn refresh_user_settings();
        /// Fetch a read-only copy of our current layout;
        fn layout() -> HudLayout;

        /// This is an entry in the cycle. The UI will ask questions of it.
        type TesItemData;
        fn kind(self: &TesItemData) -> EntryKind;
        fn highlighted(self: &TesItemData) -> bool;
        fn name(self: &TesItemData) -> String;
        fn has_count(self: &TesItemData) -> bool;
        fn count(self: &TesItemData) -> usize;
        /// Call to create a brand-new cycle entry, with a cache of game data we'll need
        /// to draw and use this item quickly.
        fn create_tesitem_shim(
            kind: EntryKind,
            two_handed: bool,
            has_count: bool,
            count: usize,
            name: &str,
            form_string: &str,
        ) -> Box<TesItemData>;
        /// Snag a default cycle entry.
        fn default_cycle_entry() -> Box<TesItemData>;

        /// Get the svg icon matching this item. Not a full path.
        fn get_icon_file(kind: &EntryKind) -> String;

        /// Handle an incoming key press event from the game. An enum encoding how it was handled.
        fn handle_key_event(key: u32, button: &ButtonEvent) -> KeyEventResponse;
        /// Handle an in-menu event (which adds/removes items) from the game.
        fn handle_menu_event(key: u32, item: Box<TesItemData>) -> MenuEventResponse;
        /// Get the item readied in the given slot, if any.
        fn entry_to_show_in_slot(slot: HudElement) -> Box<TesItemData>;
        /// A cycle delay timer has expired. Time to equip!
        fn timer_expired(slot: Action);
        /// Update the HUD without any hints about what just changed.
        fn update_equipped() -> bool;
        /// Update the HUD with info about an item the player just equipped.
        fn handle_item_equipped(item: Box<TesItemData>) -> bool;
        /// The player's inventory changed. Update if necessary.
        fn handle_inventory_changed(item: Box<TesItemData>, count: usize);
    }

    unsafe extern "C++" {
        // everything in the RE namespace is from CommonLibSE
        // I can imagine auto-generating a complete bridge at some point.
        include!("PCH.h");

        /// The form object: the source of all data! We expose selected methods.
        #[namespace = "RE"]
        type TESForm;
        #[namespace = "RE"]
        fn GetFormID(self: &TESForm) -> u32;
        
        /// The equip slot for an item. Imported from CommonLibSE.
        #[namespace = "RE"]
        type BGSEquipSlot;
        /// A keyboard, mouse, or gamepad button event. Imported from CommonLibSE.
        #[namespace = "RE"]
        type ButtonEvent;
        /// Exposes to Rust the is-down method on the button event object.
        #[namespace = "RE"]
        fn IsDown(self: &ButtonEvent) -> bool;
        /// Exposes to Rust the is-up method on the button event object.
        #[namespace = "RE"]
        fn IsUp(self: &ButtonEvent) -> bool;
    }

    // Selected helpers.
    #[namespace = "helpers"]
    unsafe extern "C++" {
        include!("helpers.h");

        /// Display a debug notification on the screen. Used as hacky action confirmation.
        fn notify_player(message: &CxxString);
        /// Start the HUD widget fading in or out to the goal transparency.
        fn set_alpha_transition(do_fade: bool, alpha: f32);
        /// Check if the HUD widget is in the middle of a fade in or out.
        fn get_is_transitioning() -> bool;
        /// Show or hide the HUD widget.
        fn toggle_hud_visibility();
        /// Show the hude no matter what;
        fn show_hud();
    }

    // Selected player data fetchers.
    #[namespace = "player"]
    unsafe extern "C++" {
        include!("player.h");

        fn equippedLeftHand() -> Box<TesItemData>;
        fn equippedRightHand() -> Box<TesItemData>;
        fn equippedPower() -> Box<TesItemData>;
        fn equippedAmmo() -> Box<TesItemData>;

        fn unequipSlot(which: Action);

        fn equipShout(form_spec: &CxxString);
        fn equipMagic(form_spec: &CxxString, which: Action, kind: EntryKind);
        fn equipWeapon(form_spec: &CxxString, which: Action, kind: EntryKind);
        fn equipArmor(form_spec: &CxxString);
        fn equipAmmo(form_spec: &CxxString);
        fn consumePotion(form_spec: &CxxString);
    }
}
