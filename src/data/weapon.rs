#![allow(non_snake_case, non_camel_case_types)]

use std::fmt::Display;

use enumset::{enum_set, EnumSet, EnumSetType};
use strum::EnumString;

use super::color::InvColor;
use super::icons::Icon;
use super::{HasIcon, HasKeywords};
use crate::plugin::Color;

#[derive(Clone, Debug, EnumString, Eq, Hash, PartialEq)]
pub enum WeaponEquipType {
    TwoHanded,
    LeftHand,
    RightHand,
    EitherHand,
}

// These are weapon types we have icons for. All OCF tags map
// into one of these. Couldn't see a gain for splitting these up
// into one/two handed types, so I left them together.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum WeaponType {
    AxeOneHanded(WeaponEquipType, InvColor),
    AxeTwoHanded(WeaponEquipType, InvColor),
    BowShort(WeaponEquipType, InvColor),
    Bow(WeaponEquipType, InvColor),
    Claw(WeaponEquipType, InvColor),
    Crossbow(WeaponEquipType, InvColor),
    Dagger(WeaponEquipType, InvColor),
    FishingRod(WeaponEquipType, InvColor),
    Flail(WeaponEquipType, InvColor),
    Grenade(WeaponEquipType, InvColor),
    Gun(WeaponEquipType, InvColor),
    Halberd(WeaponEquipType, InvColor),
    Hammer(WeaponEquipType, InvColor),
    HandToHand(WeaponEquipType, InvColor),
    Katana(WeaponEquipType, InvColor),
    Lance(WeaponEquipType, InvColor),
    Mace(WeaponEquipType, InvColor),
    Pickaxe(WeaponEquipType, InvColor),
    Quarterstaff(WeaponEquipType, InvColor),
    Rapier(WeaponEquipType, InvColor),
    Scythe(WeaponEquipType, InvColor),
    Staff(WeaponEquipType, InvColor),
    SwordOneHanded(WeaponEquipType, InvColor),
    SwordTwoHanded(WeaponEquipType, InvColor),
    WeaponDefault(WeaponEquipType, InvColor),
    Whip(WeaponEquipType, InvColor),
    WoodAxe(WeaponEquipType, InvColor),
}

impl WeaponType {
    pub fn is_one_handed(&self) -> bool {
        let two_handed = match self {
            WeaponType::AxeOneHanded(t, _) => matches!(t, WeaponEquipType::TwoHanded),
            WeaponType::AxeTwoHanded(t, _) => matches!(t, WeaponEquipType::TwoHanded),
            WeaponType::BowShort(t, _) => matches!(t, WeaponEquipType::TwoHanded),
            WeaponType::Bow(t, _) => matches!(t, WeaponEquipType::TwoHanded),
            WeaponType::Claw(t, _) => matches!(t, WeaponEquipType::TwoHanded),
            WeaponType::Crossbow(t, _) => matches!(t, WeaponEquipType::TwoHanded),
            WeaponType::Dagger(t, _) => matches!(t, WeaponEquipType::TwoHanded),
            WeaponType::Flail(t, _) => matches!(t, WeaponEquipType::TwoHanded),
            WeaponType::Grenade(t, _) => matches!(t, WeaponEquipType::TwoHanded),
            WeaponType::Gun(t, _) => matches!(t, WeaponEquipType::TwoHanded),
            WeaponType::Halberd(t, _) => matches!(t, WeaponEquipType::TwoHanded),
            WeaponType::Hammer(t, _) => matches!(t, WeaponEquipType::TwoHanded),
            WeaponType::HandToHand(t, _) => matches!(t, WeaponEquipType::TwoHanded),
            WeaponType::Katana(t, _) => matches!(t, WeaponEquipType::TwoHanded),
            WeaponType::Lance(t, _) => matches!(t, WeaponEquipType::TwoHanded),
            WeaponType::Mace(t, _) => matches!(t, WeaponEquipType::TwoHanded),
            WeaponType::Quarterstaff(t, _) => matches!(t, WeaponEquipType::TwoHanded),
            WeaponType::Rapier(t, _) => matches!(t, WeaponEquipType::TwoHanded),
            WeaponType::Scythe(t, _) => matches!(t, WeaponEquipType::TwoHanded),
            WeaponType::Staff(t, _) => matches!(t, WeaponEquipType::TwoHanded),
            WeaponType::SwordOneHanded(t, _) => matches!(t, WeaponEquipType::TwoHanded),
            WeaponType::SwordTwoHanded(t, _) => matches!(t, WeaponEquipType::TwoHanded),
            WeaponType::WeaponDefault(t, _) => matches!(t, WeaponEquipType::TwoHanded),
            WeaponType::Whip(t, _) => matches!(t, WeaponEquipType::TwoHanded),
            WeaponType::FishingRod(t, _) => matches!(t, WeaponEquipType::TwoHanded),
            WeaponType::Pickaxe(t, _) => matches!(t, WeaponEquipType::TwoHanded),
            WeaponType::WoodAxe(t, _) => matches!(t, WeaponEquipType::TwoHanded),
        };
        !two_handed
    }
}

