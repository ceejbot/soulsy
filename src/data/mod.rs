#![allow(non_snake_case, non_camel_case_types)]
//! This module defines types that allow us to connect Object Categorization
//! Framework keywords to our internal item categorizations.

pub mod ammo;
pub mod armor;
pub mod base;
pub mod color;
pub mod food;
pub mod game_enums;
pub mod huditem;
pub mod item_cache;
pub mod keywords;
pub mod magic;
pub mod potion;
pub mod power;
pub mod shout;
pub mod spell;
pub mod weapon;

use cxx::{CxxString, CxxVector};
use enumset::{EnumSet, EnumSetType};

pub use self::base::{BaseType, Proxy};
use self::color::*;
pub use self::huditem::HudItem;
use self::potion::PotionType;
use self::power::PowerType;
use self::shout::ShoutType;
use self::spell::SpellType;
pub use super::magic::SpellData;
use crate::images::icons::Icon;
#[cfg(not(test))]
use crate::plugin::{healthPotionCount, magickaPotionCount, staminaPotionCount};
use crate::plugin::{Color, ItemCategory};

// ---------- Designed for C++ to call.

pub fn empty_huditem() -> Box<HudItem> {
    Box::<HudItem>::default()
}

pub fn hud_item_from_keywords(
    category: ItemCategory,
    keywords_ffi: &CxxVector<CxxString>,
    name: String,
    form_string: String,
    count: u32,
    twohanded: bool,
) -> Box<HudItem> {
    // #[allow(clippy::map_clone)]
    let keywords: Vec<String> = keywords_ffi.iter().map(|xs| xs.to_string()).collect();
    let result = HudItem::from_keywords(category, keywords, name, form_string, count, twohanded);
    Box::new(result)
}

pub fn categorize_shout(
    keywords_ffi: &CxxVector<CxxString>,
    name: String,
    form_string: String,
) -> Box<HudItem> {
    let keywords: Vec<String> = keywords_ffi.iter().map(|xs| xs.to_string()).collect();
    let kind = BaseType::Shout(ShoutType::new(keywords));
    let result = HudItem::preclassified(name, form_string, 1, kind);
    Box::new(result)
}

pub fn fill_out_spell_data(
    hostile: bool,
    resist: i32,
    twohanded: bool,
    school: i32,
    level: u32,
    archetype: i32,
) -> Box<SpellData> {
    let result = SpellData::new(hostile, resist, twohanded, school, level, archetype);
    Box::new(result)
}

pub fn magic_from_spelldata(
    which: ItemCategory,
    #[allow(clippy::boxed_local)] spelldata: Box<SpellData>, // this is coming from C++
    keywords_ffi: &CxxVector<CxxString>,
    name: String,
    form_string: String,
    count: u32,
) -> Box<HudItem> {
    let data = *spelldata; // unbox
    let keywords: Vec<String> = keywords_ffi.iter().map(|xs| xs.to_string()).collect();

    let kind = match which {
        ItemCategory::Scroll => BaseType::Scroll(SpellType::new(data, keywords)),
        ItemCategory::Spell => BaseType::Spell(SpellType::new(data, keywords)),
        ItemCategory::Shout => BaseType::Shout(ShoutType::new(keywords)),
        _ => BaseType::Spell(SpellType::new(data, keywords)),
    };
    let result = HudItem::preclassified(name, form_string, count, kind);
    Box::new(result)
}

pub fn simple_from_formdata(kind: ItemCategory, name: String, form_string: String) -> Box<HudItem> {
    let classification = match kind {
        ItemCategory::Book => BaseType::Book,
        ItemCategory::HandToHand => BaseType::HandToHand,
        ItemCategory::Lantern => BaseType::Light(base::LightType::Lantern),
        ItemCategory::Torch => BaseType::Light(base::LightType::Torch),
        ItemCategory::Power => BaseType::Power(PowerType::default()),
        ItemCategory::Food => BaseType::Food(super::food::FoodType::default()),
        ItemCategory::Shout => BaseType::Shout(ShoutType::default()),
        _ => BaseType::Empty,
    };
    let result = HudItem::preclassified(name, form_string, 1, classification);
    Box::new(result)
}

pub fn potion_from_formdata(
    is_poison: bool,
    effect: i32,
    count: u32,
    name: String,
    form_string: String,
) -> Box<HudItem> {
    let kind = PotionType::from_effect(is_poison, effect.into());
    let result = HudItem::preclassified(name, form_string, count, BaseType::Potion(kind));
    Box::new(result)
}

pub fn make_magicka_proxy() -> HudItem {
    #[cfg(test)]
    let count = 10;
    #[cfg(not(test))]
    let count = magickaPotionCount();
    HudItem::preclassified(
        "Best Magicka".to_string(),
        "magicka_proxy".to_string(),
        count,
        BaseType::PotionProxy(Proxy::Magicka),
    )
}

pub fn make_health_proxy() -> HudItem {
    #[cfg(test)]
    let count = 8;
    #[cfg(not(test))]
    let count = healthPotionCount();
    HudItem::preclassified(
        "Best Health".to_string(),
        "health_proxy".to_string(),
        count,
        BaseType::PotionProxy(Proxy::Health),
    )
}

pub fn make_stamina_proxy() -> HudItem {
    #[cfg(test)]
    let count = 11;
    #[cfg(not(test))]
    let count = staminaPotionCount();
    HudItem::preclassified(
        "Best Stamina".to_string(),
        "stamina_proxy".to_string(),
        count,
        BaseType::PotionProxy(Proxy::Stamina),
    )
}

/// Contract adhered to by anything that has an icon.
pub trait HasIcon {
    fn color(&self) -> Color;
    fn icon(&self) -> &Icon;
}

/// Trait for turning keywords into an item with an icon.
pub trait HasKeywords {
    fn classify(name: &str, keywords: Vec<String>, twohanded: bool) -> Self;
}

// Generic convert keywords to an enum set.
pub fn strings_to_enumset<T: EnumSetType + for<'a> TryFrom<&'a str>>(
    tags: &[String],
) -> EnumSet<T> {
    let mut tagset: EnumSet<T> = EnumSet::new();
    tags.iter().for_each(|xs| {
        if let Ok(subtype) = T::try_from(xs.as_str()) {
            tagset.insert(subtype);
        } else {
            log::trace!("Unknown keyword: '{xs}';");
        }
    });
    tagset
}

// ---------- Tests. I hear they're good.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::weapon::{WeaponEquipType, WeaponType};
    use crate::images::icons::Icon;

    #[test]
    fn can_classify_huditem() {
        let kwds = vec![
            "OCF_InvColorBlood".to_string(),
            "WeapTypeHalberd".to_string(),
            "OCF_WeapTypeHalberd2H".to_string(),
        ];

        let name = "Placeholder".to_string();
        let item = HudItem::from_keywords(
            ItemCategory::Weapon,
            kwds,
            name,
            "placeholder|0xcafed00d".to_string(),
            2,
            true,
        );

        assert_eq!(
            item.name(),
            "Placeholder".to_string(),
            "handled the name bytes correctly"
        );
        let wt = WeaponType::new(
            Icon::WeaponHalberd,
            InvColor::Blood,
            WeaponEquipType::TwoHanded,
        );
        assert_eq!(*item.kind(), BaseType::Weapon(wt));
    }
}
