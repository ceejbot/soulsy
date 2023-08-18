#![allow(non_snake_case, non_camel_case_types)]

use enumset::{enum_set, EnumSet, EnumSetType};
use strum::{Display, EnumString};

use super::color::InvColor;
use super::icons::Icon;
use super::{base, HasIcon, HasKeywords};
use crate::plugin::Color;

#[derive(Clone, Debug, Display, Eq, Hash, PartialEq)]
pub enum ArmorType {
    Head(ArmorWeight, InvColor),
    Body(ArmorWeight, InvColor),
    Hands(ArmorWeight, InvColor),
    Feet(ArmorWeight, InvColor),
    Shield(ArmorWeight, InvColor),
    Amulet(InvColor),
    Circlet(InvColor),
    Cloak(InvColor),
    Earring(InvColor),
    Mask(InvColor),
    Ring(InvColor),
    Robes(InvColor),
    Belt,
    Backpack,
    Lantern,
    Default,
}

#[derive(Clone, Debug, Display, Eq, Hash, PartialEq)]
pub enum ArmorWeight {
    Clothing,
    Light,
    Heavy,
}

impl HasKeywords for ArmorType {
    fn classify(keywords: Vec<String>, _ignored: bool) -> Self {
        let color = base::color_from_keywords(&keywords);

        let tags: Vec<ArmorTag> = keywords
            .iter()
            .filter_map(|xs| {
                if let Ok(subtype) = ArmorTag::try_from(xs.as_str()) {
                    Some(subtype)
                } else {
                    None
                }
            })
            .collect();

        let weight = if let Some(w) = tags.iter().find_map(|xs| {
            if LIGHT.contains(*xs) {
                Some(ArmorWeight::Light)
            } else if HEAVY.contains(*xs) {
                Some(ArmorWeight::Heavy)
            } else if CLOTHES.contains(*xs) {
                Some(ArmorWeight::Clothing)
            } else {
                None
            }
        }) {
            w
        } else {
            ArmorWeight::Clothing
        };

        let kind = if let Some(k) = tags.iter().find_map(|tag| {
            if AMULETS.contains(*tag) {
                Some(ArmorType::Amulet(color.clone()))
            } else if CIRCLETS.contains(*tag) {
                Some(ArmorType::Circlet(color.clone()))
            } else if HEAD.contains(*tag) {
                Some(ArmorType::Head(weight.clone(), color.clone()))
            } else if HANDS.contains(*tag) {
                Some(ArmorType::Hands(weight.clone(), color.clone()))
            } else if BODY.contains(*tag) {
                Some(ArmorType::Body(weight.clone(), color.clone()))
            } else if FEET.contains(*tag) {
                Some(ArmorType::Feet(weight.clone(), color.clone()))
            } else if SHIELDS.contains(*tag) {
                Some(ArmorType::Shield(weight.clone(), color.clone()))
            } else if RINGS.contains(*tag) {
                Some(ArmorType::Ring(color.clone()))
            } else if CLOAKS.contains(*tag) {
                Some(ArmorType::Cloak(color.clone()))
            } else if MASKS.contains(*tag) {
                Some(ArmorType::Mask(color.clone()))
            } else if BELTS.contains(*tag) {
                Some(ArmorType::Belt)
            } else if LIGHTS.contains(*tag) {
                Some(ArmorType::Lantern)
            } else if JEWELRY.contains(*tag) {
                Some(ArmorType::Earring(color.clone()))
            } else if matches!(tag, ArmorTag::OCF_BagTypeBackpack) {
                Some(ArmorType::Backpack)
            } else {
                None
            }
        }) {
            k
        } else {
            log::info!("default armor type; keywords: {keywords:?}");
            ArmorType::Default
        };

        kind
    }
}

impl HasIcon for ArmorType {
    fn color(&self) -> Color {
        Color::default()
    }