impl HasKeywords for WeaponType {
    fn classify(name: &str, keywords: Vec<String>, twohanded: bool) -> Self {
        let color = super::base::color_from_keywords(&keywords);

        let equiptype = if twohanded {
            WeaponEquipType::TwoHanded
        } else {
            WeaponEquipType::EitherHand
        };

        let tags: Vec<WeaponTag> = keywords
            .iter()
            .filter_map(|xs| {
                if let Ok(subtype) = WeaponTag::try_from(xs.as_str()) {
                    Some(subtype)
                } else {
                    None
                }
            })
            .collect();

        let maybe_kind = tags.iter().find_map(|subtype| {
            // Each weapon subtype has an enumset for all the keywords that get
            // mapped to that subtype. If we have a match, we have identified
            // the subtype. Our subtypes are the ones we have icons for right now.
            // We retain the ability to add icons later.
            if BATTLEAXES.contains(*subtype) {
                Some(WeaponType::AxeTwoHanded(equiptype.clone(), color.clone()))
            } else if BOWS.contains(*subtype) {
                Some(WeaponType::Bow(equiptype.clone(), color.clone()))
            } else if CROSSBOWS.contains(*subtype) {
                Some(WeaponType::Crossbow(equiptype.clone(), color.clone()))
            } else if DAGGERS.contains(*subtype) {
                Some(WeaponType::Dagger(equiptype.clone(), color.clone()))
            } else if GREATSWORDS.contains(*subtype) {
                Some(WeaponType::SwordTwoHanded(equiptype.clone(), color.clone()))
            } else if GUNS.contains(*subtype) {
                Some(WeaponType::Gun(equiptype.clone(), color.clone()))
            } else if HAMMERS.contains(*subtype) {
                Some(WeaponType::Hammer(equiptype.clone(), color.clone()))
            } else if HALBERDS.contains(*subtype) {
                Some(WeaponType::Halberd(equiptype.clone(), color.clone()))
            } else if HAND_TO_HAND.contains(*subtype) {
                Some(WeaponType::HandToHand(
                    WeaponEquipType::EitherHand,
                    color.clone(),
                ))
            } else if KATANAS.contains(*subtype) {
                Some(WeaponType::Katana(equiptype.clone(), color.clone()))
            } else if LANCES.contains(*subtype) {
                Some(WeaponType::Lance(equiptype.clone(), color.clone()))
            } else if MACES.contains(*subtype) {
                Some(WeaponType::Mace(equiptype.clone(), color.clone()))
            } else if QUARTERSTAVES.contains(*subtype) {
                Some(WeaponType::Quarterstaff(equiptype.clone(), color.clone()))
            } else if SCYTHES.contains(*subtype) {
                Some(WeaponType::Scythe(equiptype.clone(), color.clone()))
            } else if STAVES.contains(*subtype) {
                Some(WeaponType::Staff(equiptype.clone(), color.clone()))
            } else if SWORDS.contains(*subtype) {
                Some(WeaponType::SwordOneHanded(equiptype.clone(), color.clone()))
            } else if WARAXES.contains(*subtype) {
                Some(WeaponType::AxeOneHanded(equiptype.clone(), color.clone()))
            } else if WHIPS.contains(*subtype) {
                Some(WeaponType::Whip(equiptype.clone(), color.clone()))
            } else if matches!(subtype, WeaponTag::OCF_WeapTypePickaxe1H) {
                Some(WeaponType::Pickaxe(equiptype.clone(), color.clone()))
            } else if matches!(
                subtype,
                WeaponTag::OCF_WeapTypeWoodaxe1H | WeaponTag::OCF_WeapTypeWoodHatchet1H
            ) {
                Some(WeaponType::WoodAxe(equiptype.clone(), color.clone()))
            } else if matches!(subtype, WeaponTag::OCF_WeapTypeFishingRod1H) {
                Some(WeaponType::FishingRod(equiptype.clone(), color.clone()))
            } else if matches!(
                subtype,
                WeaponTag::WeapTypeClaw | WeaponTag::OCF_WeapTypeClaw1H
            ) {
                Some(WeaponType::Claw(equiptype.clone(), color.clone()))
            } else if matches!(subtype, WeaponTag::WeapTypeFlail) {
                Some(WeaponType::Flail(equiptype.clone(), color.clone()))
            } else if matches!(subtype, WeaponTag::WeapTypeStaff) {
                Some(WeaponType::Staff(equiptype.clone(), color.clone()))
            } else if matches!(subtype, WeaponTag::WAF_WeapTypeGrenade) {
                Some(WeaponType::Grenade(equiptype.clone(), color.clone()))
            } else {
                None
            }
        });

        if let Some(kind) = maybe_kind {
            kind
        } else {
            // Now we look for more general tags. Fortunately these keyword lists are short.
            let maybe_kind: Option<WeaponType> = keywords.iter().find_map(|xs| {
                let Ok(subtype) = WeaponTag::try_from(xs.as_str()) else {
                    return None;
                };
                if FALLBACK_TYPES.contains(subtype) {
                    match subtype {
                        WeaponTag::TwoHandSword => {
                            Some(WeaponType::SwordTwoHanded(equiptype.clone(), color.clone()))
                        }
                        WeaponTag::Bow => Some(WeaponType::Bow(equiptype.clone(), color.clone())),
                        WeaponTag::WeapTypeBow => {
                            Some(WeaponType::Bow(equiptype.clone(), color.clone()))
                        }
                        WeaponTag::WeapTypeDagger => {
                            Some(WeaponType::Dagger(equiptype.clone(), color.clone()))
                        }
                        WeaponTag::OneHandDagger => {
                            Some(WeaponType::Dagger(equiptype.clone(), color.clone()))
                        }
                        WeaponTag::WeapTypeGreatsword => {
                            Some(WeaponType::SwordTwoHanded(equiptype.clone(), color.clone()))
                        }
                        WeaponTag::WeapTypeMace => {
                            Some(WeaponType::Mace(equiptype.clone(), color.clone()))
                        }
                        WeaponTag::WeapTypeSword => {
                            Some(WeaponType::SwordOneHanded(equiptype.clone(), color.clone()))
                        }
                        WeaponTag::OneHandSword => {
                            Some(WeaponType::SwordOneHanded(equiptype.clone(), color.clone()))
                        }
                        WeaponTag::WeapTypeWarAxe => {
                            Some(WeaponType::AxeOneHanded(equiptype.clone(), color.clone()))
                        }
                        WeaponTag::TwoHandAxe => {
                            Some(WeaponType::AxeTwoHanded(equiptype.clone(), color.clone()))
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            });
            if let Some(kind) = maybe_kind {
                kind
            } else {
                log::warn!(
                    "We couldn't classify this weapon! name='{name}'; keywords: {keywords:?}"
                );
                WeaponType::default()
            }
        }
    }
}

impl Default for WeaponType {
    fn default() -> Self {
        WeaponType::WeaponDefault(WeaponEquipType::EitherHand, InvColor::default())
    }
}

impl Display for WeaponType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WeaponType::AxeOneHanded(_, _) => write!(f, "axe_one_handed"),
            WeaponType::AxeTwoHanded(_, _) => write!(f, "axe_two_handed"),
            WeaponType::BowShort(_, _) => write!(f, "bow_short"),
            WeaponType::Bow(_, _) => write!(f, "bow"),
            WeaponType::Claw(_, _) => write!(f, "claw"),
            WeaponType::Crossbow(_, _) => write!(f, "crossbow"),
            WeaponType::Dagger(_, _) => write!(f, "dagger"),
            WeaponType::FishingRod(_, _) => write!(f, "fishingrod"),
            WeaponType::Flail(_, _) => write!(f, "flail"),
            WeaponType::Grenade(_, _) => write!(f, "grenade"),
            WeaponType::Gun(_, _) => write!(f, "gun"),
            WeaponType::Halberd(_, _) => write!(f, "halberd"),
            WeaponType::Hammer(_, _) => write!(f, "hammer"),
            WeaponType::HandToHand(_, _) => write!(f, "hand_to_hand"),
            WeaponType::Katana(_, _) => write!(f, "katana"),
            WeaponType::Lance(_, _) => write!(f, "lance"),
            WeaponType::Mace(_, _) => write!(f, "mace"),
            WeaponType::Pickaxe(_, _) => write!(f, "pickaxe"),
            WeaponType::Quarterstaff(_, _) => write!(f, "quarterstaff"),
            WeaponType::Rapier(_, _) => write!(f, "rapier"),
            WeaponType::Scythe(_, _) => write!(f, "scythe"),
            WeaponType::Staff(_, _) => write!(f, "staff"),
            WeaponType::SwordOneHanded(_, _) => write!(f, "sword_one_handed"),
            WeaponType::SwordTwoHanded(_, _) => write!(f, "sword_two_handed"),
            WeaponType::WeaponDefault(_, _) => write!(f, "weapon_default"),
            WeaponType::Whip(_, _) => write!(f, "whip"),
            WeaponType::WoodAxe(_, _) => write!(f, "woodaxe"),
        }
    }
}

