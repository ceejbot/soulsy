#![allow(non_snake_case, non_camel_case_types)]

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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct WeaponType {
    icon: Icon,
    color: InvColor,
    equiptype: WeaponEquipType,
}

impl WeaponType {
    pub fn new(icon: Icon, color: InvColor, equiptype: WeaponEquipType) -> Self {
        Self {
            icon,
            color,
            equiptype,
        }
    }

    pub fn is_one_handed(&self) -> bool {
        matches!(
            self.equiptype,
            WeaponEquipType::LeftHand | WeaponEquipType::RightHand | WeaponEquipType::EitherHand
        )
    }

    pub fn is_two_handed(&self) -> bool {
        matches!(self.equiptype, WeaponEquipType::TwoHanded)
    }
}

impl HasKeywords for WeaponType {
    fn classify(_name: &str, keywords: Vec<String>, twohanded: bool) -> Self {
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

        let maybe_icon = tags.iter().find_map(|subtype| {
            if BATTLEAXES.contains(*subtype) {
                Some(Icon::WeaponAxeTwoHanded)
            } else if BOWS.contains(*subtype) {
                Some(Icon::WeaponBow)
            } else if CROSSBOWS.contains(*subtype) {
                Some(Icon::WeaponCrossbow)
            } else if DAGGERS.contains(*subtype) {
                Some(Icon::WeaponDagger)
            } else if GREATSWORDS.contains(*subtype) {
                Some(Icon::WeaponSwordTwoHanded)
            } else if GUNS.contains(*subtype) {
                Some(Icon::WeaponGun)
            } else if HAMMERS.contains(*subtype) {
                Some(Icon::WeaponHammer)
            } else if HALBERDS.contains(*subtype) {
                Some(Icon::WeaponHalberd)
            } else if HAND_TO_HAND.contains(*subtype) {
                Some(Icon::HandToHand)
            } else if KATANAS.contains(*subtype) {
                Some(Icon::WeaponKatana)
            } else if LANCES.contains(*subtype) {
                Some(Icon::WeaponLance)
            } else if MACES.contains(*subtype) {
                Some(Icon::WeaponMace)
            } else if QUARTERSTAVES.contains(*subtype) {
                Some(Icon::WeaponQuarterstaff)
            } else if SCYTHES.contains(*subtype) {
                Some(Icon::WeaponScythe)
            } else if STAVES.contains(*subtype) {
                Some(Icon::WeaponStaff)
            } else if SWORDS.contains(*subtype) {
                Some(Icon::WeaponSwordOneHanded)
            } else if WARAXES.contains(*subtype) {
                Some(Icon::WeaponAxeOneHanded)
            } else if WHIPS.contains(*subtype) {
                Some(Icon::WeaponWhip)
            } else if matches!(subtype, WeaponTag::OCF_WeapTypePickaxe1H) {
                Some(Icon::WeaponPickaxe)
            } else if matches!(
                subtype,
                WeaponTag::OCF_WeapTypeWoodaxe1H | WeaponTag::OCF_WeapTypeWoodHatchet1H
            ) {
                Some(Icon::WeaponWoodAxe)
            } else if matches!(subtype, WeaponTag::OCF_WeapTypeFishingRod1H) {
                Some(Icon::WeaponFishingRod)
            } else if matches!(
                subtype,
                WeaponTag::WeapTypeClaw | WeaponTag::OCF_WeapTypeClaw1H
            ) {
                Some(Icon::WeaponClaw)
            } else if matches!(subtype, WeaponTag::WeapTypeFlail) {
                Some(Icon::WeaponFlail)
            } else if matches!(subtype, WeaponTag::WeapTypeStaff) {
                Some(Icon::WeaponStaff)
            } else if matches!(subtype, WeaponTag::WAF_WeapTypeGrenade) {
                Some(Icon::WeaponGrenade)
            } else {
                // Now we look for more general tags. Fortunately these keyword lists are short.
                keywords.iter().find_map(|xs| {
                    let Ok(subtype) = WeaponTag::try_from(xs.as_str()) else {
                        return None;
                    };
                    if FALLBACK_TYPES.contains(subtype) {
                        match subtype {
                            WeaponTag::TwoHandSword => Some(Icon::WeaponSwordTwoHanded),
                            WeaponTag::Bow => Some(Icon::WeaponBow),
                            WeaponTag::WeapTypeBow => Some(Icon::WeaponBow),
                            WeaponTag::WeapTypeDagger => Some(Icon::WeaponDagger),
                            WeaponTag::OneHandDagger => Some(Icon::WeaponDagger),
                            WeaponTag::WeapTypeGreatsword => Some(Icon::WeaponSwordTwoHanded),
                            WeaponTag::WeapTypeMace => Some(Icon::WeaponMace),
                            WeaponTag::WeapTypeSword => Some(Icon::WeaponSwordOneHanded),
                            WeaponTag::OneHandSword => Some(Icon::WeaponSwordOneHanded),
                            WeaponTag::WeapTypeWarAxe => Some(Icon::WeaponAxeOneHanded),
                            WeaponTag::TwoHandAxe => Some(Icon::WeaponAxeTwoHanded),
                            _ => None,
                        }
                    } else {
                        None
                    }
                })
            }
        });

        let icon = if let Some(icon) = maybe_icon {
            icon
        } else {
            Icon::WeaponSwordOneHanded
        };

        WeaponType::new(icon, color, equiptype)
    }
}

