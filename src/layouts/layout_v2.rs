use serde::{Deserialize, Serialize};

use super::shared::*;
use crate::plugin::{resolutionHeight, resolutionWidth, Align, Color, NamedAnchor, Point};

/// Where to arrange the HUD elements and what color to draw them in.
///
/// This data is serialized to the SoulsyHUD_HudLayout.toml file.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct HudLayout2 {
    #[serde(default)]
    /// A global scaling factor for the entire hud.
    pub global_scale: f32,
    /// Where to draw the HUD; an offset from the top left corner.
    #[serde(default)]
    pub anchor: Point,
    #[serde(default, deserialize_with = "deserialize_named_anchor")]
    pub anchor_name: NamedAnchor,
    pub background: ImageElement,
    pub right: SlotElement,
    pub left: SlotElement,
    pub power: SlotElement,
    pub utility: SlotElement,
    pub equipset: SlotElement,
    /// Hide the ammo slot if a ranged weapon is not equipped.
    #[serde(default)]
    pub hide_ammo_when_irrelevant: bool,
    /// Hide the left hand slot when a ranged weapon is equipped.
    #[serde(default)]
    pub hide_left_when_irrelevant: bool,
    /// truetype file to load
    pub font: String,
    /// The font size for most things; a hint to the font loader.
    pub font_size: f32,
    /// Whether to buld glyphs for full Chinese text display.
    #[serde(default)]
    pub chinese_full_glyphs: bool,
    /// Whether to build glyphs for simplified Chinese text display.
    #[serde(default)]
    pub simplified_chinese_glyphs: bool,
    /// Whether to build glyphs for simplified Chinese text display.
    #[serde(default)]
    pub cyrillic_glyphs: bool,
    /// Whether to build glyphs for Cyrillic text display.
    #[serde(default)]
    pub japanese_glyphs: bool,
    /// Whether to build glyphs for Japanese text display.
    #[serde(default)]
    pub korean_glyphs: bool,
    /// Whether to build glyphs for Thai text display.
    #[serde(default)]
    pub thai_glyphs: bool,
    /// Whether to build glyphs for Vietnamese text display.
    #[serde(default)]
    pub vietnamese_glyphs: bool,
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

    pub fn anchor_point(&self) -> Point {
        // If we read a named anchor point, turn it into pixels.
        // The anchor point is the location of the hud CENTER, so we offset.
        let screen_width = resolutionWidth();
        let screen_height = resolutionHeight();
        let width = self.background.size.x * self.global_scale;
        let height = self.background.size.y * self.global_scale;

        match self.anchor_name {
            NamedAnchor::TopLeft => Point {
                x: width / 2.0,
                y: height / 2.0,
            },
            NamedAnchor::TopRight => Point {
                x: screen_width - width / 2.0,
                y: height / 2.0,
            },
            NamedAnchor::BottomLeft => Point {
                x: width / 2.0,
                y: screen_height - height / 2.0,
            },
            NamedAnchor::BottomRight => Point {
                x: screen_width - width / 2.0,
                y: screen_height - height / 2.0,
            },
            NamedAnchor::Center => Point {
                x: screen_width / 2.0,
                y: screen_height / 2.0,
            },
            NamedAnchor::CenterTop => Point {
                x: screen_width / 2.0,
                y: height / 2.0,
            },
            NamedAnchor::CenterBottom => Point {
                x: screen_width / 2.0,
                y: screen_height - height / 2.0,
            },
            NamedAnchor::LeftCenter => Point {
                x: width / 2.0,
                y: screen_height / 2.0,
            },
            NamedAnchor::RightCenter => Point {
                x: screen_width - width / 2.0,
                y: screen_height / 2.0,
            },
            _ => {
                if self.anchor == Point::default() {
                    log::info!("Layout has neither a named anchor nor an anchor point. Falling back to top left.");
                    Point {
                        x: width / 2.0,
                        y: height / 2.0,
                    }
                } else {
                    self.anchor.clone()
                }
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ImageElement {
    pub svg: String,
    pub size: Point,
    pub color: Color,
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
