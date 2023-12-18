//! The layout for the HUD is read from a TOML file. This data is shared between
//! languages the same way that the user settings are. The Rust side reads the
//! toml; the C++ side uses the data in layout. The majority of the
//! implementation is filing in defaults.

#![allow(non_snake_case, non_camel_case_types)]

use eyre::{Context, Result};
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
    anchor: Option<Point>,
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
    /// Width at which to wrap any count text. 0 = no wrap.
    #[serde(default)]
    count_wrap_width: f32,

    /// The color of any item name text; 0 alpha means not to draw it at all.
    name_color: Color,
    /// Where to draw the item name.
    name_offset: Point,
    /// The font size to use for this item's name.
    #[serde(default)]
    name_font_size: f32,
    /// Width at which to wrap any name text. 0 = no wrap.
    #[serde(default)]
    name_wrap_width: f32,
}

impl HudLayout1 {
    /// Read a v2 layout from a file.
    pub fn read_from_file(pathstr: &str) -> Result<Self> {
        let path = std::path::Path::new(pathstr);
        let buf = std::fs::read_to_string(path)
            .wrap_err_with(|| format!("Unable to read the layout file: {}", pathstr))?;
        let parsed = toml::from_str::<Self>(&buf).wrap_err_with(|| {
            format!("The layout file isn't a valid v1 layout. file={}", pathstr)
        })?;
        Ok(parsed)
    }

    pub fn anchor_point(&self) -> Point {
        super::anchor_point(
            self.global_scale,
            &self.size,
            &self.anchor_name,
            self.anchor.as_ref(),
        )
    }

    fn flatten(&self, slot: &SlotLayout) -> SlotFlattened {
        let anchor = self.anchor_point();
        let center = anchor.translate(&slot.offset.scale(self.global_scale));

        let mut text = Vec::new();
        if slot.name_color.a > 0 {
            text.push(TextFlattened {
                anchor: slot.name_offset.scale(self.global_scale).translate(&center),
                color: slot.name_color.clone(),
                alignment: slot.align_text,
                contents: "{name}".to_string(),
                font_size: slot.name_font_size * self.global_scale,
                wrap_width: slot.name_wrap_width,
            });
        }
        if slot.count_color.a > 0 {
            text.push(TextFlattened {
                anchor: slot
                    .count_offset
                    .scale(self.global_scale)
                    .translate(&center),
                color: slot.count_color.clone(),
                alignment: slot.align_text,
                contents: "{count}".to_string(),
                font_size: slot.count_font_size * self.global_scale,
                wrap_width: slot.count_wrap_width,
            });
        }

        SlotFlattened {
            element: slot.element,
            center: center.clone(),
            bg_size: slot.size.scale(self.global_scale),
            bg_color: slot.bg_color.clone(),
            bg_image: "slot_bg.svg".to_string(),

            icon_size: slot.icon_size.scale(self.global_scale),
            icon_center: slot.icon_offset.scale(self.global_scale).translate(&center),
            icon_color: slot.icon_color.clone(),

            hotkey_size: slot.hotkey_size.scale(self.global_scale),
            hotkey_center: slot
                .hotkey_offset
                .scale(self.global_scale)
                .translate(&center),
            hotkey_color: slot.hotkey_color.clone(),

            hotkey_bg_size: slot.hotkey_size.scale(self.global_scale),
            hotkey_bg_color: slot.hotkey_bg_color.clone(),
            hotkey_bg_image: "key_bg.svg".to_string(),

            poison_image: "".to_string(),
            poison_color: Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            },
            poison_center: Point { x: 0.0, y: 0.0 },
            poison_size: Point { x: 0.0, y: 0.0 },

            text,
        }
    }
}

