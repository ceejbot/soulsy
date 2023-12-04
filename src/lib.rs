//! Entry point for the Rust chunk of the mod DLL. lib.rs defines the bridge
//! between C++ and Rust. C++ drives the plugin (see main.cpp) and answers
//! questions about game state, but Rust provides all the logic and data manipulation.

#![deny(future_incompatible, clippy::unwrap_used)]
#![warn(rust_2018_idioms, trivial_casts)]

pub mod controller;
pub mod data;
pub mod images;
pub mod layouts;

use controller::*;
use data::{HudItem, SpellData, *};
use images::{get_icon_key, rasterize_by_path, rasterize_icon};
use layouts::hud_layout;

/// Rust defines the bridge between it and C++ in the `plugin` mod, using the
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
        EquipSet,
        None, // not drawn
    }

    /// Text alignment options
    #[derive(Debug, Clone, Hash)]
    enum Align {
        Left,
        Right,
        Center,
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
    /// This data is an unrolled version of the data from our two layout schema
    /// versions, intended to make the render loop easier to implement. Translation:
    /// no Option<T> types and no expensive calculations in the loop. The global
    /// scale factor has already been applied.
    #[derive(Clone, Debug)]
    pub struct LayoutFlattened {
        /// A global scaling factor for the entire hud.
        global_scale: f32,
        /// Where to draw the HUD; an offset from the top left corner.
        anchor: Point,
        /// HUD size after scaling.
        size: Point,
        /// Hide the ammo slot if a ranged weapon is not equipped.
        hide_ammo_when_irrelevant: bool,
        /// Hide the left hand slot when a ranged weapon is equipped.
        hide_left_when_irrelevant: bool,
        /// The ttf file to load the font fromt.
        font: String,
        /// The font size for most things; a hint to the font loader.
        font_size: f32,
        /// Whether to buld glyphs for full Chinese text display.
        chinese_full_glyphs: bool,
        /// Whether to build glyphs for simplified Chinese text display.
        simplified_chinese_glyphs: bool,
        /// Whether to build glyphs for simplified Chinese text display.
        cyrillic_glyphs: bool,
        /// Whether to build glyphs for Cyrillic text display.
        japanese_glyphs: bool,
        /// Whether to build glyphs for Japanese text display.
        korean_glyphs: bool,
        /// Whether to build glyphs for Thai text display.
        thai_glyphs: bool,
        /// Whether to build glyphs for Vietnamese text display.
        vietnamese_glyphs: bool,
        /// The dimensions of a bounding box for the HUD.
        bg_size: Point,
        /// The color to draw the HUD bg image with; if zero will not be drawn.
        bg_color: Color,
        bg_image: String,
        /// One slot layout for each element. This wants to be map, not a vec,
        /// but the map types are not shareable.
        slots: Vec<SlotFlattened>,
    }

    /// Layout variables for a single HUD slot, e.g, the power slot.
    #[derive(Clone, Debug)]
    pub struct SlotFlattened {
        element: HudElement,
        center: Point,
        bg_size: Point,
        bg_color: Color,
        bg_image: String,

        icon_size: Point,
        icon_center: Point,
        icon_color: Color,

        hotkey_size: Point,
        hotkey_center: Point,
        hotkey_color: Color,
        hotkey_bg_size: Point,
        hotkey_bg_color: Color,
        hotkey_bg_image: String,

        poison_size: Point,
        poison_center: Point,
        poison_color: Color,
        poison_image: String,

        text: Vec<TextFlattened>,
    }

    #[derive(Clone, Debug)]
    pub struct TextFlattened {
        anchor: Point,
        color: Color,
        alignment: Align,
        contents: String,
        font_size: f32,
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
        /// Long press tracking for timers. The next 4 are menu timers.
        /// The equipset cycle hotkey.
        LongPressLeft,
        /// Long press the right cycle key.
        LongPressRight,
        /// Long press the powers/shouts key.
        LongPressPower,
        /// Long press the powers/shouts key.
        LongPressUtility,
        /// The equipset cycle hotkey.
        Equipment,
    }

    /// A high-level item category, used to jump-start item categorization via keywords & form data.
    /// These categories make sense to the HUD and do not have to map to form types.
    #[derive(Debug, Clone, Hash)]
    enum ItemCategory {
        Ammo,
        Armor,
        Food,
        HandToHand,
        Lantern,
        Potion,
        Power,
        Scroll,
        Shout,
        Spell,
        Torch,
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

    /// What the player has equipped, and which armor slots are empty.
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct EquippedData {
        items: Vec<String>,
        empty_slots: Vec<u8>,
    }

    /// Struct passing rasterized SVG data around.
    #[derive(Debug, Default, Clone)]
    struct LoadedImage {
        width: u32,
        height: u32,
        buffer: Vec<u8>,
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
        /// What log level to use, shared across Rust & C++.
        fn log_level_number(self: &UserSettings) -> u32;
        /// The identifier to use for this mod in SKSE cosaves. Not exposed in UI.
        fn skse_identifier(self: &UserSettings) -> u32;

        /// After an MCM-managed change, re-read our .ini file.
        fn refresh_user_settings();
        /// Fetch a read-only copy of our current layout.
        fn hud_layout() -> LayoutFlattened;

        /// Cached data for items displayed in cycles. This is opaque to C++.
        type HudItem;
        /// Which icon to use for diplaying this item.
        fn icon_key(self: &HudItem) -> String;
        /// Get the color to use to draw this item's icon.
        fn color(self: &HudItem) -> Color;
        /// Get the item name as a possibly-lossy utf8 string.
        fn name(self: &HudItem) -> String;
        /// Check if the item name is representable in utf8.
        fn name_is_utf8(self: &HudItem) -> bool;
        /// Get the underlying bytes of a possibly non-utf8 name for this item.
        fn name_bytes(self: &HudItem) -> Vec<u8>;
        /// Get the form spec string for this item; format is `Plugin.esp|0xdeadbeef`
        fn form_string(self: &HudItem) -> String;
        /// Get how many of this item the player has. Updated on inventory changes.
        fn count(self: &HudItem) -> u32;
        /// Check if this item has a meaningful count.
        fn count_matters(self: &HudItem) -> bool;
        /// Render a format string for the HUD.
        fn fmtstr(self: &HudItem, format: String) -> String;
        /// Check if this item is poisoned.
        fn is_poisoned(self: &HudItem) -> bool;

        /// See src/data/magic.rs for this struct. It's used to classify spells.
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

        /// Build a HUD item from a rough category and a list of keywords from OCF and other mods.
        fn hud_item_from_keywords(
            category: ItemCategory,
            keywords: &CxxVector<CxxString>,
            bytes_ffi: &CxxVector<u8>,
            form_string: String,
            count: u32,
            twohanded: bool,
        ) -> Box<HudItem>;
        /// Build a HUD item for a potion from its major effect and a hint about whether it's poison or not.
        fn potion_from_formdata(
            is_poison: bool,
            effect: i32,
            count: u32,
            bytes_ffi: &CxxVector<u8>,
            form_string: String,
        ) -> Box<HudItem>;
        /// Build a very simple item, one where the rough category can specify everything. Only used
        /// now for lights & shouts as a fallback.
        fn simple_from_formdata(
            kind: ItemCategory,
            bytes_ffi: &CxxVector<u8>,
            form_string: String,
        ) -> Box<HudItem>;
        /// Build an empty HUD item.
        fn empty_huditem() -> Box<HudItem>;

        /// Call this to get the fallback-aware key for an icon.
        fn get_icon_key(name: String) -> String;
        /// Load a rasterized image for an icon given its key.
        fn rasterize_icon(key: String, maxdim: u32) -> LoadedImage;
        /// Rasterize an SVG by path.
        fn rasterize_by_path(fpath: String) -> LoadedImage;

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
        /// Handle equipment-changed events from the game.
        fn handle_item_equipped(
            equipped: bool,
            form_spec: &String,
            worn_right: &String,
            worn_left: &String,
        ) -> bool;
        /// Handle inventory-count changed events from the game.
        fn handle_inventory_changed(form_spec: &String, count: u32);
        /// Favoriting & unfavoriting.
        fn handle_favorite_event(_button: &ButtonEvent, is_favorite: bool, _item: Box<HudItem>);
        /// Handle CGO switching grip mode.
        fn handle_grip_change(use_alt_grip: bool);
        /// Clear all cycles on player request.
        fn clear_cycles();
        /// Get the names of the entries in the given cycle as a vec of strings. Used in MCM.
        fn get_cycle_names(which: i32) -> Vec<String>;
        /// Get a list of form spec strings for the given cycle. Used in MCM.
        fn get_cycle_formids(which: i32) -> Vec<String>;
        /// Get equip set names in order by id. Used in MCM.
        fn get_equipset_names() -> Vec<String>;
        /// Get equip set ids. Used in MCM.
        fn get_equipset_ids() -> Vec<String>;
        /// Turn a string representation of an index into the above array into the id as int.
        fn equipset_index_to_id(idx: String) -> i32;
        /// Create a new equipment set. Used in MCM.
        fn handle_create_equipset(name: String) -> bool;
        /// Save an equipment set. Used in MCM.
        fn handle_update_equipset(id: u32) -> bool;
        /// Rename an equipment set. Used in MCM.
        fn handle_rename_equipset(id: u32, name: String) -> bool;
        /// Remove an equipment set. Used in MCM.
        fn handle_remove_equipset(id: u32) -> bool;
        /// For papyrus: parse a string as an int. Used in MCM.
        fn string_to_int(number: String) -> i32;
        /// Make the Rust equipped data struct from the given data.
        fn equipped_data(items: Vec<String>, empty: Vec<u8>) -> Box<EquippedData>;
        /// Get a vec of the names of all items in this equip set. Called by MCM.
        fn get_equipset_item_names(id: u32) -> Vec<String>;
        /// Set which item's icon to use for this equipset. Called by MCM.
        fn set_equipset_icon(id: u32, itemname: String) -> bool;
        /// Given the selected equipset name, get its integer id. Called by MCM.
        fn look_up_equipset_by_name(name: String) -> u32;
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

        /// Display a notification on the screen. You must format and translate in advance.
        fn notifyPlayer(message: &CxxString);
        /// Look up a translation for a format string.
        fn lookupTranslation(key: &CxxString) -> String;
        /// Play an activation failed UI sound.
        fn honk();
        /// Make a full HUD-drawing-ready item from a form spec string.
        fn formSpecToHudItem(form_spec: &CxxString) -> Box<HudItem>;
        /// Is this item poisoned?
        fn isPoisonedByFormSpec(form_spec: &CxxString) -> bool;
        /// Get an item's enchant level. Will be 0 for all unenchanted items.
        fn chargeLevelByFormSpec(form_spec: &CxxString) -> f32;
    }

    #[namespace = "ui"]
    unsafe extern "C++" {
        include!("ui_renderer.h");

        /// Get the display width in pixels.
        fn resolutionWidth() -> f32;
        /// Get the display height in pixels.
        fn resolutionHeight() -> f32;
        /// Start the named timer. Duretion is looked up from settings.
        fn startTimer(which: Action, duration: u32);
        /// Stop the named timer.
        fn stopTimer(which: Action);
        /// Show the hud very briefly on a cycle change.
        fn showBriefly();
        /// Start the HUD widget fading in or out to the goal transparency.
        fn startAlphaTransition(fade_in: bool, alpha: f32);

    }

    // A verbose shim between Rust and the PlayerCharacter type.
    #[namespace = "player"]
    unsafe extern "C++" {
        include!("player.h");

        /// Get the player's name as a vec of wide bytes. Might not be valid utf8.
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
        /// Get a vec of form specs for all relevant ammo in the player's inventory.
        /// The vec is sorted by damage.
        fn getAmmoInventory() -> Vec<String>;

        /// Get a list of form specs for all equipped armor. Used to build an equipset.
        fn getEquippedItems() -> Box<EquippedData>;

        /// Unequip the relevant slot.
        fn unequipSlot(which: Action);
        /// Unequip a slot identified by biped object slot.
        fn unequipSlotByShift(shift: u8);

        /// Equip the shout matching the form spec.
        fn equipShout(form_spec: &CxxString);
        /// Equip the spell matching the form spec.
        fn equipMagic(form_spec: &CxxString, which: Action);
        /// Equip the weapon matching the form spec.
        fn equipWeapon(form_spec: &CxxString, which: Action);
        /// Re-equip an item in the left hand. This forces an un-equip first.
        fn reequipHand(which: Action, form_spec: &CxxString);
        /// Toggle the armor matching the form spec.
        fn toggleArmor(form_spec: &CxxString);
        /// Equip the armor; do not toggle.
        fn equipArmor(form_spec: &CxxString);
        /// Equip the ammo matching the form spec.
        fn equipAmmo(form_spec: &CxxString);
        /// Potions great and small.
        fn consumePotion(form_spec: &CxxString);
        /// Choose and then consume the best potion for the given stat.
        fn chooseMagickaPotion();
        /// Choose a life. Choose a job. Choose a career. Choose a family.
        fn chooseHealthPotion();
        /// Choose a big television. Choose washing machines, cars, you know the rest.
        fn chooseStaminaPotion();
        /// How many restore stamina potions the player has in inventory. For grouped potions.
        fn staminaPotionCount() -> u32;
        /// How many restore health potions the player has in inventory. For grouped potions.
        fn healthPotionCount() -> u32;
        /// How many restore magicka potions the player has in inventory. For grouped potions.
        fn magickaPotionCount() -> u32;
        /// Get a count for items with this form spec.
        fn itemCount(form_spec: &CxxString) -> u32;
        /// Is the player using CGO's alt-grip mode? (Always false if not using CGO or compatible mod.)
        fn useCGOAltGrip() -> bool;
    }
}