    fn icon_file(&self) -> String {
        match self {
            ArmorType::Head(weight, _c) => match weight {
                ArmorWeight::Clothing => Icon::ArmorClothingHead.icon_file(),
                ArmorWeight::Heavy => Icon::ArmorHeavyHead.icon_file(),
                ArmorWeight::Light => Icon::ArmorLightHead.icon_file(),
            },
            ArmorType::Body(weight, _c) => match weight {
                ArmorWeight::Clothing => Icon::ArmorClothing.icon_file(),
                ArmorWeight::Heavy => Icon::ArmorHeavy.icon_file(),
                ArmorWeight::Light => Icon::ArmorLight.icon_file(),
            },
            ArmorType::Hands(weight, _c) => match weight {
                ArmorWeight::Clothing => Icon::ArmorClothingHands.icon_file(),
                ArmorWeight::Heavy => Icon::ArmorHeavyHands.icon_file(),
                ArmorWeight::Light => Icon::ArmorLightHands.icon_file(),
            },
            ArmorType::Feet(weight, _c) => match weight {
                ArmorWeight::Clothing => Icon::ArmorClothingFeet.icon_file(),
                ArmorWeight::Heavy => Icon::ArmorHeavyFeet.icon_file(),
                ArmorWeight::Light => Icon::ArmorLightFeet.icon_file(),
            },
            ArmorType::Shield(weight, _c) => match weight {
                ArmorWeight::Clothing => Icon::ArmorShieldLight.icon_file(),
                ArmorWeight::Heavy => Icon::ArmorShieldHeavy.icon_file(),
                ArmorWeight::Light => Icon::ArmorShieldLight.icon_file(),
            },
            ArmorType::Amulet(_) => Icon::ArmorAmulet.icon_file(),
            ArmorType::Earring(_) => Icon::ArmorEarring.icon_file(),
            ArmorType::Circlet(_) => Icon::ArmorCirclet.icon_file(),
            ArmorType::Cloak(_) => Icon::ArmorCloak.icon_file(),
            ArmorType::Mask(_) => Icon::ArmorMask.icon_file(),
            ArmorType::Ring(_) => Icon::ArmorRing.icon_file(),
            ArmorType::Robes(_) => Icon::ArmorRobes.icon_file(),
            ArmorType::Backpack => Icon::ArmorBackpack.icon_file(),
            ArmorType::Belt => Icon::ArmorBelt.icon_file(),
            ArmorType::Lantern => Icon::Lantern.icon_file(),
            ArmorType::Default => Icon::ArmorClothing.icon_file(),
        }
    }

    fn icon_fallback(&self) -> String {
        match self {
            ArmorType::Head(_, _) => Icon::ArmorHeavy.icon_file(),
            ArmorType::Body(_, _) => Icon::ArmorHeavy.icon_file(),
            ArmorType::Hands(_, _) => Icon::ArmorHeavy.icon_file(),
            ArmorType::Feet(_, _) => Icon::ArmorHeavy.icon_file(),
            ArmorType::Shield(_, _) => Icon::ArmorShieldHeavy.icon_file(),
            ArmorType::Amulet(_) => Icon::ArmorClothing.icon_file(),
            ArmorType::Circlet(_) => Icon::ArmorClothing.icon_file(),
            ArmorType::Cloak(_) => Icon::ArmorClothing.icon_file(),
            ArmorType::Earring(_) => Icon::ArmorClothing.icon_file(),
            ArmorType::Mask(_) => Icon::ArmorClothing.icon_file(),
            ArmorType::Ring(_) => Icon::ArmorClothing.icon_file(),
            ArmorType::Robes(_) => Icon::ArmorClothing.icon_file(),
            ArmorType::Backpack => Icon::ArmorClothing.icon_file(),
            ArmorType::Belt => Icon::ArmorClothing.icon_file(),
            ArmorType::Lantern => Icon::Torch.icon_file(),
            ArmorType::Default => Icon::ArmorClothing.icon_file(),
        }
    }
}

const CLOTHES: EnumSet<ArmorTag> = enum_set!(
    ArmorTag::ArmorClothing
        | ArmorTag::ClothingBody
        | ArmorTag::ClothingCirclet
        | ArmorTag::ClothingCrown
        | ArmorTag::ClothingEarrings
        | ArmorTag::ClothingFeet
        | ArmorTag::ClothingHands
        | ArmorTag::ClothingNecklace
        | ArmorTag::ClothingPanties
        | ArmorTag::ClothingRing
        | ArmorTag::ClothingStrapOn
        | ArmorTag::FrostfallIsCloakCloth
        | ArmorTag::VendorItemClothing
        | ArmorTag::WAF_ClothingAccessories
        | ArmorTag::WAF_ClothingCloak
        | ArmorTag::WAF_ClothingMedicalHealing
        | ArmorTag::WAF_ClothingPouch
);

const LIGHT: EnumSet<ArmorTag> = enum_set!(
    ArmorTag::OCF_AccessoryShield_Light
        | ArmorTag::OCF_ArmorBoots_Light
        | ArmorTag::OCF_ArmorCuirass_Light
        | ArmorTag::OCF_ArmorGauntlets_Light
        | ArmorTag::OCF_ArmorHelmet_Light
        | ArmorTag::OCF_ArmorShield_Light
);
const HEAVY: EnumSet<ArmorTag> = enum_set!(
    ArmorTag::OCF_ArmorBoots_Heavy
        | ArmorTag::OCF_ArmorCuirass_Heavy
        | ArmorTag::OCF_ArmorGauntlets_Heavy
        | ArmorTag::OCF_ArmorHelmet_Heavy
        | ArmorTag::OCF_ArmorShield_Heavy
);