impl HasIcon for WeaponType {
    fn color(&self) -> Color {
        match self {
            WeaponType::AxeOneHanded(_, c) => c.color(),
            WeaponType::AxeTwoHanded(_, c) => c.color(),
            WeaponType::BowShort(_, c) => c.color(),
            WeaponType::Bow(_, c) => c.color(),
            WeaponType::Claw(_, c) => c.color(),
            WeaponType::Crossbow(_, c) => c.color(),
            WeaponType::Dagger(_, c) => c.color(),
            WeaponType::FishingRod(_, c) => c.color(),
            WeaponType::Flail(_, c) => c.color(),
            WeaponType::Grenade(_, c) => c.color(),
            WeaponType::Gun(_, c) => c.color(),
            WeaponType::Halberd(_, c) => c.color(),
            WeaponType::Hammer(_, c) => c.color(),
            WeaponType::HandToHand(_, c) => c.color(),
            WeaponType::Katana(_, c) => c.color(),
            WeaponType::Lance(_, c) => c.color(),
            WeaponType::Mace(_, c) => c.color(),
            WeaponType::Pickaxe(_, c) => c.color(),
            WeaponType::Quarterstaff(_, c) => c.color(),
            WeaponType::Rapier(_, c) => c.color(),
            WeaponType::Scythe(_, c) => c.color(),
            WeaponType::Staff(_, c) => c.color(),
            WeaponType::SwordOneHanded(_, c) => c.color(),
            WeaponType::SwordTwoHanded(_, c) => c.color(),
            WeaponType::WeaponDefault(_, c) => c.color(),
            WeaponType::Whip(_, c) => c.color(),
            WeaponType::WoodAxe(_, c) => c.color(),
        }
    }