impl Default for WeaponType {
    fn default() -> Self {
        WeaponType::new(
            Icon::WeaponSwordOneHanded,
            InvColor::default(),
            WeaponEquipType::EitherHand,
        )
    }
}

impl HasIcon for WeaponType {
    fn color(&self) -> Color {
        self.color.color()
    }

    fn icon_file(&self) -> String {
        self.icon.icon_file()
    }

    fn icon_fallback(&self) -> String {
        Icon::WeaponSwordOneHanded.icon_file()
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
        | WeaponTag::OCF_WeapTypeLongsword2H
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
    OCF_WeapTypeLongsword2H,
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
            "OCF_WeapTypeHalberd2H".to_string(),
        ];

        let result = WeaponType::classify("TestName", input, true);
        assert_eq!(result.equiptype, WeaponEquipType::TwoHanded);
        assert_eq!(result.color, InvColor::Blood);
        assert_eq!(result.icon, Icon::WeaponHalberd);

        let input = vec![
            "OCF_InvColorBlood".to_string(),
            "Weapon".to_string(),
            "OCF_WeapTypePole2H_Swing".to_string(),
            "OCF_WeapTypeGlaive2H".to_string(),
            "OCF_WeapTypeWarscythe2H".to_string(),
            "OCF_WeapTypeHalberd2H".to_string(),
        ];
        let result = WeaponType::classify("TestName", input, true);
        assert_eq!(result.equiptype, WeaponEquipType::TwoHanded);
        assert_eq!(result.color, InvColor::Blood);
        assert_eq!(result.icon, Icon::WeaponHalberd);

        let input = vec![
            "DaedricArtifact".to_string(),
            "Weapon".to_string(),
            "OCF_ArtifactDaedric".to_string(),
        ];
        let result = WeaponType::classify("TestName", input, false);
        assert_eq!(result.equiptype, WeaponEquipType::EitherHand);
        assert_eq!(result.color, InvColor::White);
        assert_eq!(result.icon, Icon::WeaponSwordOneHanded);

        let input = vec![
            "OCF_WeapTypeLongsword2H".to_string(),
            "OCF_InvColorFire".to_string(),
            "Weapon".to_string(),
            "TwoHandSword".to_string(),
        ];
        let result = WeaponType::classify("TestName", input, true);
        assert_eq!(result.equiptype, WeaponEquipType::TwoHanded);
        assert_eq!(result.color, InvColor::Fire);
        assert_eq!(result.icon, Icon::WeaponSwordTwoHanded);
    }
}