const AMULETS: EnumSet<ArmorTag> = enum_set!(
    ArmorTag::ClothingNecklace
        | ArmorTag::OCF_ArtifactAedric_AmuletKings
        | ArmorTag::OCF_ArtifactDaedric_SanctuaryAmulet
        | ArmorTag::OCF_ArtifactLegendary_GaulderAmulet
        | ArmorTag::OCF_ArtifactLegendary_NecromancerAmulet
        | ArmorTag::OCF_ReplicaAedric_AmuletKings
        | ArmorTag::OCF_ReplicaDaedric_SanctuaryAmulet
        | ArmorTag::OCF_ReplicaLegendary_GaulderAmulet
        | ArmorTag::OCF_ReplicaLegendary_NecromancerAmulet
);

const CLOAKS: EnumSet<ArmorTag> =
    enum_set!(ArmorTag::FrostfallIsCloakCloth | ArmorTag::WAF_ClothingCloak);

const HANDS: EnumSet<ArmorTag> = enum_set!(
    ArmorTag::ClothingHands
        | ArmorTag::OCF_ArmorGauntlets_Heavy
        | ArmorTag::OCF_ArmorGauntlets_Light
        | ArmorTag::OCF_ArmorGauntlets_Medium
        | ArmorTag::OCF_ArmorTypeHands
        | ArmorTag::OCF_ArmorTypeHands
        | ArmorTag::OCF_ArmorTypeHands_Alt
        | ArmorTag::OCF_ArmorTypeHands_Main
        | ArmorTag::OCF_ArtifactAedric_CrusaderGauntlets
        | ArmorTag::OCF_ArtifactDaedric_NightingaleGauntlets
        | ArmorTag::OCF_HandTypeArmlet
        | ArmorTag::OCF_HandTypeBandage
        | ArmorTag::OCF_HandTypeBracer
        | ArmorTag::OCF_HandTypeClaws
        | ArmorTag::OCF_HandTypeCuffs
        | ArmorTag::OCF_HandTypeGloves
        | ArmorTag::OCF_HandTypeSleeves
        | ArmorTag::OCF_ReplicaAedric_CrusaderGauntlets
        | ArmorTag::OCF_ReplicaDaedric_NightingaleGauntlets
        | ArmorTag::WAF_FingerlessGauntletsBracers
        | ArmorTag::WAF_SpikedGauntletGloves
        | ArmorTag::OCF_ArtifactDwarven_Wraithguard
        | ArmorTag::OCF_ArtifactLegendary_FistsRandagulf
);

const RINGS: EnumSet<ArmorTag> = enum_set!(
    ArmorTag::ClothingRing
        | ArmorTag::OCF_ArtifactAedric_RingPhynaster
        | ArmorTag::OCF_ArtifactAedric_RingWarlock
        | ArmorTag::OCF_ArtifactAedric_RingWind
        | ArmorTag::OCF_ArtifactDaedric_RingHircine
        | ArmorTag::OCF_ArtifactDaedric_RingKhajiit
        | ArmorTag::OCF_ArtifactDaedric_RingMoonStar
        | ArmorTag::OCF_ArtifactDaedric_RingNamira
        | ArmorTag::OCF_ArtifactLegendary_RingMasser
        | ArmorTag::OCF_ArtifactLegendary_RingMentor
        | ArmorTag::OCF_ArtifactLegendary_RingVampiric
        | ArmorTag::OCF_ArtifactLegendary_RingVipereye
        | ArmorTag::OCF_ArtifactLegendary_RingZurinArctus
        | ArmorTag::OCF_ReplicaAedric_RingPhynaster
        | ArmorTag::OCF_ReplicaAedric_RingWarlock
        | ArmorTag::OCF_ReplicaAedric_RingWind
        | ArmorTag::OCF_ReplicaDaedric_RingHircine
        | ArmorTag::OCF_ReplicaDaedric_RingKhajiit
        | ArmorTag::OCF_ReplicaDaedric_RingNamira
        | ArmorTag::OCF_ReplicaLegendary_RingMentor
        | ArmorTag::OCF_ReplicaLegendary_RingVampiric
        | ArmorTag::OCF_ReplicaLegendary_RingVipereye
        | ArmorTag::OCF_ReplicaLegendary_RingZurinArctus
);

