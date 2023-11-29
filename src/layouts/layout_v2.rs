use eyre::{Context, Result};
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
    size: Point,
    #[serde(default, deserialize_with = "deserialize_named_anchor")]
    anchor_name: NamedAnchor,
    /// A background image.
    background: Option<ImageElement>,
    right: SlotElement,
    left: SlotElement,
    power: SlotElement,
    utility: SlotElement,
    ammo: SlotElement,
    equipset: Option<SlotElement>,
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
    icon: IconElement,
    text: Vec<TextElement>,
    background: Option<ImageElement>,
    hotkey: Option<HotkeyElement>,
    progress_bar: Option<ProgressElement>,
}

impl HudLayout2 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fallback() -> Self {
        let buf = include_str!("../../data/SKSE/plugins/SoulsyHUD_Layout.toml");
        match toml::from_str::<HudLayout2>(buf) {
            Ok(v) => v,
            Err(e) => {
                log::warn!("The built-in default layout is broken. Please file a bug.");
                log::warn!("{e:#}");
                HudLayout2::default()
            }
        }
    }

    /// Read a v2 layout from a file.
    pub fn read_from_file(pathstr: &str) -> Result<Self> {
        let path = std::path::Path::new(pathstr);
        let buf = std::fs::read_to_string(path)
            .wrap_err_with(|| format!("Unable to read the layout file: {}", pathstr))?;
        let parsed = toml::from_str::<Self>(&buf).wrap_err_with(|| {
            format!("The layout file isn't a valid v2 layout. file={}", pathstr)
        })?;
        Ok(parsed)
    }

    pub fn anchor_point(&self) -> Point {
        super::anchor_point(self.global_scale, &self.size, &self.anchor_name, None)
    }

    fn flatten_slot(&self, slot: &SlotElement, element: HudElement) -> SlotFlattened {
        let bg = slot.background.clone().unwrap_or_default();
        let hotkey = slot.hotkey.clone().unwrap_or_default();
        let hkbg = hotkey.background.unwrap_or_default();

        let anchor = self.anchor_point();
        let center = anchor.translate(&slot.offset.scale(self.global_scale));
        let text = slot
            .text
            .iter()
            .map(|xs| self.flatten_text(xs, &center))
            .collect();

        SlotFlattened {
            element,
            center: center.clone(),
            bg_size: bg.size.scale(self.global_scale),
            bg_color: bg.color,
            bg_image: bg.svg,
            icon_size: slot.icon.size.scale(self.global_scale),
            icon_center: slot.icon.offset.scale(self.global_scale).translate(&center),
            icon_color: slot.icon.color.clone(),
            hotkey_size: hotkey.size.scale(self.global_scale),
            hotkey_center: hotkey.offset.scale(self.global_scale).translate(&center),
            hotkey_color: hotkey.color,
            hotkey_bg_size: hkbg.size.scale(self.global_scale),
            hotkey_bg_color: hkbg.color,
            hotkey_bg_image: hkbg.svg,
            text,
        }
    }

    fn flatten_text(&self, text: &TextElement, center: &Point) -> TextFlattened {
        TextFlattened {
            anchor: text.offset.scale(self.global_scale).translate(center),
            color: text.color.clone(),
            alignment: text.alignment,
            contents: text.contents.clone(),
            font_size: text.font_size,
        }
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
    offset: Point,
    color: Color,
    #[serde(default, deserialize_with = "deserialize_align")]
    alignment: Align,
    contents: String,
    font_size: f32,
}

// TODO TODO TODO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ProgressElement {
    offset: Point,
    color: Color,
}

