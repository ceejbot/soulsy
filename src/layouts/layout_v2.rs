//! Version 2 of the layout schema. This version is the only one that gets
//! new features-- v1 is to be retired gently over time.

use eyre::{Context, Result};
use serde::{Deserialize, Serialize};

use super::shared::*;
use crate::plugin::{
    Align, Color, HudElement, LayoutFlattened, MeterKind, Point, SlotFlattened, TextFlattened,
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
    anchor: Option<Point>,
    #[serde(default, deserialize_with = "deserialize_named_anchor")]
    anchor_name: NamedAnchor,
    size: Point,
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
    meter: Option<MeterElement>,
    poison: Option<PoisonElement>,
}

impl HudLayout2 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fallback() -> Self {
        let buf =
            include_str!("../../installer/core/SKSE/plugins/soulsy_layouts/SoulsyHUD_default.toml");
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
        super::anchor_point(
            self.global_scale,
            &self.size,
            &self.anchor_name,
            self.anchor.as_ref(),
        )
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

        let poison = slot.poison.clone().unwrap_or_default();
        let poison_image = poison.indicator.svg;
        let poison_size = poison.indicator.size.scale(self.global_scale);
        let poison_color = poison.indicator.color;
        let poison_center = center.translate(&poison.offset.scale(self.global_scale));

        let meter = slot.meter.clone().unwrap_or_default();
        let meter_kind = meter.kind;
        let meter_center = center.translate(&meter.offset.scale(self.global_scale));
        let meter_size = meter.size.scale(self.global_scale);
        let meter_empty_image = meter.empty_image;
        let meter_empty_color = meter.empty_color;
        let meter_fill_image = meter.fill_image;
        let meter_fill_color = meter.fill_color;

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
            poison_size,
            poison_image,
            poison_color,
            poison_center,
            meter_kind,
            meter_center,
            meter_size,
            meter_empty_image,
            meter_empty_color,
            meter_fill_image,
            meter_fill_color,
            text,
        }
    }

    fn flatten_text(&self, text: &TextElement, center: &Point) -> TextFlattened {
        TextFlattened {
            anchor: center.translate(&text.offset.scale(self.global_scale)),
            color: text.color.clone(),
            alignment: text.alignment,
            contents: text.contents.clone(),
            font_size: text.font_size * self.global_scale,
            wrap_width: text.wrap_width,
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
            size: Point::origin(),
            color: Color::invisible(),
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
            offset: Point::origin(),
            size: Point::origin(),
            color: Color::invisible(),
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
    bounds: Option<Point>,
    #[serde(default)]
    wrap_width: f32,
}

// TODO TODO TODO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MeterElement {
    #[serde(default, deserialize_with = "deserialize_meter_kind")]
    kind: MeterKind,
    offset: Point,
    size: Point,
    empty_image: String,
    empty_color: Color,
    fill_image: String,
    fill_color: Color,
}