const CIRCLETS: EnumSet<ArmorTag> = enum_set!(
    ArmorTag::ArmorCrown
        | ArmorTag::ClothingCirclet
        | ArmorTag::ClothingCrown
        | ArmorTag::OCF_ArtifactDwarven_AetherialCrown
        | ArmorTag::OCF_ArtifactLegendary_JaggedCrown
        | ArmorTag::OCF_ReplicaDwarven_AetherialCrown
        | ArmorTag::OCF_ReplicaLegendary_JaggedCrown
        | ArmorTag::OCF_ArtifactDaedric_Nightingale
);

const JEWELRY: EnumSet<ArmorTag> =
    enum_set!(ArmorTag::ClothingEarrings | ArmorTag::OCF_AccessoryJewelry);

const HEAD: EnumSet<ArmorTag> = enum_set!(
    ArmorTag::ArmorHelmet
        | ArmorTag::OCF_ArmorHelmet_Heavy
        | ArmorTag::OCF_ArmorHelmet_Light
        | ArmorTag::OCF_ArmorHelmet_Medium
        | ArmorTag::OCF_ArmorTypeHead
        | ArmorTag::OCF_ArmorTypeHead_Alt
        | ArmorTag::OCF_ArmorTypeHead_Main
        | ArmorTag::OCF_ArtifactAedric_CrusaderHelm
        | ArmorTag::OCF_ArtifactDaedric_NightingaleHelmet
        | ArmorTag::OCF_ArtifactLegendary_AdamantiumHelmTohan
        | ArmorTag::OCF_ArtifactLegendary_BloodwormHelm
        | ArmorTag::OCF_ArtifactLegendary_HelmOreynBearclaw
        | ArmorTag::OCF_ArtifactLegendary_HelmTiberSeptim
        | ArmorTag::OCF_HeadTypeBandage
        | ArmorTag::OCF_HeadTypeBandana
        | ArmorTag::OCF_HeadTypeBarrette
        | ArmorTag::OCF_HeadTypeBlindfold
        | ArmorTag::OCF_HeadTypeEarsReal
        | ArmorTag::OCF_HeadTypeEyePatch
        | ArmorTag::OCF_HeadTypeGag
        | ArmorTag::OCF_HeadTypeGoggles
        | ArmorTag::OCF_HeadTypeHalo
        | ArmorTag::OCF_HeadTypeHat
        | ArmorTag::OCF_HeadTypeHood
        | ArmorTag::OCF_HeadTypeHorns
        | ArmorTag::OCF_HeadTypeHornsAntlers
        | ArmorTag::OCF_HeadTypeWig
        | ArmorTag::OCF_ReplicaAedric_CrusaderHelm
        | ArmorTag::OCF_ReplicaDaedric_NightingaleHelmet
        | ArmorTag::OCF_ReplicaLegendary_AdamantiumHelmTohan
        | ArmorTag::OCF_ReplicaLegendary_BloodwormHelm
        | ArmorTag::OCF_ReplicaLegendary_HelmOreynBearclaw
        | ArmorTag::OCF_ReplicaLegendary_HelmTiberSeptim
        | ArmorTag::OCF_ArtifactDaedric_GrayCowlNocturnal
        | ArmorTag::ClavicusVileMask
        | ArmorTag::OCF_ArtifactDaedric_MasqueClavicusVile
        | ArmorTag::OCF_ArtifactDwarven_VisageMzund
        | ArmorTag::OCF_ArtifactLegendary_DragonPriestMask
        | ArmorTag::OCF_ReplicaDaedric_MasqueClavicusVile
);

const MASKS: EnumSet<ArmorTag> = enum_set!(
    ArmorTag::OCF_HeadTypeMask
        | ArmorTag::OCF_HeadTypeMaskEyes
        | ArmorTag::OCF_HeadTypeMaskFull
        | ArmorTag::OCF_HeadTypeMaskHood
        | ArmorTag::OCF_HeadTypeMaskMouth
);