impl From<&HudLayout1> for LayoutFlattened {
    fn from(v: &HudLayout1) -> Self {
        let slots = v.layouts.iter().map(|xs| v.flatten(xs)).collect();

        LayoutFlattened {
            global_scale: v.global_scale,
            anchor: v.anchor_point(),
            size: v.size.scale(v.global_scale),
            bg_size: Point {
                x: v.size.x * v.global_scale,
                y: v.size.y * v.global_scale,
            },
            bg_color: v.bg_color.clone(),
            bg_image: "hud_bg.svg".to_string(),
            hide_ammo_when_irrelevant: v.hide_ammo_when_irrelevant,
            hide_left_when_irrelevant: v.hide_left_when_irrelevant,
            font: v.font.clone(),
            font_size: v.font_size * v.global_scale,
            // glyphs requested
            chinese_full_glyphs: v.chinese_full_glyphs,
            simplified_chinese_glyphs: v.simplified_chinese_glyphs,
            cyrillic_glyphs: v.cyrillic_glyphs,
            japanese_glyphs: v.japanese_glyphs,
            korean_glyphs: v.korean_glyphs,
            thai_glyphs: v.thai_glyphs,
            vietnamese_glyphs: v.vietnamese_glyphs,
            // layout slots
            slots,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::Layout;
    use super::*;

    #[test]
    fn hexagon_lr_valid() {
        let data = include_str!("../../layouts/hexagons/SoulsyHUD_hexagons_lr.toml");
        let specific: HudLayout1 =
            toml::from_str(data).expect("minimal layout should be valid toml");
        assert_eq!(specific.anchor_name, NamedAnchor::TopRight);
        let minimal: Layout =
            toml::from_str(data).expect("serde should figure out which layout schema");
        match minimal {
            Layout::Version2(_) => unreachable!(),
            Layout::Version1(ref v) => {
                assert_eq!(v.anchor_name, NamedAnchor::TopRight);
                assert_eq!(v.anchor_point().x, 3290.0);
                assert_eq!(v.anchor_point().y, 150.0);
            }
        }
        let flattened = minimal.flatten();
        assert_eq!(flattened.anchor.x, 3290.0);
        assert_eq!(flattened.anchor.y, 150.0);
    }

    #[test]
    fn hexagon_tb_valid() {
        let data = include_str!("../../layouts/hexagons/SoulsyHUD_hexagons_tb.toml");
        let specific: HudLayout1 =
            toml::from_str(data).expect("hexagons_tb layout should be valid toml");
        assert_eq!(specific.anchor_name, NamedAnchor::BottomRight);
        let hexagonal: Layout =
            toml::from_str(data).expect("serde should figure out which layout schema");
        match hexagonal {
            Layout::Version2(_) => unreachable!(),
            Layout::Version1(ref v) => {
                assert_eq!(v.anchor_name, NamedAnchor::BottomRight);
                assert_eq!(v.anchor_point().x, 3290.0);
                assert_eq!(v.anchor_point().y, 1290.0);
            }
        }
        let flattened = hexagonal.flatten();
        assert_eq!(flattened.anchor.x, 3290.0);
        assert_eq!(flattened.anchor.y, 1290.0);
    }

    #[test]
    fn flattening_applies_scale() {
        let data = include_str!("../../layouts/hexagons/SoulsyHUD_hexagons_tb.toml");
        let layout: HudLayout1 = toml::from_str(data).expect("minimal layout should be valid toml");
        assert_eq!(layout.global_scale, 0.5);
        assert_eq!(layout.size, Point { x: 600.0, y: 600.0 });
        assert_eq!(layout.font_size, 37.0);
        assert_eq!(layout.layouts[0].size, Point { x: 200.0, y: 200.0 });
        let anchor = layout.anchor_point();
        let right_original = layout
            .layouts
            .iter()
            .find(|slot| slot.element == HudElement::Right)
            .expect("layout expected to have a right hud element")
            .clone();

        // if the above assertions succeed, these should too.
        let flattened = Layout::Version1(Box::new(layout)).flatten();
        assert_eq!(flattened.size, Point { x: 300.0, y: 300.0 });
        assert_eq!(flattened.bg_size, Point { x: 300.0, y: 300.0 });
        assert_eq!(flattened.font_size, 18.5);
        assert_eq!(flattened.anchor, anchor);
        assert_eq!(flattened.slots[0].bg_size, Point { x: 100.0, y: 100.0 });
        let right_slot = flattened
            .slots
            .iter()
            .find(|slot| slot.element == HudElement::Right)
            .expect("the right slot must be present");
        let slot_center = Point {
            x: flattened.anchor.x + (right_original.offset.x * flattened.global_scale),
            y: flattened.anchor.y + (right_original.offset.y * flattened.global_scale),
        };
        assert_eq!(right_slot.center, slot_center);
    }
}
