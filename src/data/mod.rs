#![allow(non_snake_case, non_camel_case_types)]
//! This mod defines types that allow us to connect Object Categorization
//! Framework keywords to our internal item categorizations.

pub mod ammo;
pub mod armor;
pub mod base;
pub mod color;
pub mod game_enums;
pub mod huditem;
pub mod icons;
pub mod item_cache;
pub mod potion;
pub mod shout;
pub mod spell;
pub mod weapon;

use cxx::{CxxString, CxxVector};

use self::ammo::AmmoType;
pub use self::base::{BaseType, Proxy};
use self::color::*;
pub use self::huditem::HudItem;
use self::potion::PotionType;
use self::shout::ShoutVariant;
pub use self::spell::SpellData;
use crate::plugin::{
    healthPotionCount, magickaPotionCount, staminaPotionCount, Color, ItemCategory,
};

// ---------- Designed for C++ to call.

pub fn empty_huditem() -> Box<HudItem> {
    Box::<HudItem>::default()
}

pub fn hud_item_from_keywords(
    category: ItemCategory,
    keywords_ffi: &CxxVector<CxxString>,
    bytes_ffi: &CxxVector<u8>,
    form_string: String,
    count: u32,
    twohanded: bool,
) -> Box<HudItem> {
    // #[allow(clippy::map_clone)]
    let name_bytes: Vec<u8> = bytes_ffi.iter().copied().collect();
    let keywords: Vec<String> = keywords_ffi.iter().map(|xs| xs.to_string()).collect();
    let result = HudItem::from_keywords(
        category,
        keywords,
        name_bytes,
        form_string,
        count,
        twohanded,
    );
    Box::new(result)
}

pub fn fill_out_spell_data(
    effect: i32,
    resist: i32,
    twohanded: bool,
    school: i32,
    level: u32,
    archetype: i32,
    formspec: String,
) -> Box<SpellData> {
    let result = SpellData::from_game_data(
        effect, resist, twohanded, school, level, archetype, formspec,
    );
    Box::new(result)
}

pub fn magic_from_spelldata(
    which: ItemCategory,
    #[allow(clippy::boxed_local)] spelldata: Box<SpellData>, // this is coming from C++
    bytes_ffi: &CxxVector<u8>,
    form_string: String,
    count: u32,
) -> Box<HudItem> {
    let name_bytes: Vec<u8> = bytes_ffi.iter().map(|xs| *xs).collect();
    let data = *spelldata; // unbox

    let kind = match which {
        ItemCategory::Scroll => BaseType::create_scroll(data),
        ItemCategory::Spell => BaseType::create_spell(data),
        ItemCategory::Shout => BaseType::create_shout(data),
        _ => BaseType::create_spell(data),
    };
    let result = HudItem::preclassified(name_bytes, form_string, count, kind);
    Box::new(result)
}

pub fn simple_from_formdata(
    kind: ItemCategory,
    bytes_ffi: &CxxVector<u8>,
    form_string: String,
) -> Box<HudItem> {
    let name_bytes: Vec<u8> = bytes_ffi.iter().map(|xs| *xs).collect();
    let classification = match kind {
        ItemCategory::HandToHand => BaseType::HandToHand,
        ItemCategory::Light => BaseType::Light,
        ItemCategory::Power => BaseType::Power,
        ItemCategory::Food => BaseType::Food(base::FoodType::Fruit),
        ItemCategory::Shout => BaseType::Shout(ShoutVariant::default()),
        _ => BaseType::Empty,
    };
    let result = HudItem::preclassified(name_bytes, form_string, 1, classification);
    Box::new(result)
}

pub fn potion_from_formdata(
    is_poison: bool,
    effect: i32,
    count: u32,
    bytes_ffi: &CxxVector<u8>,
    form_string: String,
) -> Box<HudItem> {
    let name_bytes: Vec<u8> = bytes_ffi.iter().map(|xs| *xs).collect();
    let kind = PotionType::from_effect(is_poison, effect.into());
    let result = HudItem::preclassified(name_bytes, form_string, count, BaseType::Potion(kind));
    Box::new(result)
}

pub fn make_base_ammo(count: u32, bytes_ffi: &CxxVector<u8>, form_string: String) -> Box<HudItem> {
    let name_bytes: Vec<u8> = bytes_ffi.iter().map(|xs| *xs).collect();
    let kind = AmmoType::OCF_AmmoTypeArrow(InvColor::default());
    let result = HudItem::preclassified(name_bytes, form_string, count, BaseType::Ammo(kind));
    Box::new(result)
}

pub fn make_unarmed_proxy() -> Box<HudItem> {
    Box::new(HudItem::make_unarmed_proxy())
}

pub fn make_magicka_proxy() -> HudItem {
    let count = magickaPotionCount();
    HudItem::preclassified(
        "Best Magicka".as_bytes().to_vec(),
        "magicka_proxy".to_string(),
        count,
        BaseType::PotionProxy(Proxy::Magicka),
    )
}

pub fn make_health_proxy() -> HudItem {
    let count = healthPotionCount();
    HudItem::preclassified(
        "Best Health".as_bytes().to_vec(),
        "health_proxy".to_string(),
        count,
        BaseType::PotionProxy(Proxy::Health),
    )
}

pub fn make_stamina_proxy() -> HudItem {
    let count = staminaPotionCount();
    HudItem::preclassified(
        "Best Stamina".as_bytes().to_vec(),
        "stamina_proxy".to_string(),
        count,
        BaseType::PotionProxy(Proxy::Stamina),
    )
}

// ---------- Things that have icons also have fallbacks.

pub trait HasIcon {
    fn color(&self) -> Color;
    fn icon_file(&self) -> String;
    fn icon_fallback(&self) -> String;
}

pub trait HasKeywords {
    fn classify(keywords: Vec<String>, twohanded: bool) -> Self;
}

// ---------- Unclear this needs to be a trait.

pub trait IsHudItem {
    // TODO fold most of these up into base type
    fn count_matters(&self) -> bool;
    fn is_ammo(&self) -> bool;
    fn is_armor(&self) -> bool;
    fn is_magic(&self) -> bool;
    fn is_potion(&self) -> bool;
    fn is_power(&self) -> bool;
    fn is_spell(&self) -> bool;
    fn is_utility(&self) -> bool;
    fn is_weapon(&self) -> bool;
    fn is_one_handed(&self) -> bool;
    fn left_hand_ok(&self) -> bool;
    fn right_hand_ok(&self) -> bool;
    fn two_handed(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::weapon::{WeaponEquipType, WeaponType};

    #[test]
    fn can_classify_huditem() {
        let input = vec![
            "OCF_InvColorBlood".to_string(),
            "WeapTypeGreatsword".to_string(),
            "WeapTypeHalberd".to_string(),
            "Weapon".to_string(),
            "OCF_WeapTypeHalberd2H".to_string(),
        ];

        let name_bytes = "Placeholder".as_bytes().to_vec();
        let item = HudItem::from_keywords(
            ItemCategory::Weapon,
            input,
            name_bytes,
            "placeholder".to_string(),
            2,
            true,
        );

        assert_eq!(
            item.name(),
            "Placeholder".to_string(),
            "handled the name bytes correctly"
        );
        assert_eq!(
            *item.kind(),
            BaseType::Weapon(WeaponType::Halberd(
                WeaponEquipType::TwoHanded,
                InvColor::Blood
            )),
            "classified weapon as halberd"
        );
    }
}