const BODY: EnumSet<ArmorTag> = enum_set!(
    ArmorTag::ClothingBody
        | ArmorTag::ArmorClothing
        | ArmorTag::OCF_ArmorBodyPart
        | ArmorTag::OCF_ArmorTypeBody
        | ArmorTag::OCF_ArmorTypeBody_Alt
        | ArmorTag::OCF_ArmorTypeBody_Main
        | ArmorTag::OCF_BodyTypeCollar
        | ArmorTag::OCF_BodyTypeCorset
        | ArmorTag::OCF_BodyTypeDress
        | ArmorTag::OCF_BodyTypeLingerie
        | ArmorTag::OCF_BodyTypeMantle
        | ArmorTag::OCF_BodyTypePants
        | ArmorTag::OCF_BodyTypePauldron
        | ArmorTag::OCF_BodyTypePauldronL
        | ArmorTag::OCF_BodyTypePauldronLR
        | ArmorTag::OCF_BodyTypePauldronR
        | ArmorTag::OCF_BodyTypeRobes
        | ArmorTag::OCF_BodyTypeScarf
        | ArmorTag::OCF_BodyTypeSkirt
        | ArmorTag::OCF_BodyTypeTail
        | ArmorTag::OCF_BodyTypeTailReal
        | ArmorTag::OCF_BodyTypeTasset
        | ArmorTag::OCF_BodyTypeTorc
        | ArmorTag::OCF_BodyTypeTorso
        | ArmorTag::OCF_BodyTypeUnderwear_FullF
        | ArmorTag::OCF_BodyTypeWings
        | ArmorTag::OCF_BodyTypeWingsJewelry
        | ArmorTag::OCF_BodyTypeWingsReal
        | ArmorTag::OCF_ArmorCuirass_Heavy
        | ArmorTag::OCF_ArmorCuirass_Light
        | ArmorTag::OCF_ArmorCuirass_Medium
        | ArmorTag::OCF_ArtifactAedric_CrusaderCuirass
        | ArmorTag::OCF_ArtifactAedric_MorihausCuirass
        | ArmorTag::OCF_ArtifactDaedric_NightingaleCuirass
        | ArmorTag::OCF_ArtifactLegendary_DragonboneCuirass
        | ArmorTag::OCF_ReplicaAedric_CrusaderCuirass
        | ArmorTag::OCF_ReplicaAedric_MorihausCuirass
        | ArmorTag::OCF_ReplicaDaedric_NightingaleCuirass
        | ArmorTag::OCF_ReplicaLegendary_DragonboneCuirass
        | ArmorTag::OCF_ArtifactAedric_LordMail
        | ArmorTag::OCF_ArtifactDaedric_EbonyMail
        | ArmorTag::OCF_ReplicaAedric_LordMail
        | ArmorTag::OCF_ReplicaDaedric_EbonyMail
        | ArmorTag::OCF_ArtifactDaedric_SaviorHide
        | ArmorTag::OCF_ReplicaDaedric_SaviorHide
);
const FEET: EnumSet<ArmorTag> = enum_set!(
    ArmorTag::ClothingFeet
        | ArmorTag::OCF_ArmorTypeFeet
        | ArmorTag::OCF_ArmorTypeFeet_Alt
        | ArmorTag::OCF_ArmorTypeFeet_Main
        | ArmorTag::OCF_FeetTypeFootwraps
        | ArmorTag::OCF_FeetTypeHeels
        | ArmorTag::OCF_FeetTypeHeelsBoots
        | ArmorTag::OCF_FeetTypeSabatons
        | ArmorTag::OCF_FeetTypeSandals
        | ArmorTag::OCF_FeetTypeShoes
        | ArmorTag::OCF_FeetTypeStockings
        | ArmorTag::OCF_ArmorBoots_Heavy
        | ArmorTag::OCF_ArmorBoots_Light
        | ArmorTag::OCF_ArmorBoots_Medium
        | ArmorTag::OCF_ArtifactAedric_CrusaderBoots
        | ArmorTag::OCF_ArtifactDaedric_NightingaleBoots
        | ArmorTag::OCF_ArtifactLegendary_BootsApostle
        | ArmorTag::OCF_ArtifactLegendary_BootsBlindingSpeed
        | ArmorTag::OCF_FeetTypeHeelsBoots
        | ArmorTag::OCF_ReplicaAedric_CrusaderBoots
        | ArmorTag::OCF_ReplicaDaedric_NightingaleBoots
        | ArmorTag::OCF_ReplicaLegendary_BootsApostle
        | ArmorTag::OCF_ReplicaLegendary_BootsBlindingSpeed
        | ArmorTag::ClothingPanties
        | ArmorTag::ClothingStrapOn
);

const LIGHTS: EnumSet<ArmorTag> =
    enum_set!(ArmorTag::OCF_ToolLantern | ArmorTag::OCF_ToolLanternPaper);
const BELTS: EnumSet<ArmorTag> = enum_set!(
    ArmorTag::OCF_AccessoryBelt | ArmorTag::OCF_AccessoryBeltBook | ArmorTag::OCF_BagTypeBelt
);