impl From<&HudLayout2> for LayoutFlattened {
    fn from(v: &HudLayout2) -> Self {
        let mut slots = vec![
            v.flatten_slot(&v.power, HudElement::Power),
            v.flatten_slot(&v.utility, HudElement::Utility),
            v.flatten_slot(&v.left, HudElement::Left),
            v.flatten_slot(&v.right, HudElement::Right),
            v.flatten_slot(&v.ammo, HudElement::Ammo),
        ];
        if let Some(equipset) = v.equipset.as_ref() {
            slots.push(v.flatten_slot(equipset, HudElement::EquipSet));
        }
        let bg = v.background.clone().unwrap_or_default();

        LayoutFlattened {
            global_scale: v.global_scale,
            anchor: v.anchor_point(),
            size: v.size.scale(v.global_scale),
            bg_size: bg.size.scale(v.global_scale),
            bg_color: bg.color.clone(),
            bg_image: bg.svg.clone(),
            hide_ammo_when_irrelevant: v.hide_ammo_when_irrelevant,
            hide_left_when_irrelevant: v.hide_left_when_irrelevant,
            font: v.font.clone(),
            font_size: v.font_size * v.global_scale,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layouts::{resolutionHeight, Layout};

    // #[test]
    // #[ignore]
    // fn default_layout_valid() {
    //     // The github runner compilation step can't find this file. I have no idea why not.
    //     let buf = include_str!("../../data/SKSE/plugins/SoulsyHUD_layout.toml");
    //     let builtin: Layout = toml::from_str(buf).expect("layout should be valid toml");
    //     match builtin {
    //         Layout::Version1(_) => unreachable!(),
    //         Layout::Version2(v) => {
    //             assert_eq!(v.anchor_name, NamedAnchor::BottomLeft);
    //             assert_eq!(v.anchor_point().x, 150.0);
    //             assert_eq!(v.anchor_point().y, 1290.0);
    //         }
    //     }
    // }

    #[test]
    fn centered_layout_valid() {
        let buf = include_str!("../../layouts/SoulsyHUD_centered.toml");
        let centered: HudLayout2 = toml::from_str(buf).expect("layout should be valid toml");
        assert_eq!(centered.anchor_name, NamedAnchor::Center);
        assert_eq!(centered.anchor_point().x, 1720.0);
        assert_eq!(centered.anchor_point().y, 720.0);
    }

    #[test]
    fn topleft_layout_valid() {
        let buf = include_str!("../../layouts/SoulsyHUD_topleft.toml");
        let layout: HudLayout2 = toml::from_str(buf).expect("layout should be valid toml");
        assert_eq!(layout.anchor_name, NamedAnchor::None);
        assert_eq!(layout.anchor_point().x, 150.0);
        assert_eq!(layout.anchor_point().y, 150.0);
    }

    #[test]
    fn minimal_layout_valid() {
        let data = include_str!("../../layouts/SoulsyHUD_minimal.toml");
        let specific: HudLayout2 =
            toml::from_str(data).expect("minimal layout should be valid toml");
        assert_eq!(specific.anchor_name, NamedAnchor::BottomLeft);
        let minimal: Layout =
            toml::from_str(data).expect("serde should figure out which layout schema");
        match minimal {
            Layout::Version1(_) => unreachable!(),
            Layout::Version2(v) => {
                assert_eq!(v.anchor_name, NamedAnchor::BottomLeft);
                assert_eq!(v.anchor_point().x, 150.0);
                assert_eq!(v.anchor_point().y, 1315.0);
            }
        }
    }

    #[test]
    fn curvy_left_top_valid() {
        let data = include_str!("../../layouts/curvy/SoulsyHUD_curvy_left_top.toml");
        let parsed: Layout =
            toml::from_str(data).expect("serde should figure out which layout schema");
        match parsed {
            Layout::Version1(_) => unreachable!(),
            Layout::Version2(v) => {
                assert_eq!(v.anchor_name, NamedAnchor::TopLeft);
                assert_eq!(v.anchor_point().x, 160.0);
                assert_eq!(v.anchor_point().y, 160.0);
            }
        }
    }

    #[test]
    fn curvy_left_bottom_valid() {
        let data = include_str!("../../layouts/curvy/SoulsyHUD_curvy_left_bottom.toml");
        let parsed: Layout =
            toml::from_str(data).expect("serde should figure out which layout schema");
        match parsed {
            Layout::Version1(_) => unreachable!(),
            Layout::Version2(v) => {
                assert_eq!(v.anchor_name, NamedAnchor::BottomLeft);
                assert_eq!(v.anchor_point().x, 160.0);
                assert_eq!(v.anchor_point().y, 1280.0);
            }
        }
    }

    #[test]
    fn flattening_applies_scale() {
        let data = include_str!("../../layouts/square/LayoutV2.toml");
        let layout: HudLayout2 =
            toml::from_str(data).expect("square text fixture should be valid toml");
        assert_eq!(layout.global_scale, 2.0);
        assert_eq!(layout.size, Point { x: 190.0, y: 250.0 });
        assert_eq!(layout.font_size, 18.0);
        assert_eq!(layout.right.offset, Point { x: 375.0, y: 0.0 });
        let anchor = layout.anchor_point();
        assert_eq!(
            anchor,
            Point {
                x: 190.0,
                y: resolutionHeight() - layout.size.y
            }
        );

        // if the above assertions succeed, these should too.
        let flattened = Layout::Version2(Box::new(layout.clone())).flatten();
        assert_eq!(flattened.size, Point { x: 380.0, y: 500.0 });
        assert_eq!(flattened.bg_size, Point { x: 380.0, y: 500.0 });
        assert_eq!(flattened.font_size, 36.0);
        assert_eq!(flattened.anchor, anchor);
        assert_eq!(flattened.slots[0].bg_size, Point { x: 200.0, y: 200.0 });
        let right_slot = flattened
            .slots
            .iter()
            .find(|slot| slot.element == HudElement::Right)
            .expect("the right slot must be present");
        let slot_center = Point {
            x: flattened.anchor.x + (layout.right.offset.x * flattened.global_scale),
            y: flattened.anchor.y + (layout.right.offset.y * flattened.global_scale),
        };
        assert_eq!(right_slot.center, slot_center);
    }
}