    fn icon_file(&self) -> String {
        match self {
            WeaponType::AxeOneHanded(_, _) => Icon::WeaponAxeOneHanded.icon_file(),
            WeaponType::AxeTwoHanded(_, _) => Icon::WeaponAxeTwoHanded.icon_file(),
            WeaponType::BowShort(_, _) => Icon::WeaponBowShort.icon_file(),
            WeaponType::Bow(_, _) => Icon::WeaponBow.icon_file(),
            WeaponType::Claw(_, _) => Icon::WeaponClaw.icon_file(),
            WeaponType::Crossbow(_, _) => Icon::WeaponCrossbow.icon_file(),
            WeaponType::Dagger(_, _) => Icon::WeaponDagger.icon_file(),
            WeaponType::FishingRod(_, _) => Icon::WeaponFishingRod.icon_file(),
            WeaponType::Flail(_, _) => Icon::WeaponFlail.icon_file(),
            WeaponType::Grenade(_, _) => Icon::WeaponGrenade.icon_file(),
            WeaponType::Gun(_, _) => Icon::WeaponGun.icon_file(),
            WeaponType::Halberd(_, _) => Icon::WeaponHalberd.icon_file(),
            WeaponType::Hammer(_, _) => Icon::WeaponHammer.icon_file(),
            WeaponType::HandToHand(_, _) => Icon::HandToHand.icon_file(),
            WeaponType::Katana(_, _) => Icon::WeaponKatana.icon_file(),
            WeaponType::Lance(_, _) => Icon::WeaponLance.icon_file(),
            WeaponType::Mace(_, _) => Icon::WeaponMace.icon_file(),
            WeaponType::Pickaxe(_, _) => Icon::WeaponPickaxe.icon_file(),
            WeaponType::Quarterstaff(_, _) => Icon::WeaponQuarterstaff.icon_file(),
            WeaponType::Rapier(_, _) => Icon::WeaponRapier.icon_file(),
            WeaponType::Scythe(_, _) => Icon::WeaponScythe.icon_file(),
            WeaponType::Staff(_, _) => Icon::WeaponStaff.icon_file(),
            WeaponType::SwordOneHanded(_, _) => Icon::WeaponSwordOneHanded.icon_file(),
            WeaponType::SwordTwoHanded(_, _) => Icon::WeaponSwordTwoHanded.icon_file(),
            WeaponType::WeaponDefault(_, _) => Icon::WeaponSwordOneHanded.icon_file(),
            WeaponType::Whip(_, _) => Icon::WeaponWhip.icon_file(),
            WeaponType::WoodAxe(_, _) => Icon::WeaponWoodAxe.icon_file(),
        }
    }

