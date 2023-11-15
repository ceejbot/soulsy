//! The layout for the HUD is read from a TOML file. This data is shared between
//! languages the same way that the user settings are. The Rust side reads the
//! toml; the C++ side uses the data in layout. The majority of the implementation
//! is filing in defaults.

#![allow(non_snake_case, non_camel_case_types)]

use serde::{Deserialize, Serialize};

use crate::layouts::shared::NamedAnchor;
use crate::plugin::{
    Align, Color, HudElement, LayoutFlattened, Point, SlotFlattened, TextFlattened,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HudLayout1 {
    #[serde(default)]
    /// A global scaling factor for the entire hud.
    global_scale: f32,
    /// Where to draw the HUD; an offset from the top left corner.
    #[serde(default)]
    anchor: Point,
    #[serde(
        default,
        deserialize_with = "crate::layouts::shared::deserialize_named_anchor"
    )]
    pub anchor_name: NamedAnchor, // pub for tests
    /// The dimensions of a bounding box for the HUD.
    size: Point,
    /// The color to draw the HUD bg image with; if zero will not be drawn.
    bg_color: Color,
    /// Hide the ammo slot if a ranged weapon is not equipped.
    #[serde(default)]
    hide_ammo_when_irrelevant: bool,
    /// Hide the left hand slot when a ranged weapon is equipped.
    #[serde(default)]
    hide_left_when_irrelevant: bool,
    /// One slot layout for each element. This wants to be map, not a vec,
    /// but the map types are not shareable.
    layouts: Vec<SlotLayout>,
    font: String,
    /// The font size for most things; a hint to the font loader.
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SlotLayout {
    /// The hud element this layout is for.
    element: HudElement,
    /// The name of the hud element this layout is for. For humans.
    name: String,
    /// How to align any text associated with this slot.
    #[serde(
        default,
        deserialize_with = "crate::layouts::shared::deserialize_align"
    )]
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

impl HudLayout1 {
    pub fn anchor_point(&self) -> Point {
        super::anchor_point(
            self.global_scale,
            &self.size,
            &self.anchor_name,
            Some(&self.anchor),
        )
    }
}

impl From<&HudLayout1> for LayoutFlattened {
    fn from(v: &HudLayout1) -> Self {
        let slots = v.layouts.iter().map(SlotFlattened::from).collect();

        LayoutFlattened {
            global_scale: v.global_scale,
            anchor: v.anchor_point(),
            bg_size: v.size.clone(),
            bg_color: v.bg_color.clone(),
            bg_image: "hud_bg.svg".to_string(),
            hide_ammo_when_irrelevant: v.hide_ammo_when_irrelevant,
            hide_left_when_irrelevant: v.hide_left_when_irrelevant,
            font: v.font.clone(),
            font_size: v.font_size,
            chinese_full_glyphs: v.chinese_full_glyphs,
            simplified_chinese_glyphs: v.simplified_chinese_glyphs,
            cyrillic_glyphs: v.cyrillic_glyphs,
            japanese_glyphs: v.japanese_glyphs,
            korean_glyphs: v.korean_glyphs,
            thai_glyphs: v.thai_glyphs,
            vietnamese_glyphs: v.vietnamese_glyphs,
            slots,
        }
    }
}

impl From<&SlotLayout> for SlotFlattened {
    fn from(slot: &SlotLayout) -> Self {
        let text = vec![
            TextFlattened {
                offset: slot.name_offset.clone(),
                color: slot.name_color.clone(),
                alignment: slot.align_text,
                contents: "{name}".to_string(),
                font_size: slot.name_font_size,
            },
            TextFlattened {
                offset: slot.count_offset.clone(),
                color: slot.count_color.clone(),
                alignment: slot.align_text,
                contents: "{count}".to_string(),
                font_size: slot.count_font_size,
            },
        ];
        SlotFlattened {
            element: slot.element,
            offset: slot.offset.clone(),
            bg_size: slot.size.clone(),
            bg_color: slot.bg_color.clone(),
            bg_image: "slot_bg.svg".to_string(),
            icon_size: slot.icon_size.clone(),
            icon_offset: slot.icon_offset.clone(),
            icon_color: slot.icon_color.clone(),
            hotkey_size: slot.hotkey_size.clone(),
            hotkey_offset: slot.hotkey_offset.clone(),
            hotkey_color: slot.hotkey_color.clone(),
            hotkey_bg_size: slot.hotkey_size.clone(),
            hotkey_bg_color: slot.hotkey_bg_color.clone(),
            hotkey_bg_image: "key_bg.svg".to_string(),
            text,
        }
    }
}

impl Default for HudLayout1 {
    fn default() -> Self {
        // compile-time include of default layout toml
        let buf = include_str!("../../data/SKSE/plugins/SoulsyHUD_Layout.toml");
        toml::from_str::<HudLayout1>(buf)
            .expect("Default layout is not valid toml! Cannot proceed.")
    }
}