const SHIELDS: EnumSet<ArmorTag> = enum_set!(
    ArmorTag::ArmorShield
        | ArmorTag::OCF_AccessoryShield
        | ArmorTag::OCF_AccessoryShield_Light
        | ArmorTag::OCF_ArmorShield_Heavy
        | ArmorTag::OCF_ArmorShield_Light
        | ArmorTag::OCF_ArmorShield_Medium
        | ArmorTag::OCF_ArtifactAedric_AurielShield
        | ArmorTag::OCF_ArtifactAedric_CrusaderShield
        | ArmorTag::OCF_ArtifactDaedric_Spellbreaker
        | ArmorTag::OCF_ArtifactDwarven_AetherialShield
        | ArmorTag::OCF_ArtifactLegendary_EleidonWard
        | ArmorTag::OCF_ArtifactLegendary_YsgramorShield
        | ArmorTag::OCF_ReplicaAedric_AurielShield
        | ArmorTag::OCF_ReplicaAedric_CrusaderShield
        | ArmorTag::OCF_ReplicaDwarven_AetherialShield
        | ArmorTag::OCF_ReplicaLegendary_YsgramorShield
        | ArmorTag::OCF_ShieldTypeBuckler
        | ArmorTag::OCF_ShieldTypeKite
        | ArmorTag::OCF_ShieldTypeSpiked
        | ArmorTag::OCF_ShieldTypeTower
        | ArmorTag::OCF_ReplicaDaedric_Spellbreaker
);

