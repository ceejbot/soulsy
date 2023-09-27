#![deny(future_incompatible, clippy::unwrap_used)]
#![warn(rust_2018_idioms, trivial_casts)]

pub mod controller;
use controller::*;
pub mod data;
use data::icons::icon_files;
use data::{HudItem, SpellData, *};

/// Rust defines the bridge between it and C++ in this mod, using the
/// affordances of the `cxx` crate. At build time `cxx_build` generates the
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
        None, // not drawn
    }

    /// Text alignment options
    #[derive(Debug, Clone, Hash)]
    enum Align {
        Left,
        Right,
        Center,
    }

    /// Named HUD anchor points.
    #[derive(Debug, Clone, Hash)]
    enum NamedAnchor {
        TopLeft,
        TopRight,
        BottomLeft,
        BottomRight,
        Center,
        CenterTop,    // top edge
        CenterBottom, // bottom edge
        LeftCenter,   // left edge midway down
        RightCenter,  // right edge
        None,
    }

    /// An x,y coordinate used to indicate size or an offset.
    #[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
    struct Point {
        /// Width or side-to-side offset. Negative values move left.
        x: f32,
        /// Height or top-to-bottom offset. Negative values move up.
        y: f32,
    }

    /// Color as rgba between 0 and 255. The default is white at full alpha.
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    struct Color {
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    }

    /// Where to arrange the HUD elements and what color to draw them in.
    ///
    /// This data is serialized to the SoulsyHUD_HudLayout.toml file.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    struct HudLayout {
        #[serde(default)]
        /// A global scaling factor for the entire hud.
        global_scale: f32,
        /// Where to draw the HUD; an offset from the top left corner.
        #[serde(default)]
        anchor: Point,
        #[serde(default, deserialize_with = "crate::deserialize_named_anchor")]
        anchor_name: NamedAnchor,
        /// The dimensions of a bounding box for the HUD.
        size: Point,
        /// The color to draw the HUD bg image with; if zero will not be drawn.
        bg_color: Color,
        /// Swap the ammo slot in and the left hand slot out when ranged weapon
        /// equipped.
        #[serde(default)]
        hide_ammo_when_irrelevant: bool,
        /// Hide the left hand slot when a ranged weapon is equipped.
        #[serde(default)]
        hide_left_when_irrelevant: bool,
        /// One slot layout for each element. This wants to be map, not a vec,
        /// but the map types are not shareable.
        layouts: Vec<SlotLayout>,
        #[serde(default)]
        /// How intense the slot-change animation is. Unused.
        animation_alpha: u8,
        #[serde(default)]
        /// How long the slot-change animation runs. Unused.
        animation_duration: f32,
        /// The font file to load to use for all text.
        font: String,
        /// The font size for most things.
        #[serde(default)]
        font_size: f32,
        /// Whether to buld glyphs for full Chinese text display.
        #[serde(default)]
        chinese_full_glyphs: bool,
        /// Whether to build glyphs for simplified Chinese text display.
        #[serde(default)]
        simplified_chinese_glyphs: bool,
        /// Whether to build glyphs for simplified Chinese text display.
        #[serde(default)]
        cyrillic_glyphs: bool,
        /// Whether to build glyphs for Cyrillic text display.
        #[serde(default)]
        japanese_glyphs: bool,
        /// Whether to build glyphs for Japanese text display.
        #[serde(default)]
        korean_glyphs: bool,
        /// Whether to build glyphs for Thai text display.
        #[serde(default)]
        thai_glyphs: bool,
        /// Whether to build glyphs for Vietnamese text display.
        #[serde(default)]
        vietnamese_glyphs: bool,
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
        /// How to align any text associated with this slot.
        #[serde(default, deserialize_with = "crate::deserialize_align")]
        align_text: Align,
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
        /// Where to draw the icon; a center point relative to the center of this slot.
        #[serde(default)]
        icon_offset: Point,

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
        /// The font size to use for this item's name.
        #[serde(default)]
        name_font_size: f32,
    }

    /// This enum maps key presses to the desired action. More like a C/java
    /// enum than a Rust sum type enum.
    #[derive(Debug, Clone, Hash)]
    enum Action {
        /// We do not need to do anything, possibly because the key was not one of our hotkeys.
        None,
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

    #[derive(Debug, Clone, Hash)]
    enum ItemCategory {
        Ammo,
        Armor,
        Food,
        HandToHand,
        Light,
        Potion,
        Power,
        Scroll,
        Shout,
        Spell,
        Weapon,
        Empty,
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
        fn initialize_rust_logging(logdir: &CxxVector<u16>);

        /// Trigger rust to read config, figure out what the player has equipped,
        /// and figure out what it should draw.
        fn initialize_hud();
        /// Check if the user wants the HUD visible right now or not.
        fn show_ui() -> bool;
        /// Get cycle data for cosave.
        fn serialize_cycles() -> Vec<u8>;
        /// Serialization format version.
        fn serialize_version() -> u32;
        fn cycle_loaded_from_cosave(bytes: &CxxVector<u8>, version: u32);
        /// On save load or death restore, wipe the hud item cache.
        fn clear_cache();

        /// Give access to the settings to the C++ side.
        type UserSettings;
        /// Get read-only access to our settings for use.
        fn user_settings() -> Box<UserSettings>;
        /// Get the user setting for the equip delay timer, in milliseconds.
        fn equip_delay_ms(self: &UserSettings) -> u32;
        /// Get whether the HUD should control its own visibility.
        fn autofade(self: &UserSettings) -> bool;
        /// Check if this button is relevant to the HUD.
        fn is_cycle_button(self: &UserSettings, key: u32) -> bool;
        /// Get the hotkey for a specific action.
        fn hotkey_for(self: &UserSettings, action: HudElement) -> u32;
        /// Get which kind of controller to draw shortcuts for: PS5 or Xbox.
        fn controller_kind(self: &UserSettings) -> u32;
        /// If we should enter slow motion while cycling.
        fn cycling_slows_time(self: &UserSettings) -> bool;
        /// How much to slow down time.
        fn slow_time_factor(self: &UserSettings) -> f32;
        /// How long to spend fading in or out.
        fn fade_time(self: &UserSettings) -> u32;
        /// If we care about favorites.
        fn link_to_favorites(self: &UserSettings) -> bool;
        /// If icons should be colorful.
        fn colorize_icons(self: &UserSettings) -> bool;
        /// If we should log at debug level.
        fn debug(self: &UserSettings) -> bool;
        /// The identifier to use for this mod in SKSE cosaves. Not exposed in UI.
        fn skse_identifier(self: &UserSettings) -> u32;

        /// After an MCM-managed change, re-read our .ini file.
        fn refresh_user_settings();
        /// Fetch a read-only copy of our current layout.
        fn hud_layout() -> HudLayout;
        /// Get the hud's anchor point, computed on the fly from the
        /// current screen size and layout data.
        fn anchor_point(self: &HudLayout) -> Point;
        /// Clear all cycles on player request.
        fn clear_cycles();
        /// Get the names of the entries in the given cycle as a string.
        fn get_cycle_names(which: u32) -> Vec<String>;

        /// NEW cycle entry type. This is opaque.
        type HudItem;
        /// Which icon to use for diplaying this item.
        fn icon_file(self: &HudItem) -> String;
        fn icon_fallback(self: &HudItem) -> String;
        fn color(self: &HudItem) -> Color;
        /// Get the item name as a possibly-lossy utf8 string.
        fn name(self: &HudItem) -> String;
        /// Check if the item name is representable in utf8.
        fn name_is_utf8(self: &HudItem) -> bool;
        /// Get the underlying bytes of a possibly non-utf8 name for this item.
        fn name_bytes(self: &HudItem) -> Vec<u8>;
        fn form_string(self: &HudItem) -> String;
        /// Get how many of this item the player has. Updated on inventory changes.
        fn count(self: &HudItem) -> u32;
        fn count_matters(self: &HudItem) -> bool;
        fn is_magic(self: &HudItem) -> bool;

        type SpellData;
        fn fill_out_spell_data(
            hostile: bool,
            resist: i32,
            twohanded: bool,
            school: i32,
            level: u32,
            archetype: i32,
        ) -> Box<SpellData>;
        fn magic_from_spelldata(
            which: ItemCategory,
            spelldata: Box<SpellData>,
            keywords: &CxxVector<CxxString>,
            bytes_ffi: &CxxVector<u8>,
            form_string: String,
            count: u32,
        ) -> Box<HudItem>;
        fn hud_item_from_keywords(
            category: ItemCategory,
            keywords: &CxxVector<CxxString>,
            bytes_ffi: &CxxVector<u8>,
            form_string: String,
            count: u32,
            twohanded: bool,
        ) -> Box<HudItem>;
        fn potion_from_formdata(
            is_poison: bool,
            effect: i32,
            count: u32,
            bytes_ffi: &CxxVector<u8>,
            form_string: String,
        ) -> Box<HudItem>;
        fn make_base_ammo(
            count: u32,
            bytes_ffi: &CxxVector<u8>,
            form_string: String,
        ) -> Box<HudItem>;
        fn simple_from_formdata(
            kind: ItemCategory,
            bytes_ffi: &CxxVector<u8>,
            form_string: String,
        ) -> Box<HudItem>;
        fn make_unarmed_proxy() -> Box<HudItem>;
        fn empty_huditem() -> Box<HudItem>;

        fn icon_files() -> Vec<String>;

        // These are called by plugin hooks and sinks.

        /// Handle an incoming key press event, responding with how it was handled.
        fn handle_key_event(key: u32, button: &ButtonEvent) -> KeyEventResponse;
        /// Handle an in-menu event (which adds/removes items) from the game.
        fn handle_menu_event(key: u32, button: &ButtonEvent) -> bool;
        /// Toggle a menu item in the given cycle.
        fn toggle_item(key: u32, item: Box<HudItem>);
        /// Get the item readied in the given slot, if any.
        fn entry_to_show_in_slot(slot: HudElement) -> Box<HudItem>;
        /// A cycle delay timer has expired. Time to equip!
        fn timer_expired(slot: Action);
        /// Update the entire HUD without any hints about what just changed.
        fn update_hud() -> bool;
        /// Handle equipment-changed events from the game.
        fn handle_item_equipped(
            equipped: bool,
            form_spec: &String,
            right: bool,
            left: bool,
        ) -> bool;
        /// Handle inventory-count changed events from the game.
        fn handle_inventory_changed(form_spec: &String, delta: i32);
        /// Favoriting & unfavoriting.
        fn handle_favorite_event(_button: &ButtonEvent, is_favorite: bool, _item: Box<HudItem>);
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
        /// Check if this is a button-down event.
        fn IsDown(self: &ButtonEvent) -> bool;
        /// Check if this is a button-up event.
        fn IsUp(self: &ButtonEvent) -> bool;
        /// Check if this button is pressed.
        fn IsPressed(self: &ButtonEvent) -> bool;
    }

    // Selected helpers.
    #[namespace = "helpers"]
    unsafe extern "C++" {
        include!("helpers.h");

        /// Display a debug notification on the screen. Used as hacky action confirmation.
        fn notifyPlayer(message: &CxxString);
        /// Start the HUD widget fading in or out to the goal transparency.
        fn startAlphaTransition(fade_in: bool, alpha: f32);
        /// Enter slow time while cycling.
        fn enterSlowMotion();
        /// Exit slow time.
        fn exitSlowMotion();
        /// Show the hud very briefly on a cycle change.
        fn show_briefly();
        /// Play an activation failed UI sound.
        fn honk();
        /// Make a full rust-side item from a form spec string.
        fn formSpecToHudItem(form_spec: &CxxString) -> Box<HudItem>;
    }

    #[namespace = "ui"]
    unsafe extern "C++" {
        include!("ui_renderer.h");

        fn resolutionWidth() -> f32;
        fn resolutionHeight() -> f32;
    }

    // A verbose shim between Rust and the PlayerCharacter type.
    #[namespace = "player"]
    unsafe extern "C++" {
        include!("player.h");

        /// Get the player's name.
        fn playerName() -> Vec<u16>;

        /// Is the player in combat?
        fn isInCombat() -> bool;
        /// Are the player's weapons drawn?
        fn weaponsAreDrawn() -> bool;

        /// Get the form spec for the item readied in the left hand, bound form if possible.
        fn specEquippedLeft() -> String;
        /// Get the form spec for the item readied in the right hand, bound form if possible.
        fn specEquippedRight() -> String;
        /// Get the form id in spec format for the equipped power or shout.
        fn specEquippedPower() -> String;
        /// Get the form id in spec format for the equipped ammo.
        fn specEquippedAmmo() -> String;

        /// Check if the player still has items from this form in their inventory.
        fn hasItemOrSpell(form_spec: &CxxString) -> bool;

        /// Does the player have a bow or crossbow equipped?
        fn hasRangedEquipped() -> bool;
        fn getAmmoInventory() -> Vec<String>;

        /// Unequip the relevant slot.
        fn unequipSlot(which: Action);

        /// Equip the shout matching the form spec.
        fn equipShout(form_spec: &CxxString);
        /// Equip the spell matching the form spec.
        fn equipMagic(form_spec: &CxxString, which: Action);
        /// Equip the weapon matching the form spec.
        fn equipWeapon(form_spec: &CxxString, which: Action);
        /// Re-equip an item in the left hand. This forces an un-equip first.
        fn reequipHand(which: Action, form_spec: &CxxString);
        /// Equip the armor matching the form spec.
        fn equipArmor(form_spec: &CxxString);
        /// Equip the amoo matching the form spec.
        fn equipAmmo(form_spec: &CxxString);
        /// Potions great and small.
        fn consumePotion(form_spec: &CxxString);
        /// Choose and then consume the best potion for the given stat.
        fn chooseMagickaPotion();
        /// Choose a life. Choose a job. Choose a career. Choose a family.
        fn chooseHealthPotion();
        /// Choose a big television. Choose washing machines, cars, etc.
        fn chooseStaminaPotion();
        /// Potion counts for the auto-select item display.
        fn staminaPotionCount() -> u32;
        fn healthPotionCount() -> u32;
        fn magickaPotionCount() -> u32;
        /// Get a count for items with this form spec.
        fn itemCount(form_spec: &CxxString) -> u32;

    }
}
