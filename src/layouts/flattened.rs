//! A shared format for layouts that is used by the renderer.
//! This must be accessible to C++ and support a fast-decisions-only
//! render loop.
//!
//! The struct should move into the plugin module so it is not opaque to C++,
//! but we'll get started on it here.
use crate::plugin::{Action, Align, Color, HudElement, NamedAnchor, Point, SlotLayout};

use super::{layout_v2::SlotElement, HudLayout1, HudLayout2};

pub struct LayoutFlattened {
    /// A global scaling factor for the entire hud.
    global_scale: f32,
    /// Where to draw the HUD; an offset from the top left corner.
    anchor: Point,
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

pub struct SlotFlattened {
    element: HudElement,
    bg_size: Point,
    bg_color: Color,
    bg_image: String,

    icon_size: Point,
    icon_offset: Point,
    icon_color: Color,

    hotkey_size: Point,
    hotkey_offset: Point,
    hotkey_color: Color,
    hotkey_bg_size: Point,
    hotkey_bg_color: Color,
    hotkey_bg_image: String,

    text: Vec<TextFlattened>,
}

pub struct TextFlattened {
    offset: Point,
    color: Color,
    alignment: Align,
    contents: String,
    font_size: f32,
}

impl LayoutFlattened {}

impl From<HudLayout1> for LayoutFlattened {
    fn from(v: HudLayout1) -> Self {
        let slots = v
            .layouts
            .iter()
            .map(|xs| SlotFlattened::from(*xs))
            .collect();

        LayoutFlattened {
            global_scale: v.global_scale,
            anchor: v.anchor_point(),
            bg_size: v.size,
            bg_color: v.bg_color,
            bg_image: "hud_bg.svg".to_string(),
            hide_ammo_when_irrelevant: v.hide_ammo_when_irrelevant,
            hide_left_when_irrelevant: v.hide_left_when_irrelevant,
            font: v.font,
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

impl From<SlotLayout> for SlotFlattened {
    fn from(xs: SlotLayout) -> Self {
        let mut text = Vec::new();
        text.push(TextFlattened {
            offset: xs.name_offset.clone(),
            color: xs.name_color.clone(),
            alignment: xs.align_text.clone(),
            contents: "{name}".to_string(),
            font_size: xs.name_font_size,
        });
        text.push(TextFlattened {
            offset: xs.count_offset.clone(),
            color: xs.count_color.clone(),
            alignment: xs.align_text.clone(),
            contents: "{count}".to_string(),
            font_size: xs.count_font_size,
        });
        SlotFlattened {
            element: xs.element,
            bg_size: xs.size.clone(),
            bg_color: xs.bg_color.clone(),
            bg_image: "slot_bg.svg".to_string(),
            icon_size: xs.icon_size.clone(),
            icon_offset: xs.icon_offset.clone(),
            icon_color: xs.icon_color.clone(),
            hotkey_size: xs.hotkey_size.clone(),
            hotkey_offset: xs.hotkey_offset.clone(),
            hotkey_color: xs.hotkey_color.clone(),
            hotkey_bg_size: xs.hotkey_size.clone(),
            hotkey_bg_color: xs.hotkey_bg_color.clone(),
            hotkey_bg_image: "key_bg.svg".to_string(),
            text,
        }
    }
}

impl From<HudLayout2> for LayoutFlattened {
    fn from(v: HudLayout2) -> Self {
        let mut slots = Vec::new();
        slots.push(SlotFlattened::from(v.power);
        slots.push(SlotFlattened::from(v.utility);
        slots.push(SlotFlattened::from(v.left);
        slots.push(SlotFlattened::from(v.right);
        slots.push(SlotFlattened::from(v.ammo);
        slots.push(SlotFlattened::from(v.equipset);

        LayoutFlattened {
            global_scale: v.global_scale,
            anchor: v.anchor_point(),
            bg_size: v.background.size,
            bg_color: v.background.color,
            bg_image: v.background.svg.clone(),
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

impl From<SlotElement> for SlotFlattened {
    fn from(value: SlotElement) -> Self {
        todo!()
    }
}
