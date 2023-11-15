use serde::{Deserialize, Serialize};

use super::shared::*;
use crate::plugin::{
    Align, Color, HudElement, LayoutFlattened, Point, SlotFlattened, TextFlattened,
};

/// Where to arrange the HUD elements and what color to draw them in.
///
/// This data is serialized to the SoulsyHUD_HudLayout.toml file.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct HudLayout2 {
    #[serde(default)]
    /// A global scaling factor for the entire hud.
    global_scale: f32,
    /// Where to draw the HUD; an offset from the top left corner.
    #[serde(default)]
    anchor: Point,
    /// A background image.
    #[serde(default, deserialize_with = "deserialize_named_anchor")]
    anchor_name: NamedAnchor,
    background: ImageElement,
    right: SlotElement,
    left: SlotElement,
    power: SlotElement,
    utility: SlotElement,
    ammo: SlotElement,
    equipset: SlotElement,
    /// Hide the ammo slot if a ranged weapon is not equipped.
    #[serde(default)]
    hide_ammo_when_irrelevant: bool,
    /// Hide the left hand slot when a ranged weapon is equipped.
    #[serde(default)]
    hide_left_when_irrelevant: bool,
    /// truetype file to load
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

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct SlotElement {
    pub offset: Point,
    pub background: Option<ImageElement>,
    pub icon: IconElement,
    pub hotkey: Option<HotkeyElement>,
    pub text: Vec<TextElement>,
    progress_bar: Option<ProgressElement>,
}

impl HudLayout2 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn anchor_point(&self) -> Point {
        super::anchor_point(
            self.global_scale,
            &self.background.size,
            &self.anchor_name,
            None,
        )
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ImageElement {
    pub svg: String,
    pub size: Point,
    pub color: Color,
}

impl Default for ImageElement {
    fn default() -> Self {
        ImageElement {
            svg: "".to_string(),
            size: Point { x: 0.0, y: 0.0 },
            color: Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            },
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct IconElement {
    pub size: Point,
    pub offset: Point,
    pub color: Color,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HotkeyElement {
    pub size: Point,
    pub offset: Point,
    pub color: Color,
    pub background: Option<ImageElement>,
}

impl Default for HotkeyElement {
    fn default() -> Self {
        HotkeyElement {
            offset: Point { x: 0.0, y: 0.0 },
            size: Point { x: 0.0, y: 0.0 },
            color: Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            },
            background: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TextElement {
    pub offset: Point,
    pub color: Color,
    #[serde(default, deserialize_with = "deserialize_align")]
    pub alignment: Align,
    pub contents: String,
    pub font_size: f32,
}

// TODO TODO TODO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ProgressElement {
    offset: Point,
    color: Color,
}

impl From<&HudLayout2> for LayoutFlattened {
    fn from(v: &HudLayout2) -> Self {
        let slots = vec![
            flatten(&v.power, HudElement::Power),
            flatten(&v.utility, HudElement::Utility),
            flatten(&v.left, HudElement::Left),
            flatten(&v.right, HudElement::Right),
            flatten(&v.ammo, HudElement::Ammo),
            flatten(&v.equipset, HudElement::EquipSet),
        ];

        LayoutFlattened {
            global_scale: v.global_scale,
            anchor: v.anchor_point(),
            bg_size: v.background.size.clone(),
            bg_color: v.background.color.clone(),
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

fn flatten(slot: &SlotElement, element: HudElement) -> SlotFlattened {
    let bg = slot.background.clone().unwrap_or_default();
    let hotkey = slot.hotkey.clone().unwrap_or_default();
    let hkbg = hotkey.background.unwrap_or_default();
    let text = slot.text.iter().map(TextFlattened::from).collect();

    SlotFlattened {
        element,
        offset: slot.offset.clone(),
        bg_size: bg.size,
        bg_color: bg.color,
        bg_image: bg.svg,
        icon_size: slot.icon.size.clone(),
        icon_offset: slot.icon.offset.clone(),
        icon_color: slot.icon.color.clone(),
        hotkey_size: hotkey.size,
        hotkey_offset: hotkey.offset,
        hotkey_color: hotkey.color,
        hotkey_bg_size: hkbg.size,
        hotkey_bg_color: hkbg.color,
        hotkey_bg_image: hkbg.svg,
        text,
    }
}