#[derive(Debug, EnumString, Hash, EnumSetType)]
pub enum ArmorTag {
    ArmorClothing,
    ArmorCrown,
    ArmorHelmet,
    ArmorMaterialHide,
    ArmorShield,
    ClavicusVileMask,
    ClothingBody,
    ClothingCirclet,
    ClothingCrown,
    ClothingEarrings,
    ClothingFeet,
    ClothingHands,
    ClothingNecklace,
    ClothingPanties,
    ClothingRing,
    ClothingStrapOn,
    DaedricArtifact,
    FrostfallEnableKeywordProtection,
    FrostfallIsCloakCloth,
    FrostfallIsWeatherproofAccessory,
    JewelryExpensive,
    OCF_AccessoryBelt,
    OCF_AccessoryBeltBook,
    OCF_AccessoryJewelry,
    OCF_AccessoryKatana,
    OCF_AccessoryMagic,
    OCF_AccessoryPiercing,
    OCF_AccessoryShield,
    OCF_AccessoryShield_Light,
    OCF_ArmorBindings,
    OCF_ArmorBodyPart,
    OCF_ArmorBoots_Heavy,
    OCF_ArmorBoots_Light,
    OCF_ArmorBoots_Medium,
    OCF_ArmorCuirass_Heavy,
    OCF_ArmorCuirass_Light,
    OCF_ArmorCuirass_Medium,
    OCF_ArmorGauntlets_Heavy,
    OCF_ArmorGauntlets_Light,
    OCF_ArmorGauntlets_Medium,
    OCF_ArmorHelmet_Heavy,
    OCF_ArmorHelmet_Light,
    OCF_ArmorHelmet_Medium,
    OCF_ArmorKinky,
    OCF_ArmorMainSkimpy,
    OCF_ArmorShield_Heavy,
    OCF_ArmorShield_Light,
    OCF_ArmorShield_Medium,
    OCF_ArmorTypeBody,
    OCF_ArmorTypeBody_Alt,
    OCF_ArmorTypeBody_Main,
    OCF_ArmorTypeFeet,
    OCF_ArmorTypeFeet_Alt,
    OCF_ArmorTypeFeet_Main,
    OCF_ArmorTypeHands,
    OCF_ArmorTypeHands_Alt,
    OCF_ArmorTypeHands_Main,
    OCF_ArmorTypeHead,
    OCF_ArmorTypeHead_Alt,
    OCF_ArmorTypeHead_Main,
    OCF_ArmorTypeOther,
    OCF_ArmorWintersun,
    OCF_Artifact,
    OCF_ArtifactAedric,
    OCF_ArtifactAedric_AmuletKings,
    OCF_ArtifactAedric_AurielShield,
    OCF_ArtifactAedric_Crusader,
    OCF_ArtifactAedric_CrusaderBoots,
    OCF_ArtifactAedric_CrusaderCuirass,
    OCF_ArtifactAedric_CrusaderGauntlets,
    OCF_ArtifactAedric_CrusaderHelm,
    OCF_ArtifactAedric_CrusaderShield,
    OCF_ArtifactAedric_LordMail,
    OCF_ArtifactAedric_Morihaus,
    OCF_ArtifactAedric_MorihausCuirass,
    OCF_ArtifactAedric_RingPhynaster,
    OCF_ArtifactAedric_RingWarlock,
    OCF_ArtifactAedric_RingWind,
    OCF_ArtifactDaedric,
    OCF_ArtifactDaedric_EbonyMail,
    OCF_ArtifactDaedric_GrayCowlNocturnal,
    OCF_ArtifactDaedric_MasqueClavicusVile,
    OCF_ArtifactDaedric_Nightingale,
    OCF_ArtifactDaedric_NightingaleBoots,
    OCF_ArtifactDaedric_NightingaleCuirass,
    OCF_ArtifactDaedric_NightingaleGauntlets,
    OCF_ArtifactDaedric_NightingaleHelmet,
    OCF_ArtifactDaedric_RingHircine,
    OCF_ArtifactDaedric_RingKhajiit,
    OCF_ArtifactDaedric_RingMoonStar,
    OCF_ArtifactDaedric_RingNamira,
    OCF_ArtifactDaedric_SanctuaryAmulet,
    OCF_ArtifactDaedric_SaviorHide,
    OCF_ArtifactDaedric_Spellbreaker,
    OCF_ArtifactDwarven,
    OCF_ArtifactDwarven_AetherialCrown,
    OCF_ArtifactDwarven_AetherialShield,
    OCF_ArtifactDwarven_VisageMzund,
    OCF_ArtifactDwarven_Wraithguard,
    OCF_ArtifactLegendary,
    OCF_ArtifactLegendary_AdamantiumHelmTohan,
    OCF_ArtifactLegendary_BloodwormHelm,
    OCF_ArtifactLegendary_BootsApostle,
    OCF_ArtifactLegendary_BootsBlindingSpeed,
    OCF_ArtifactLegendary_DragonPriestMask,
    OCF_ArtifactLegendary_DragonboneCuirass,
    OCF_ArtifactLegendary_EleidonWard,
    OCF_ArtifactLegendary_FistsRandagulf,
    OCF_ArtifactLegendary_GaulderAmulet,
    OCF_ArtifactLegendary_HelmOreynBearclaw,
    OCF_ArtifactLegendary_HelmTiberSeptim,
    OCF_ArtifactLegendary_JaggedCrown,
    OCF_ArtifactLegendary_NecromancerAmulet,
    OCF_ArtifactLegendary_RingMasser,
    OCF_ArtifactLegendary_RingMentor,
    OCF_ArtifactLegendary_RingVampiric,
    OCF_ArtifactLegendary_RingVipereye,
    OCF_ArtifactLegendary_RingZurinArctus,
    OCF_ArtifactLegendary_Ysgramor,
    OCF_ArtifactLegendary_YsgramorShield,
    OCF_BagTypeBackpack,
    OCF_BagTypeBandolier,
    OCF_BagTypeBelt,
    OCF_BodyTypeCollar,
    OCF_BodyTypeCorset,
    OCF_BodyTypeDress,
    OCF_BodyTypeLingerie,
    OCF_BodyTypeMantle,
    OCF_BodyTypePants,
    OCF_BodyTypePauldron,
    OCF_BodyTypePauldronL,
    OCF_BodyTypePauldronLR,
    OCF_BodyTypePauldronR,
    OCF_BodyTypeRobes,
    OCF_BodyTypeScarf,
    OCF_BodyTypeSkirt,
    OCF_BodyTypeTail,
    OCF_BodyTypeTailReal,
    OCF_BodyTypeTasset,
    OCF_BodyTypeTorc,
    OCF_BodyTypeTorso,
    OCF_BodyTypeUnderwear_FullF,
    OCF_BodyTypeWings,
    OCF_BodyTypeWingsJewelry,
    OCF_BodyTypeWingsReal,
    OCF_BookTextMap,
    OCF_FeetTypeFootwraps,
    OCF_FeetTypeHeels,
    OCF_FeetTypeHeelsBoots,
    OCF_FeetTypeSabatons,
    OCF_FeetTypeSandals,
    OCF_FeetTypeShoes,
    OCF_FeetTypeStockings,
    OCF_HandTypeArmlet,
    OCF_HandTypeBandage,
    OCF_HandTypeBracer,
    OCF_HandTypeClaws,
    OCF_HandTypeCuffs,
    OCF_HandTypeGloves,
    OCF_HandTypeSleeves,
    OCF_HeadTypeBandage,
    OCF_HeadTypeBandana,
    OCF_HeadTypeBarrette,
    OCF_HeadTypeBlindfold,
    OCF_HeadTypeEarsReal,
    OCF_HeadTypeEyePatch,
    OCF_HeadTypeGag,
    OCF_HeadTypeGoggles,
    OCF_HeadTypeHalo,
    OCF_HeadTypeHat,
    OCF_HeadTypeHood,
    OCF_HeadTypeHorns,
    OCF_HeadTypeHornsAntlers,
    OCF_HeadTypeMask,
    OCF_HeadTypeMaskEyes,
    OCF_HeadTypeMaskFull,
    OCF_HeadTypeMaskHood,
    OCF_HeadTypeMaskMouth,
    OCF_HeadTypeWig,
    OCF_IngrRemains_BoneSkull_Troll,
    OCF_MiscEmptyVessel_Flask,
    OCF_MiscEmptyVessel_Jar,
    OCF_MiscHorseGear,
    OCF_MiscJarBug,
    OCF_Placeholder_BuildingPart,
    OCF_Placeholder_Filter,
    OCF_Placeholder_Separate,
    OCF_Relic,
    OCF_RelicAyleid,
    OCF_RelicDaedric,
    OCF_RelicDunmer,
    OCF_RelicFalmer,
    OCF_RelicImperial,
    OCF_RelicNordic,
    OCF_Replica,
    OCF_ReplicaAedric,
    OCF_ReplicaAedric_AmuletKings,
    OCF_ReplicaAedric_AurielShield,
    OCF_ReplicaAedric_Crusader,
    OCF_ReplicaAedric_CrusaderBoots,
    OCF_ReplicaAedric_CrusaderCuirass,
    OCF_ReplicaAedric_CrusaderGauntlets,
    OCF_ReplicaAedric_CrusaderHelm,
    OCF_ReplicaAedric_CrusaderShield,
    OCF_ReplicaAedric_LordMail,
    OCF_ReplicaAedric_Morihaus,
    OCF_ReplicaAedric_MorihausCuirass,
    OCF_ReplicaAedric_RingPhynaster,
    OCF_ReplicaAedric_RingWarlock,
    OCF_ReplicaAedric_RingWind,
    OCF_ReplicaAyleid,
    OCF_ReplicaDaedric,
    OCF_ReplicaDaedric_EbonyMail,
    OCF_ReplicaDaedric_MasqueClavicusVile,
    OCF_ReplicaDaedric_Nightingale,
    OCF_ReplicaDaedric_NightingaleBoots,
    OCF_ReplicaDaedric_NightingaleCuirass,
    OCF_ReplicaDaedric_NightingaleGauntlets,
    OCF_ReplicaDaedric_NightingaleHelmet,
    OCF_ReplicaDaedric_RingHircine,
    OCF_ReplicaDaedric_RingKhajiit,
    OCF_ReplicaDaedric_RingNamira,
    OCF_ReplicaDaedric_SanctuaryAmulet,
    OCF_ReplicaDaedric_SaviorHide,
    OCF_ReplicaDaedric_Spellbreaker,
    OCF_ReplicaDunmer,
    OCF_ReplicaDwarven,
    OCF_ReplicaDwarven_AetherialCrown,
    OCF_ReplicaDwarven_AetherialShield,
    OCF_ReplicaDwarven_VisageMzund,
    OCF_ReplicaDwarven_Wraithguard,
    OCF_ReplicaImperial,
    OCF_ReplicaLegendary,
    OCF_ReplicaLegendary_AdamantiumHelmTohan,
    OCF_ReplicaLegendary_BloodwormHelm,
    OCF_ReplicaLegendary_BootsApostle,
    OCF_ReplicaLegendary_BootsBlindingSpeed,
    OCF_ReplicaLegendary_DragonPriestMask,
    OCF_ReplicaLegendary_DragonboneCuirass,
    OCF_ReplicaLegendary_EleidonWard,
    OCF_ReplicaLegendary_FistsRandagulf,
    OCF_ReplicaLegendary_GaulderAmulet,
    OCF_ReplicaLegendary_HelmOreynBearclaw,
    OCF_ReplicaLegendary_HelmTiberSeptim,
    OCF_ReplicaLegendary_JaggedCrown,
    OCF_ReplicaLegendary_NecromancerAmulet,
    OCF_ReplicaLegendary_RingMentor,
    OCF_ReplicaLegendary_RingVampiric,
    OCF_ReplicaLegendary_RingVipereye,
    OCF_ReplicaLegendary_RingZurinArctus,
    OCF_ReplicaLegendary_Ysgramor,
    OCF_ReplicaLegendary_YsgramorShield,
    OCF_ReplicaNordic,
    OCF_ShieldTypeBuckler,
    OCF_ShieldTypeKite,
    OCF_ShieldTypeSpiked,
    OCF_ShieldTypeTower,
    OCF_Tool,
    OCF_ToolAlchemy,
    OCF_ToolCompass,
    OCF_ToolExtractor,
    OCF_ToolLantern,
    OCF_ToolLanternPaper,
    OCF_ToolSpyglass,
    OCF_ToolWalkingStick,
    OCF_VesselBottleSkooma,
    OCF_VesselFlask,
    OCF_VesselWaterskin,
    OCF_WeapThrowable,
    VendorItemClothing,
    WAF_ClothingAccessories,
    WAF_ClothingCloak,
    WAF_ClothingMedicalHealing,
    WAF_ClothingPouch,
    WAF_FingerlessGauntletsBracers,
    WAF_SpikedGauntletGloves,
}