    fn icon_fallback(&self) -> String {
        "weapon_default.svg".to_string()
    }
}

// Enum sets to let us pluck out matches from keywords efficiently.

const FALLBACK_TYPES: EnumSet<WeaponTag> = enum_set!(
    WeaponTag::TwoHandSword
        | WeaponTag::WeapTypeBow
        | WeaponTag::Bow
        | WeaponTag::Crossbow
        | WeaponTag::WeapTypeDagger
        | WeaponTag::OneHandDagger
        | WeaponTag::WeapTypeGreatsword
        | WeaponTag::WeapTypeSword
        | WeaponTag::OneHandSword
        | WeaponTag::WeapTypeWarAxe
        | WeaponTag::WeapTypeMace
        | WeaponTag::TwoHandAxe
);

const BATTLEAXES: EnumSet<WeaponTag> = enum_set!(
    WeaponTag::WeapTypeAxeTwoHanded
        | WeaponTag::WeapTypeBattleaxe
        | WeaponTag::OCF_WeapTypeBattleaxe2H
        | WeaponTag::OCF_WeapTypeWarpick2H
);
const BOWS: EnumSet<WeaponTag> = enum_set!(
    WeaponTag::OCF_WeapTypeBlowgun2H
        | WeaponTag::OCF_WeapTypeBow
        | WeaponTag::OCF_WeapTypeBow2H
        | WeaponTag::OCF_WeapTypeGun
        | WeaponTag::OCF_WeapTypeSlingshot2H
        | WeaponTag::OCF_WeapTypeGreatbow2H
);
const CROSSBOWS: EnumSet<WeaponTag> = enum_set!(
    WeaponTag::WeapTypeCrossbow
        | WeaponTag::OCF_WeapTypeCrossbow
        | WeaponTag::OCF_WeapTypeCrossbow1H
        | WeaponTag::OCF_WeapTypeCrossbow2H
);
const DAGGERS: EnumSet<WeaponTag> = enum_set!(
    WeaponTag::OCF_WeapTypeChakram1H
        | WeaponTag::OCF_WeapTypeCutlery1H
        | WeaponTag::OCF_WeapTypeDagger1H
        | WeaponTag::OCF_WeapTypeKunai1H
        | WeaponTag::OCF_WeapTypeRevDagger1H
        | WeaponTag::OCF_WeapTypeSai1H
        | WeaponTag::OCF_WeapTypeShiv1H
        | WeaponTag::OCF_WeapTypeShuriken1H
        | WeaponTag::OCF_WeapTypeTanto1H
        | WeaponTag::OCF_WeapTypeToolKnife1H
        | WeaponTag::OCF_WeapTypeHuntingKnife1H
        | WeaponTag::WAF_WeapTypeScalpel
        | WeaponTag::OCF_WeapTypeTwinDagger1H
);
const GREATSWORDS: EnumSet<WeaponTag> = enum_set!(
    WeaponTag::OCF_WeapTypeMassiveSword2H
        | WeaponTag::OCF_WeapTypeRapier2H
        | WeaponTag::OCF_WeapTypeSaber2H
        | WeaponTag::OCF_WeapTypeScimitar2H
        | WeaponTag::OCF_WeapTypeTwinblade2H
        | WeaponTag::OCF_WeapTypeCleaver2H
        | WeaponTag::OCF_WeapTypeGreatsword2H
        | WeaponTag::OCF_WeapTypeLightsaber2H
);
const GUNS: EnumSet<WeaponTag> = enum_set!(
    WeaponTag::WeapTypeGun
        | WeaponTag::OCF_WeapTypeGun
        | WeaponTag::OCF_WeapTypeGun1H
        | WeaponTag::OCF_WeapTypeGun2H
        | WeaponTag::OCF_WeapTypeGun1H_Axe
        | WeaponTag::OCF_WeapTypeGun1H_Basic
        | WeaponTag::OCF_WeapTypeGun1H_Gravity
        | WeaponTag::OCF_WeapTypeGun1H_Sword
        | WeaponTag::OCF_WeapTypeGun2H_Basic
        | WeaponTag::OCF_WeapTypeGun2H_Launcher
        | WeaponTag::OCF_WeapTypeGun2H_Shotgun
        | WeaponTag::OCF_WeapTypeGun2H_Spear
        | WeaponTag::OCF_WeapTypeGun2H_Special
);
const HALBERDS: EnumSet<WeaponTag> = enum_set!(
    WeaponTag::WeapTypeHalberd
        | WeaponTag::OCF_WeapTypeGlaive1H
        | WeaponTag::OCF_WeapTypeGlaive2H
        | WeaponTag::OCF_WeapTypeHalberd1H
        | WeaponTag::OCF_WeapTypeHalberd2H
        | WeaponTag::OCF_WeapTypePole1H_Swing
        | WeaponTag::OCF_WeapTypePole2H_Swing
);
const HAMMERS: EnumSet<WeaponTag> = enum_set!(
    WeaponTag::WeapTypeHammer
        | WeaponTag::OCF_WeapTypeHammer1H
        | WeaponTag::WeapTypeWarhammer
        | WeaponTag::OCF_WeapTypeWarhammer2H
);
const HAND_TO_HAND: EnumSet<WeaponTag> =
    enum_set!(WeaponTag::HandToHandMelee | WeaponTag::OCF_WeapTypeUnarmed | WeaponTag::None);