impl Default for MeterElement {
    fn default() -> Self {
        MeterElement {
            kind: MeterKind::None,
            offset: Point::origin(),
            size: Point::origin(),
            empty_image: String::new(),
            empty_color: Color::invisible(),
            fill_image: String::new(),
            fill_color: Color::invisible(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PoisonElement {
    offset: Point,
    indicator: ImageElement,
}

impl Default for PoisonElement {
    fn default() -> Self {
        PoisonElement {
            offset: Point::origin(),
            indicator: ImageElement::default(),
        }
    }
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

    #[test]
    fn default_layout_valid() {
        let buf =
            include_str!("../../installer/core/SKSE/plugins/soulsy_layouts/SoulsyHUD_default.toml");
        match toml::from_str::<HudLayout2>(buf) {
            Ok(v) => {
                assert_eq!(v.anchor_point().x, 150.0);
            }
            Err(e) => {
                eprintln!("{e:#?}");
                unreachable!();
            }
        }
        let builtin: Layout = toml::from_str(buf).expect("layout should be valid toml");
        match builtin {
            Layout::Version1(_) => unreachable!(),
            Layout::Version2(v) => {
                assert_eq!(v.anchor_name, NamedAnchor::BottomLeft);
                assert_eq!(v.anchor_point().x, 150.0);
                assert_eq!(v.anchor_point().y, 1290.0);
                let right_poison = v
                    .right
                    .poison
                    .as_ref()
                    .expect("the right slot should have a poison indicator");
                assert_eq!(
                    right_poison.indicator.svg,
                    "../icons/indicator_poison.svg".to_string()
                );
                let _left_poison = v
                    .left
                    .poison
                    .as_ref()
                    .expect("the left slot should have a poison indicator");

                let flattened = Layout::Version2(v.clone()).flatten();
                assert_eq!(flattened.anchor, v.anchor_point());
                let right_slot = flattened
                    .slots
                    .iter()
                    .find(|slot| slot.element == HudElement::Right)
                    .expect("the flattened layout needs to have a right slot");
                assert_eq!(right_slot.poison_image, right_poison.indicator.svg);
                assert_eq!(right_slot.poison_color, right_poison.indicator.color);
                let slot_center = Point {
                    x: flattened.anchor.x + (v.right.offset.x * flattened.global_scale),
                    y: flattened.anchor.y + (v.right.offset.y * flattened.global_scale),
                };
                assert_eq!(right_slot.center, slot_center);
                assert_eq!(
                    right_slot.poison_center,
                    slot_center.translate(&right_poison.offset)
                );
            }
        }
    }

    #[test]
    fn centered_layout_valid() {
        let buf = include_str!(
            "../../installer/core/SKSE/plugins/soulsy_layouts/SoulsyHUD_centered.toml"
        );
        let centered: HudLayout2 = toml::from_str(buf).expect("layout should be valid toml");
        assert_eq!(centered.anchor_name, NamedAnchor::Center);
        assert_eq!(centered.anchor_point().x, 1720.0);
        assert_eq!(centered.anchor_point().y, 720.0);
    }

    #[test]
    fn topleft_layout_valid() {
        let buf =
            include_str!("../../installer/core/SKSE/plugins/soulsy_layouts/SoulsyHUD_topleft.toml");
        let layout: HudLayout2 = toml::from_str(buf).expect("layout should be valid toml");
        assert_eq!(layout.anchor_name, NamedAnchor::None);
        assert_eq!(layout.anchor_point().x, 150.0);
        assert_eq!(layout.anchor_point().y, 150.0);
    }

    #[test]
    fn minimal_layout_valid() {
        let data =
            include_str!("../../installer/core/SKSE/plugins/soulsy_layouts/SoulsyHUD_minimal.toml");
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
    fn i18n_layout_valid() {
        let data =
            include_str!("../../installer/core/SKSE/plugins/soulsy_layouts/SoulsyHUD_i18n.toml");
        let specific: HudLayout2 = match toml::from_str::<HudLayout2>(data) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("i18n layout is invalid as a v2 layout.");
                eprintln!("{e:#}");
                unreachable!();
            }
        };
        assert_eq!(specific.anchor_name, NamedAnchor::BottomLeft);
        let minimal: Layout =
            toml::from_str(data).expect("serde should figure out which layout schema");
        match minimal {
            Layout::Version1(_) => unreachable!(),
            Layout::Version2(v) => {
                assert_eq!(v.anchor_name, NamedAnchor::BottomLeft);
                assert_eq!(v.anchor_point().x, 150.0);
                assert_eq!(v.anchor_point().y, 1290.0);
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
        let data = include_str!("../../tests/fixtures/layout-v2.toml");
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
        assert_eq!(layout.right.text.len(), right_slot.text.len());
        assert_eq!(
            layout.right.text[0].font_size * layout.global_scale,
            right_slot.text[0].font_size
        );
    }

    #[test]
    fn charge_meters() {
        let data = include_str!("../../tests/fixtures/layout-v2.toml");
        let layout: HudLayout2 =
            toml::from_str(data).expect("v2 text fixture should be valid toml");

        assert!(layout.power.meter.is_none());
        assert!(layout.utility.meter.is_none());
        assert!(layout.right.meter.is_some());
        assert!(layout.left.meter.is_some());

        let rmeter = layout
            .right
            .meter
            .clone()
            .expect("we just asserted this exists");
        assert_eq!(rmeter.kind, MeterKind::Vertical);

        let lmeter = layout
            .left
            .meter
            .clone()
            .expect("we just asserted this exists");
        assert_eq!(lmeter.kind, MeterKind::Horizontal);

        let flattened = Layout::Version2(Box::new(layout.clone())).flatten();
        let rflat = flattened
            .slots
            .iter()
            .find(|slot| slot.element == HudElement::Right)
            .expect("the right slot must be present");
        let lflat = flattened
            .slots
            .iter()
            .find(|slot| slot.element == HudElement::Left)
            .expect("the right slot must be present");

        assert_eq!(rflat.meter_kind, rmeter.kind);
        assert_eq!(lflat.meter_kind, lmeter.kind);

        assert_eq!(
            rflat.meter_center,
            rflat
                .center
                .translate(&rmeter.offset.scale(layout.global_scale))
        );
        assert_eq!(
            lflat.meter_center,
            lflat
                .center
                .translate(&lmeter.offset.scale(layout.global_scale))
        );
    }
}
