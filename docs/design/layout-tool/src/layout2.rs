use serde::{Deserialize, Serialize};

use crate::layout_elements::*;

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
    #[serde(default, deserialize_with = "crate::deserialize_named_anchor")]
    anchor_name: NamedAnchor,
    background: Option<ImageElement>,
    right: SlotElement,
    left: SlotElement,
    power: SlotElement,
    utility: SlotElement,
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
    offset: Point,
    background: Option<ImageElement>,
    icon: IconElement,
    hotkey: Option<HotkeyElement>,
    text: Vec<TextElement>,
    progress_bar: Option<ProgressElement>,
}

impl HudLayout2 {
    pub fn new() -> Self {
        Self::default()
    }
}