const KATANAS: EnumSet<WeaponTag> =
    enum_set!(WeaponTag::OCF_WeapTypeKatana1H | WeaponTag::OCF_WeapTypeKatana2H);
const LANCES: EnumSet<WeaponTag> = enum_set!(
    WeaponTag::WeapTypeLance
        | WeaponTag::OCF_WeapTypeJavelin1H
        | WeaponTag::OCF_WeapTypeJavelin2H
        | WeaponTag::OCF_WeapTypeLance1H
        | WeaponTag::OCF_WeapTypeLance2H
        | WeaponTag::OCF_WeapTypePike1H
        | WeaponTag::OCF_WeapTypePike2H
        | WeaponTag::OCF_WeapTypePole1H_Thrust
        | WeaponTag::OCF_WeapTypePole2H_Thrust
        | WeaponTag::OCF_WeapTypeSpear1H
        | WeaponTag::OCF_WeapTypeSpear2H
        | WeaponTag::OCF_WeapTypeTrident1H
        | WeaponTag::OCF_WeapTypeTrident2H
);
const MACES: EnumSet<WeaponTag> = enum_set!(
    WeaponTag::OCF_WeapTypeBoomerang1H
        | WeaponTag::OCF_WeapTypeCestus1H
        | WeaponTag::OCF_WeapTypeClub1H
        | WeaponTag::OCF_WeapTypeMace1H
        | WeaponTag::OCF_WeapTypeClub2H
        | WeaponTag::OCF_WeapTypeMace2H
);
const QUARTERSTAVES: EnumSet<WeaponTag> =
    enum_set!(WeaponTag::WeapTypeQtrStaff | WeaponTag::OCF_WeapTypeQuarterstaff1H);
const SCYTHES: EnumSet<WeaponTag> = enum_set!(
    WeaponTag::WeapTypeScythe
        | WeaponTag::OCF_WeapTypeWarscythe1H
        | WeaponTag::OCF_WeapTypeWarscythe2H
        | WeaponTag::OCF_WeapTypeScythe2H
        | WeaponTag::OCF_WeapTypeScythe1H
);
const SWORDS: EnumSet<WeaponTag> = enum_set!(
    WeaponTag::OCF_WeapTypeRapier1H
        | WeaponTag::OCF_WeapTypeSaber1H
        | WeaponTag::OCF_WeapTypeScimitar1H
        | WeaponTag::OCF_WeapTypeSword1H
        | WeaponTag::OCF_WeapTypeTwinblade1H
        | WeaponTag::OCF_WeapTypeLightsaber1H
);
const STAVES: EnumSet<WeaponTag> =
    enum_set!(WeaponTag::Staff | WeaponTag::WeapTypeStaff | WeaponTag::OCF_WeapTypeBlankStaff);
