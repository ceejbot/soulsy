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
    // ceejbot says: organize into namespaces; getting pretty cluttered

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
    #[derive(Deserialize, Serialize, Debug, Clone)]
    struct SlotLayout {
        /// The hud element this layout is for.
        element: HudElement,
        /// The name of the hud element this layout is for. For humans.
        name: String,
        /// An offset from the overall hud anchor point to draw this element at.
        offset: Point,
        /// The size of this element, to scale everything to.
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
    enum TesItemKind {
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

    /// This enum maps key presses to the desired action. More like a C/java
    /// enum than a Rust sum type enum.
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

    /// What Rust did with a key event, so the C++ caller can present UI.
    ///
    /// This struct passes data from controller to C++ to signal if it should
    /// start or stop a timer, and if so which timer. For complicated reasons,
    /// timers on the Rust side are impractical (they need to be async) and so I
    /// am doing them on the  C++ side.
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
        /// Tell the rust side where to log.
        fn initialize_rust_logging(logdir: &CxxString);
        /// Trigger rust to read config, figure out what the player has equipped,
        /// and figure out what it should draw.
        fn initialize_hud();

        /// Give access to the settings to the C++ side.
        type UserSettings;
        /// Get the user setting for the equip delay timer, in milliseconds.
        fn equip_delay(self: &UserSettings) -> u32;
        /// Get the user setting for the fade-out delay timer, in milliseconds.
        fn fade_delay(self: &UserSettings) -> u32;
        /// Get whether the HUD should fade out when not in combat.
        fn fade(self: &UserSettings) -> bool;
        /// Check if this button is relevant to the HUD.
        fn is_cycle_button(self: &UserSettings, key: u32) -> bool;
        /// Get how long a cycle is allow to be.
        fn maxlen(self: &UserSettings) -> u32;
        /// Get the hotkey for a specific action.
        fn hotkey_for(self: &UserSettings, action: HudElement) -> u32;
        /// Get which kind of controller to draw shortcuts for: keyboard, PS5, or Xbox.
        fn controller_kind(self: &UserSettings) -> u32;
        /// If a settings change has shortened the max cycle length, truncate if we have to.
        fn truncate_cycles(new_length: u32);

        /// Managed access to the settings object, so we can lazy-load if necessary.
        fn user_settings() -> Box<UserSettings>;
        /// After an MCM-managed change, re-read our .ini file.
        fn refresh_user_settings();
        /// Fetch a read-only copy of our current layout.
        fn hud_layout() -> HudLayout;

        /// This is an entry in the cycle. The UI will ask questions of it.
        type TesItemData;
        /// The item category, fine-grained to help with icon choices.
        fn kind(self: &TesItemData) -> TesItemKind;
        /// True if any UI for this item should be drawn highlight. UNUSED.
        fn highlighted(self: &TesItemData) -> bool;
        /// The game's name for this item.
        fn name(self: &TesItemData) -> String;
        /// Whether this item has a relevant count.
        fn has_count(self: &TesItemData) -> bool;
        /// How many of this item the player has last time we checked. Updated on inventory changes.
        fn count(self: &TesItemData) -> u32;
        /// Call to create a brand-new cycle entry, with a cache of game data we'll need
        /// to draw and use this item quickly.
        fn make_tesitem(
            kind: TesItemKind,
            two_handed: bool,
            has_count: bool,
            count: u32,
            name: &str,
            form_string: &str,
        ) -> Box<TesItemData>;
        /// Make a default item, representing an empty choice.
        fn default_tes_item() -> Box<TesItemData>;

        /// Check if this item category can be stacked in inventory.
        fn kind_has_count(kind: TesItemKind) -> bool;
        /// Check if this item category counts as magic for the HUD.
        fn kind_is_magic(kind: TesItemKind) -> bool;
        /// Get the filename of the svg icon matching this item. Not a full path.
        fn get_icon_file(kind: &TesItemKind) -> String;

        // These are called by plugin hooks and sinks.

        /// Handle an incoming key press event from the game. An enum encoding how it was handled.
        fn handle_key_event(key: u32, button: &ButtonEvent) -> KeyEventResponse;
        /// Handle an in-menu event (which adds/removes items) from the game.
        fn handle_menu_event(key: u32, item: Box<TesItemData>);
        /// Get the item readied in the given slot, if any.
        fn entry_to_show_in_slot(slot: HudElement) -> Box<TesItemData>;
        /// A cycle delay timer has expired. Time to equip!
        fn timer_expired(slot: Action);
        /// Update the entire HUD without any hints about what just changed.
        fn update_hud() -> bool;
        /// Handle equipment-changed events from the game.
        fn handle_item_equipped(equipped: bool, item: Box<TesItemData>) -> bool;
        /// The player's inventory changed. Update if necessary.
        fn handle_inventory_changed(item: Box<TesItemData>, count: i32);
    }

    #[namespace = "RE"]
    unsafe extern "C++" {
        // everything in the RE namespace is from CommonLibSE
        // I can imagine auto-generating a near-complete bridge at some point.
        // This cannot be done using cxx because of all the char*.
        include!("PCH.h");

        /// The form object: the source of all data! We expose selected methods.
        type TESForm;
        /// Get the id for this form.
        fn GetFormID(self: &TESForm) -> u32;

        /// The equip slot for an item.
        type BGSEquipSlot;
        /// A keyboard, mouse, or gamepad button event. Imported from CommonLibSE.
        type ButtonEvent;
        /// Exposes to Rust the is-down method on the button event object.
        fn IsDown(self: &ButtonEvent) -> bool;
        /// Exposes to Rust the is-up method on the button event object.
        fn IsUp(self: &ButtonEvent) -> bool;
    }

    // Selected helpers.
    #[namespace = "helpers"]
    unsafe extern "C++" {
        include!("helpers.h");

        /// Display a debug notification on the screen. Used as hacky action confirmation.
        fn notifyPlayer(message: &CxxString);
        /// Start the HUD widget fading in or out to the goal transparency.
        fn fadeToAlpha(do_fade: bool, alpha: f32);
        /// Check if the HUD widget is in the middle of a fade in or out.
        fn getIsFading() -> bool;
        /// Show or hide the HUD widget.
        fn toggleHUD();
        /// Show the hud no matter what.
        fn showHUD();
    }

    // Selected player data fetchers.
    #[namespace = "player"]
    unsafe extern "C++" {
        include!("player.h");

        /// Get the player's name.
        fn playerName() -> String;

        /// Get the parent form item for the object equipped in the left hand.
        fn equippedLeftHand() -> Box<TesItemData>;
        /// Get the bound object (not the parent!) for the object equipped in the left hand.
        fn boundObjectLeftHand() -> Box<TesItemData>;
        /// Get the parent form item for the object equipped in the right hand.
        fn equippedRightHand() -> Box<TesItemData>;
        /// Get the bound object (not the parent!) for the object equipped in the right hand.
        fn boundObjectRightHand() -> Box<TesItemData>;
        /// Get the form for the equipped shout or power.
        fn equippedPower() -> Box<TesItemData>;
        /// Get the form for the equipped ammo.
        fn equippedAmmo() -> Box<TesItemData>;

        /// Check if the player still has items from this form in their inventory.
        fn hasItemOrSpell(form_spec: &CxxString) -> bool;

        /// Unequip the relevant slot.
        fn unequipSlot(which: Action);

        /// Equip the shout matching the form spec.
        fn equipShout(form_spec: &CxxString);
        /// Equip the spell matching the form spec.
        fn equipMagic(form_spec: &CxxString, which: Action);
        /// Equip the weapon matching the form spec.
        fn equipWeapon(form_spec: &CxxString, which: Action);
        /// Equip the armor matching the form spec.
        fn equipArmor(form_spec: &CxxString);
        /// Equip the amoo matching the form spec.
        fn equipAmmo(form_spec: &CxxString);
        /// Consume a potion matching the form spec. Skips dynamic items (for now).
        fn consumePotion(form_spec: &CxxString);
        /// Re-equip an item in the left hand. This forces an un-equip first.
        fn reequipLeftHand(form_spec: &CxxString);
    }
}