const WARAXES: EnumSet<WeaponTag> = enum_set!(
    WeaponTag::OCF_WeapTypeCleaver1H
        | WeaponTag::OCF_WeapTypeCrescent1H
        | WeaponTag::OCF_WeapTypeHandBlade1H
        | WeaponTag::OCF_WeapTypeHatchet1H
        | WeaponTag::OCF_WeapTypeSickle1H
        | WeaponTag::OCF_WeapTypeWarAxe1H
        | WeaponTag::OCF_WeapTypeWarpick1H
);
const WHIPS: EnumSet<WeaponTag> =
    enum_set!(WeaponTag::OCF_WeapTypeWhip1H | WeaponTag::WeapTypeWhip);

// const TWO_HANDED: EnumSet<WeaponTag> = enum_set!();

/// This enum represents all the keywords we expect for weapon types. We group
/// the tags into sets for efficient subtype classification from the tags.
#[derive(Debug, EnumString, Hash, EnumSetType)]
pub enum WeaponTag {
    Bow,
    Crossbow,
    Gun,
    OCF_WeapTypeBattleaxe2H,
    OCF_WeapTypeBlankStaff,
    OCF_WeapTypeBlowgun2H,
    OCF_WeapTypeBoomerang1H,
    OCF_WeapTypeBow,
    OCF_WeapTypeBow2H,
    OCF_WeapTypeCestus1H,
    OCF_WeapTypeChakram1H,
    OCF_WeapTypeClaw1H,
    OCF_WeapTypeCleaver1H,
    OCF_WeapTypeCleaver2H,
    OCF_WeapTypeClub1H,
    OCF_WeapTypeClub2H,
    OCF_WeapTypeCrescent1H,
    OCF_WeapTypeCrossbow,
    OCF_WeapTypeCrossbow1H,
    OCF_WeapTypeCrossbow2H,
    OCF_WeapTypeCutlery1H,
    OCF_WeapTypeDagger1H,
    OCF_WeapTypeFishingRod1H,
    OCF_WeapTypeGlaive1H,
    OCF_WeapTypeGlaive2H,
    OCF_WeapTypeGreatbow2H,
    OCF_WeapTypeGreatsword2H,
    OCF_WeapTypeGun,
    OCF_WeapTypeGun1H,
    OCF_WeapTypeGun1H_Axe,
    OCF_WeapTypeGun1H_Basic,
    OCF_WeapTypeGun1H_Gravity,
    OCF_WeapTypeGun1H_Sword,
    OCF_WeapTypeGun2H,
    OCF_WeapTypeGun2H_Basic,
    OCF_WeapTypeGun2H_Launcher,
    OCF_WeapTypeGun2H_Shotgun,
    OCF_WeapTypeGun2H_Spear,
    OCF_WeapTypeGun2H_Special,
    OCF_WeapTypeHalberd1H,
    OCF_WeapTypeHalberd2H,
    OCF_WeapTypeHammer1H,
    OCF_WeapTypeHandBlade1H,
    OCF_WeapTypeHatchet1H,
    OCF_WeapTypeHuntingKnife1H,
    OCF_WeapTypeJavelin1H,
    OCF_WeapTypeJavelin2H,
    OCF_WeapTypeKatana1H,
    OCF_WeapTypeKatana2H,
    OCF_WeapTypeKunai1H,
    OCF_WeapTypeLance1H,
    OCF_WeapTypeLance2H,
    OCF_WeapTypeLightsaber1H,
    OCF_WeapTypeLightsaber2H,
    OCF_WeapTypeMace1H,
    OCF_WeapTypeMace2H,
    OCF_WeapTypeMassiveSword2H,
    OCF_WeapTypePickaxe1H,
    OCF_WeapTypePike1H,
    OCF_WeapTypePike2H,
    OCF_WeapTypePole1H_Swing,
    OCF_WeapTypePole1H_Thrust,
    OCF_WeapTypePole2H_Swing,
    OCF_WeapTypePole2H_Thrust,
    OCF_WeapTypeQuarterstaff1H,
    OCF_WeapTypeRapier1H,
    OCF_WeapTypeRapier2H,
    OCF_WeapTypeRevDagger1H,
    OCF_WeapTypeSaber1H,
    OCF_WeapTypeSaber2H,
    OCF_WeapTypeSai1H,
    OCF_WeapTypeScimitar1H,
    OCF_WeapTypeScimitar2H,
    OCF_WeapTypeScythe1H,
    OCF_WeapTypeScythe2H,
    OCF_WeapTypeShiv1H,
    OCF_WeapTypeShuriken1H,
    OCF_WeapTypeSickle1H,
    OCF_WeapTypeSlingshot2H,
    OCF_WeapTypeSpear1H,
    OCF_WeapTypeSpear2H,
    OCF_WeapTypeSword1H,
    OCF_WeapTypeTanto1H,
    OCF_WeapTypeToolKnife1H,
    OCF_WeapTypeTrident1H,
    OCF_WeapTypeTrident2H,
    OCF_WeapTypeTwinblade1H,
    OCF_WeapTypeTwinblade2H,
    OCF_WeapTypeTwinDagger1H,
    OCF_WeapTypeUnarmed,
    OCF_WeapTypeWarAxe1H,
    OCF_WeapTypeWarhammer2H,
    OCF_WeapTypeWarpick1H,
    OCF_WeapTypeWarpick2H,
    OCF_WeapTypeWarscythe1H,
    OCF_WeapTypeWarscythe2H,
    OCF_WeapTypeWhip1H,
    OCF_WeapTypeWoodaxe1H,
    OCF_WeapTypeWoodHatchet1H,
    OneHandDagger,
    OneHandSword,
    Staff,
    TwoHandAxe,
    TwoHandSword,
    WAF_WeapTypeGrenade,
    WAF_WeapTypeScalpel,
    WeapTypeAxeTwoHanded,
    WeapTypeBattleaxe,
    WeapTypeBow,
    WeapTypeClaw,
    WeapTypeCrossbow,
    WeapTypeDagger,
    WeapTypeFlail,
    WeapTypeGreatsword,
    WeapTypeGun,
    WeapTypeHalberd,
    WeapTypeHammer,
    WeapTypeLance,
    WeapTypeMace,
    WeapTypeQtrStaff,
    WeapTypeScythe,
    WeapTypeStaff,
    WeapTypeSword,
    WeapTypeWarAxe,
    WeapTypeWarhammer,
    WeapTypeWhip,
    HandToHandMelee,
    None,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keywords_convert() {
        let input = vec![
            "OCF_InvColorBlood".to_string(),
            "WeapTypeGreatsword".to_string(),
            "WeapTypeHalberd".to_string(),
            "OCF_WeapTypeHalberd2H".to_string(),
        ];

        let result = WeaponType::classify("TestName", input, true);
        assert_eq!(
            result,
            WeaponType::Halberd(WeaponEquipType::TwoHanded, InvColor::Blood)
        );

        let input = vec![
            "OCF_InvColorBlood".to_string(),
            "Weapon".to_string(),
            "OCF_WeapTypePole2H_Swing".to_string(),
            "OCF_WeapTypeGlaive2H".to_string(),
            "OCF_WeapTypeWarscythe2H".to_string(),
            "OCF_WeapTypeHalberd2H".to_string(),
        ];
        let result = WeaponType::classify("TestName", input, true);
        assert_eq!(
            result,
            WeaponType::Halberd(WeaponEquipType::TwoHanded, InvColor::Blood)
        );

        let input = vec![
            "DaedricArtifact".to_string(),
            "Weapon".to_string(),
            "OCF_ArtifactDaedric".to_string(),
        ];
        let result = WeaponType::classify("TestName", input, false);
        assert_eq!(
            result,
            WeaponType::WeaponDefault(WeaponEquipType::OneHanded, InvColor::White)
        );

        let input = vec![
            "OCF_WeapTypeLongsword2H".to_string(),
            "OCF_InvColorFire".to_string(),
            "Weapon".to_string(),
            "TwoHandSword".to_string(),
        ];
        let result = WeaponType::classify("TestName", input, true);
        assert_eq!(
            result,
            WeaponType::SwordTwoHanded(WeaponEquipType::TwoHanded, InvColor::Fire)
        );
    }
}
