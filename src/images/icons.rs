//! All usable icons are defined here as enum variants.
//!
//! To add a new icon, you must add a variant name here AND add the variant to
//! `fallback()`, so the HUD renders something if the file is missing (e.g., an
//! icon pack dose not include it). If the icon is added to the core set in the
//! main mod, it is its own fallback and can be a fallback target. (A test
//! validates that all fallbacks are valid.)
//!
//! You should then add a file for the icon in one of the icon packs or the core
//! set. The name of the file *must* be the enum variation name in snake_case.
//! That is, an icon variant named `SnakeCase` maps to `snake_case.svg`.
//!
//! When naming icons, follow the principle of most general -> most specific.
//! For example `ArmorLightHands` starts with the general category of armor,
//! then narrows it down to light armor, then picks out hands specifically. This
//! approach makes related icons sort together, which makes them easier to edit
//! or preview in groups.
//!
//! Top-level icon name categories sort of match data types in the data/
//! submodule. Ammo, Armor, Food (Drink also categorized here), Potion, Power,
//! Shout, Spell, Weapon. This could be tidier.

use strum::{Display, EnumString, EnumVariantNames};

/// The Icon enum. Each variant maps to a known icon type.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, EnumString, EnumVariantNames, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Icon {
    Alteration,
    AmmoArrow,
    AmmoArrowBodkin,
    AmmoArrowBroadhead,
    AmmoArrowCrescent,
    AmmoArrowFire,
    AmmoArrowHammerhead,
    AmmoArrowPractice,
    AmmoArrowWhistle,
    AmmoBolt,
    AmmoBullet,
    AmmoDart,
    AmmoSlingshot,
    ArmorAmulet,
    ArmorBackpack,
    ArmorBelt,
    ArmorBracelet,
    ArmorCirclet,
    ArmorCloak,
    ArmorClothingFeet,
    ArmorClothingHands,
    ArmorClothingHead,
    ArmorClothing,
    ArmorEarring,
    ArmorHeavyFeet,
    ArmorHeavyHands,
    ArmorHeavyHead,
    ArmorHeavy,
    ArmorLightFeet,
    ArmorLightHands,
    ArmorLightHead,
    ArmorLight,
    ArmorMask,
    ArmorQuiver,
    ArmorRing,
    ArmorRobes,
    ArmorShieldHeavy,
    ArmorShieldLight,
    Conjuration,
    Destruction,
    DrinkMead,
    DrinkTea,
    DrinkWater,
    DrinkWine,
    FoodBread,
    FoodCarrot,
    FoodCheese,
    FoodFish,
    FoodMeat,
    FoodPie,
    FoodStew,
    Food,
    HandToHand,
    #[default]
    IconDefault,
    Illusion,
    MiscLantern,
    MiscCampfire,
    MiscLute,
    MiscTent,
    PotionDefault,
    PotionHealth,
    PotionMagicka,
    PotionPoison,
    PotionResist,
    PotionResistFire,
    PotionResistFrost,
    PotionResistShock,
    PotionSkooma,
    PotionStamina,
    Power,
    PowerCraft,
    PowerFillBottles,
    PowerHorse,
    PowerPeek,
    PowerPray,
    PowerWash,
    PowerWerebear,
    PowerWerewolf,
    Restoration,
    Scroll,
    Shout,
    ShoutAnimalAllegiance,
    ShoutAuraWhisper,
    ShoutBattleFury,
    ShoutBendWill,
    ShoutBreathAttack,
    ShoutCallDragon,
    ShoutCallOfValor,
    ShoutClearSkies,
    ShoutCyclone,
    ShoutDisarm,
    ShoutDismay,
    ShoutDragonAspect,
    ShoutDragonrend,
    ShoutDrainVitality,
    ShoutElementalFury,
    ShoutIceForm,
    ShoutKynesPeace,
    ShoutMarkedForDeath,
    ShoutPhantomForm,
    ShoutSlowtime,
    ShoutSoulTear,
    ShoutStormcall,
    ShoutSummonDurnehviir,
    ShoutThrowVoice,
    ShoutUnrelentingForce,
    ShoutWhirlwindSprint,
    // Stormcrown's modest additions.
    ShoutLightningBreath,
    ShoutPoisonBreath,
    // These are Thunderchild shouts.
    ShoutAlessiasLove,
    ShoutAnnihilate,
    ShoutArcaneHelix,
    ShoutArmageddon,
    ShoutCurse,
    ShoutDanceOfTheDead,
    ShoutEarthquake,
    ShoutEssenceRip,
    ShoutEvocation,
    ShoutGeomagnetism,
    ShoutIceborn,
    ShoutJonesShadow,
    ShoutKingsbane,
    ShoutLifestream,
    ShoutLightningShield,
    ShoutOblivion,
    ShoutPhantomDecoy,
    ShoutRiftwalk,
    ShoutShattersphere,
    ShoutShorsWrath,
    ShoutShroudOfSnowfall,
    ShoutSpeakUntoTheStars,
    ShoutSplinterTwins,
    ShoutStormblast,
    ShoutTheConqueror,
    ShoutTrueshot,
    ShoutWailOfTheBanshee,
    ShoutWanderlust,
    ShoutWarcry,

    // SpellArcane,
    SpellArclight,
    // SpellBlast, // not yet used
    SpellBleed,
    // SpellBolt,           // not yet used
    // SpellChainLightning, // not yet used
    SpellCircle,
    SpellConstellation,
    SpellControl,
    SpellCure,
    SpellDetect,
    // SpellDrain,
    SpellEagleEye,
    SpellEarth,
    SpellEvade,
    SpellFear,
    SpellFeather,
    SpellFire,
    // SpellFireDual, // not yet used
    // SpellFireWall, // not yet used
    // SpellFireball, // not yet used
    SpellFrost,
    // SpellFrostWall, // not yet used
    SpellHeal,
    SpellHoly,
    // SpellIceShard, // not yet used
    SpellLeaf,   // not yet used
    SpellLeaves, // not yet used
    SpellLight,
    // SpellLightning, // not yet used
    // SpellLightningBall, // not yet used
    SpellLightningBlast,
    // SpellMeteor, // not yet used
    SpellMoon,
    SpellDesecration,
    SpellParalyze,
    SpellPoison,
    SpellReanimate,
    SpellReflect,
    SpellRoot,
    SpellRune,
    SpellShadow,
    SpellSharpen,
    SpellShock,
    // SpellShockStrong, // not yet used
    SpellSilence,
    SpellSlow,
    // SpellSmoke, // not yet used
    SpellSoultrap,
    SpellSprint,
    SpellStamina,
    SpellStars,
    SpellSummon,
    SpellSun,
    SpellTeleport,
    SpellTime,
    SpellVampire,
    SpellWard,
    SpellWater,
    // SpellWave, // not yet used
    SpellWind,
    SpellWisp,
    ToolFishingRod,
    // ToolHammer, // not yet used
    ToolPickaxe,
    ToolShovel,
    ToolSickle,
    MiscTorch,
    WeaponAxeOneHanded,
    WeaponAxeTwoHanded,
    WeaponBow,
    WeaponClaw,
    WeaponCrossbow,
    WeaponDagger,
    WeaponFlail,
    WeaponGrenade,
    WeaponGun,
    WeaponHalberd,
    WeaponHammer,
    WeaponKatana,
    WeaponLance,
    WeaponMace,
    WeaponPike,
    WeaponQuarterstaff,
    WeaponRapier,
    WeaponScythe,
    WeaponStaff,
    WeaponSwordOneHanded,
    WeaponSwordTwoHanded,
    WeaponWhip,
    WeaponWoodAxe,
}

impl Icon {
    /// Get the SVG filename for this icon. Programmatically derived.
    pub fn icon_file(&self) -> String {
        format!("{self}.svg")
    }

    /// Fall back from any icon to one in the core set guaranteed to come with the base HUD.
    /// Fallbacks for the base icons are provided for some, but unlikely to have hits in the
    /// case where the user has nuked the base icons set. People who do that get what they deserve.
    pub fn fallback(&self) -> Icon {
        match self {
            // grouping logically, starting with magic schools
            Icon::Alteration => Icon::Scroll,
            Icon::Conjuration => Icon::Scroll,
            Icon::Destruction => Icon::Scroll,
            Icon::Illusion => Icon::Scroll,
            Icon::Restoration => Icon::Scroll,

            Icon::AmmoArrow => Icon::AmmoArrow,
            Icon::AmmoBullet => Icon::AmmoArrow,
            Icon::AmmoBolt => Icon::AmmoArrow,
            Icon::AmmoDart => Icon::AmmoArrow,
            Icon::AmmoSlingshot => Icon::AmmoArrow,
            Icon::AmmoArrowBodkin => Icon::AmmoArrow,
            Icon::AmmoArrowBroadhead => Icon::AmmoArrow,
            Icon::AmmoArrowHammerhead => Icon::AmmoArrow,
            Icon::AmmoArrowCrescent => Icon::AmmoArrow,
            Icon::AmmoArrowFire => Icon::AmmoArrow,
            Icon::AmmoArrowWhistle => Icon::AmmoArrow,
            Icon::AmmoArrowPractice => Icon::AmmoArrow,

            // All armor becomes the heavy armor icon.
            Icon::ArmorAmulet => Icon::ArmorHeavy,
            Icon::ArmorBackpack => Icon::ArmorHeavy,
            Icon::ArmorBelt => Icon::ArmorHeavy,
            Icon::ArmorBracelet => Icon::ArmorHeavy,
            Icon::ArmorCirclet => Icon::ArmorHeavy,
            Icon::ArmorCloak => Icon::ArmorHeavy,
            Icon::ArmorClothingFeet => Icon::ArmorHeavy,
            Icon::ArmorClothingHands => Icon::ArmorHeavy,
            Icon::ArmorClothingHead => Icon::ArmorHeavy,
            Icon::ArmorClothing => Icon::ArmorHeavy,
            Icon::ArmorEarring => Icon::ArmorHeavy,
            Icon::ArmorHeavyFeet => Icon::ArmorHeavy,
            Icon::ArmorHeavyHands => Icon::ArmorHeavy,
            Icon::ArmorHeavyHead => Icon::ArmorHeavy,
            Icon::ArmorHeavy => Icon::ArmorHeavy,
            Icon::ArmorLightFeet => Icon::ArmorHeavy,
            Icon::ArmorLightHands => Icon::ArmorHeavy,
            Icon::ArmorLightHead => Icon::ArmorHeavy,
            Icon::ArmorLight => Icon::ArmorHeavy,
            Icon::ArmorMask => Icon::ArmorHeavy,
            Icon::ArmorQuiver => Icon::ArmorHeavy,
            Icon::ArmorRing => Icon::ArmorHeavy,
            Icon::ArmorRobes => Icon::ArmorHeavy,
            Icon::ArmorShieldHeavy => Icon::ArmorHeavy,
            Icon::ArmorShieldLight => Icon::ArmorHeavy,

            Icon::DrinkMead => Icon::Food,
            Icon::DrinkTea => Icon::Food,
            Icon::DrinkWater => Icon::Food,
            Icon::DrinkWine => Icon::Food,
            Icon::FoodBread => Icon::Food,
            Icon::FoodCarrot => Icon::Food,
            Icon::FoodCheese => Icon::Food,
            Icon::FoodFish => Icon::Food,
            Icon::FoodMeat => Icon::Food,
            Icon::FoodPie => Icon::Food,
            Icon::FoodStew => Icon::Food,
            Icon::Food => Icon::Food,

            Icon::HandToHand => Icon::HandToHand,
            Icon::IconDefault => Icon::IconDefault,

            Icon::MiscCampfire => Icon::IconDefault,
            Icon::MiscLantern => Icon::MiscLantern,
            Icon::MiscLute => Icon::IconDefault,
            Icon::MiscTent => Icon::IconDefault,

            Icon::ToolFishingRod => Icon::WeaponSwordOneHanded,
            Icon::ToolPickaxe => Icon::WeaponAxeTwoHanded,
            Icon::ToolShovel => Icon::IconDefault,
            Icon::ToolSickle => Icon::IconDefault,

            Icon::PotionDefault => Icon::PotionDefault,
            Icon::PotionHealth => Icon::PotionDefault,
            Icon::PotionMagicka => Icon::PotionDefault,
            Icon::PotionPoison => Icon::PotionDefault,
            Icon::PotionResist => Icon::PotionDefault,
            Icon::PotionResistFire => Icon::PotionDefault,
            Icon::PotionResistFrost => Icon::PotionDefault,
            Icon::PotionResistShock => Icon::PotionDefault,
            Icon::PotionSkooma => Icon::PotionDefault,
            Icon::PotionStamina => Icon::PotionDefault,

            Icon::Power => Icon::Power,
            Icon::PowerCraft => Icon::Power,
            Icon::PowerFillBottles => Icon::Power,
            Icon::PowerHorse => Icon::Power,
            Icon::PowerPeek => Icon::Power,
            Icon::PowerPray => Icon::Power,
            Icon::PowerWash => Icon::Power,
            Icon::PowerWerebear => Icon::Power,
            Icon::PowerWerewolf => Icon::Power,

            Icon::Scroll => Icon::Scroll,

            // Shout. Shout. Let it all out.
            Icon::Shout => Icon::Shout,
            Icon::ShoutAnimalAllegiance => Icon::Shout,
            Icon::ShoutAuraWhisper => Icon::Shout,
            Icon::ShoutBattleFury => Icon::Shout,
            Icon::ShoutBendWill => Icon::Shout,
            Icon::ShoutBreathAttack => Icon::Shout,
            Icon::ShoutCallDragon => Icon::Shout,
            Icon::ShoutCallOfValor => Icon::Shout,
            Icon::ShoutClearSkies => Icon::Shout,
            Icon::ShoutCyclone => Icon::Shout,
            Icon::ShoutDisarm => Icon::Shout,
            Icon::ShoutDismay => Icon::Shout,
            Icon::ShoutDragonAspect => Icon::Shout,
            Icon::ShoutDragonrend => Icon::Shout,
            Icon::ShoutDrainVitality => Icon::Shout,
            Icon::ShoutElementalFury => Icon::Shout,
            Icon::ShoutIceForm => Icon::Shout,
            Icon::ShoutKynesPeace => Icon::Shout,
            Icon::ShoutMarkedForDeath => Icon::Shout,
            Icon::ShoutPhantomForm => Icon::Shout,
            Icon::ShoutSlowtime => Icon::Shout,
            Icon::ShoutSoulTear => Icon::Shout,
            Icon::ShoutStormcall => Icon::Shout,
            Icon::ShoutSummonDurnehviir => Icon::Shout,
            Icon::ShoutThrowVoice => Icon::Shout,
            Icon::ShoutUnrelentingForce => Icon::Shout,
            Icon::ShoutWhirlwindSprint => Icon::Shout,
            // stormcrown
            Icon::ShoutLightningBreath => Icon::Shout,
            Icon::ShoutPoisonBreath => Icon::Shout,
            // thunderchild's massive shout list
            Icon::ShoutAlessiasLove => Icon::Shout,
            Icon::ShoutAnnihilate => Icon::Shout,
            Icon::ShoutArcaneHelix => Icon::Shout,
            Icon::ShoutArmageddon => Icon::Shout,
            Icon::ShoutCurse => Icon::Shout,
            Icon::ShoutDanceOfTheDead => Icon::Shout,
            Icon::ShoutEarthquake => Icon::Shout,
            Icon::ShoutEssenceRip => Icon::Shout,
            Icon::ShoutEvocation => Icon::Shout,
            Icon::ShoutGeomagnetism => Icon::Shout,
            Icon::ShoutIceborn => Icon::Shout,
            Icon::ShoutJonesShadow => Icon::Shout,
            Icon::ShoutKingsbane => Icon::Shout,
            Icon::ShoutLifestream => Icon::Shout,
            Icon::ShoutLightningShield => Icon::Shout,
            Icon::ShoutOblivion => Icon::Shout,
            Icon::ShoutPhantomDecoy => Icon::Shout,
            Icon::ShoutRiftwalk => Icon::Shout,
            Icon::ShoutShattersphere => Icon::Shout,
            Icon::ShoutShorsWrath => Icon::Shout,
            Icon::ShoutShroudOfSnowfall => Icon::Shout,
            Icon::ShoutSpeakUntoTheStars => Icon::Shout,
            Icon::ShoutSplinterTwins => Icon::Shout,
            Icon::ShoutStormblast => Icon::Shout,
            Icon::ShoutTheConqueror => Icon::Shout,
            Icon::ShoutTrueshot => Icon::Shout,
            Icon::ShoutWailOfTheBanshee => Icon::Shout,
            Icon::ShoutWanderlust => Icon::Shout,
            Icon::ShoutWarcry => Icon::Shout,

            // Most spells won't ever reach this because they'll fall back to their
            // schools, but just in case.
            // Icon::SpellArcane => Icon::Destruction,
            Icon::SpellArclight => Icon::Destruction,
            // Icon::SpellBlast => Icon::Destruction,
            Icon::SpellBleed => Icon::Destruction,
            // Icon::SpellBolt => Icon::Destruction,
            // Icon::SpellChainLightning => Icon::Destruction,
            Icon::SpellCircle => Icon::Restoration,
            Icon::SpellConstellation => Icon::Destruction,
            Icon::SpellControl => Icon::Illusion,
            Icon::SpellCure => Icon::Restoration,
            Icon::SpellDesecration => Icon::Destruction,
            Icon::SpellDetect => Icon::Alteration,
            // Icon::SpellDrain => Icon::Destruction,
            Icon::SpellEagleEye => Icon::Alteration,
            Icon::SpellEarth => Icon::Destruction,
            Icon::SpellEvade => Icon::Illusion,
            Icon::SpellFear => Icon::Illusion,
            Icon::SpellFeather => Icon::Alteration,
            Icon::SpellFire => Icon::Destruction,
            // Icon::SpellFireball => Icon::Destruction,
            // Icon::SpellFireDual => Icon::Destruction,
            // Icon::SpellFireWall => Icon::Destruction,
            Icon::SpellFrost => Icon::Destruction,
            // Icon::SpellFrostWall => Icon::Destruction,
            Icon::SpellHeal => Icon::Restoration,
            Icon::SpellHoly => Icon::Restoration,
            // Icon::SpellIceShard => Icon::Destruction,
            Icon::SpellLeaf => Icon::Restoration,
            Icon::SpellLeaves => Icon::Restoration,
            Icon::SpellLight => Icon::Alteration,
            // Icon::SpellLightning => Icon::Destruction,
            // Icon::SpellLightningBall => Icon::Destruction,
            Icon::SpellLightningBlast => Icon::Destruction,
            // Icon::SpellMeteor => Icon::Destruction,
            Icon::SpellMoon => Icon::Destruction,
            Icon::SpellParalyze => Icon::Alteration,
            Icon::SpellPoison => Icon::Restoration,
            Icon::SpellReanimate => Icon::Conjuration,
            Icon::SpellReflect => Icon::Alteration,
            Icon::SpellRoot => Icon::Restoration,
            Icon::SpellRune => Icon::Destruction,
            Icon::SpellShadow => Icon::Destruction,
            Icon::SpellSharpen => Icon::Alteration,
            Icon::SpellShock => Icon::Destruction,
            // Icon::SpellShockStrong => Icon::Destruction,
            Icon::SpellSilence => Icon::Illusion,
            Icon::SpellSlow => Icon::Alteration,
            // Icon::SpellSmoke => Icon::Illusion,
            Icon::SpellSoultrap => Icon::Conjuration,
            Icon::SpellSprint => Icon::Alteration,
            Icon::SpellStamina => Icon::Restoration,
            Icon::SpellStars => Icon::Destruction,
            Icon::SpellSummon => Icon::Conjuration,
            Icon::SpellSun => Icon::Restoration,
            Icon::SpellTeleport => Icon::Alteration,
            Icon::SpellTime => Icon::Alteration,
            Icon::SpellVampire => Icon::Destruction,
            Icon::SpellWard => Icon::Restoration,
            Icon::SpellWater => Icon::Destruction,
            // Icon::SpellWave => Icon::Destruction,
            Icon::SpellWind => Icon::Destruction,
            Icon::SpellWisp => Icon::Illusion,

            Icon::MiscTorch => Icon::MiscTorch, // core set

            // weapons
            Icon::WeaponAxeOneHanded => Icon::WeaponAxeOneHanded, // core set
            Icon::WeaponAxeTwoHanded => Icon::WeaponAxeTwoHanded, // core set
            Icon::WeaponBow => Icon::WeaponBow,                   // core set
            Icon::WeaponClaw => Icon::WeaponSwordOneHanded,
            Icon::WeaponCrossbow => Icon::WeaponCrossbow, // core set
            Icon::WeaponDagger => Icon::WeaponDagger,     // core set
            Icon::WeaponFlail => Icon::WeaponMace,
            Icon::WeaponGrenade => Icon::WeaponDagger,
            Icon::WeaponGun => Icon::WeaponBow,
            Icon::WeaponHalberd => Icon::WeaponHalberd, // core set
            Icon::WeaponHammer => Icon::WeaponMace,
            Icon::WeaponKatana => Icon::WeaponSwordOneHanded,
            Icon::WeaponLance => Icon::WeaponHalberd,
            Icon::WeaponMace => Icon::WeaponMace, // core set
            Icon::WeaponPike => Icon::WeaponAxeTwoHanded, // core set
            Icon::WeaponQuarterstaff => Icon::WeaponQuarterstaff, // core set
            Icon::WeaponRapier => Icon::WeaponRapier, // core set
            Icon::WeaponScythe => Icon::WeaponHalberd,
            Icon::WeaponStaff => Icon::WeaponStaff, // core set
            Icon::WeaponSwordOneHanded => Icon::WeaponAxeOneHanded, // core set
            Icon::WeaponSwordTwoHanded => Icon::WeaponAxeTwoHanded, // core set
            Icon::WeaponWhip => Icon::WeaponWhip,   // core set
            Icon::WeaponWoodAxe => Icon::WeaponAxeOneHanded,
        }
    }
}

/// For tests, pick a random icon to use for randomly-generated items.
#[cfg(test)]
pub fn random_icon() -> Icon {
    use std::str::FromStr;

    use rand::prelude::*;
    use strum::VariantNames;

    if let Some(variant) = Icon::VARIANTS.choose(&mut rand::thread_rng()) {
        Icon::from_str(variant).unwrap_or(Icon::WeaponSwordTwoHanded)
    } else {
        Icon::WeaponSwordOneHanded
    }
}

/// Check if an icon is in the core set.
#[cfg(test)]
pub fn is_in_core_set(icon: &Icon) -> bool {
    matches!(
        icon,
        Icon::Alteration
            | Icon::AmmoArrow
            | Icon::ArmorClothing
            | Icon::ArmorHeavy
            | Icon::ArmorLight
            | Icon::ArmorMask
            | Icon::ArmorShieldHeavy
            | Icon::Conjuration
            | Icon::Destruction
            | Icon::Food
            | Icon::HandToHand
            | Icon::IconDefault
            | Icon::Illusion
            | Icon::PotionDefault
            | Icon::PotionHealth
            | Icon::PotionMagicka
            | Icon::PotionPoison
            | Icon::PotionResistFire
            | Icon::PotionResistFrost
            | Icon::PotionResistShock
            | Icon::PotionStamina
            | Icon::Power
            | Icon::Restoration
            | Icon::Scroll
            | Icon::Shout
            | Icon::MiscLantern
            | Icon::MiscTorch
            | Icon::SpellFire
            | Icon::SpellFrost
            | Icon::SpellShock
            | Icon::WeaponAxeOneHanded
            | Icon::WeaponAxeTwoHanded
            | Icon::WeaponBow
            | Icon::WeaponClaw
            | Icon::WeaponCrossbow
            | Icon::WeaponDagger
            | Icon::WeaponHalberd
            | Icon::WeaponKatana
            | Icon::WeaponMace
            | Icon::WeaponPike
            | Icon::WeaponQuarterstaff
            | Icon::WeaponRapier
            | Icon::WeaponStaff
            | Icon::WeaponSwordOneHanded
            | Icon::WeaponSwordTwoHanded
            | Icon::WeaponWhip
    )
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::str::FromStr;

    use strum::VariantNames;

    use super::*;

    #[test]
    fn validate_fallbacks() {
        let wrong: Vec<&&str> = Icon::VARIANTS
            .iter()
            .filter(|variant| {
                let icon =
                    Icon::from_str(variant).expect("icon names should darn well turn into icons");
                let fpath: PathBuf = [
                    "installer/core/SKSE/plugins/resources/icons/",
                    icon.icon_file().as_str(),
                ]
                .iter()
                .collect();

                if fpath.exists() != is_in_core_set(&icon) {
                    eprintln!(
                        "{icon:?} wrong: file is in core set={} but function says {}; {}",
                        fpath.exists(),
                        is_in_core_set(&icon),
                        icon.icon_file()
                    );
                    true
                } else {
                    false
                }
            })
            .collect();
        assert!(wrong.is_empty());

        // now make sure every fallback is okay
        let bad_fallback: Vec<&&str> = Icon::VARIANTS
            .iter()
            .filter(|variant| {
                let icon =
                    Icon::from_str(variant).expect("icon names should darn well turn into icons");
                if is_in_core_set(&icon.fallback()) {
                    false
                } else {
                    eprintln!("{icon:?} has bad fallback: {:?}", icon.fallback());
                    true
                }
            })
            .collect();
        assert!(bad_fallback.is_empty());
    }

    #[test]
    fn soulsy_pack_complete() {
        let icon_paths = [
            "installer/core/SKSE/plugins/resources/icons/",
            "installer/icon-pack-soulsy",
        ];
        let missing: Vec<&&str> = Icon::VARIANTS
            .iter()
            .filter(|variant| {
                let icon =
                    Icon::from_str(variant).expect("icon names should darn well turn into icons");
                let found = icon_paths.iter().any(|prefix| {
                    let fpath: PathBuf = [prefix, icon.icon_file().as_str()].iter().collect();
                    fpath.exists()
                });
                if !found {
                    eprintln!("svg for {icon:?} missing: {}", icon.icon_file());
                }
                !found
            })
            .collect();
        assert!(missing.is_empty());
    }

    #[test]
    fn thicc_pack_complete() {
        let icon_paths = [
            "installer/core/SKSE/plugins/resources/icons/",
            "installer/icon-pack-thicc",
        ];
        let missing: Vec<String> = Icon::VARIANTS
            .iter()
            .filter_map(|variant| {
                let icon =
                    Icon::from_str(variant).expect("icon names should darn well turn into icons");
                let found = icon_paths.iter().any(|prefix| {
                    let fpath: PathBuf = [prefix, icon.icon_file().as_str()].iter().collect();
                    fpath.exists()
                });
                if !found {
                    Some(format!("MISSING {icon:?}: {}", icon.icon_file()))
                } else {
                    None
                }
            })
            .collect();
        assert!(missing.is_empty(), "{missing:#?}");
    }

    #[test]
    #[ignore]
    fn emit_icon_files() {
        Icon::VARIANTS.iter().for_each(|xs| {
            eprintln!("{xs}.svg");
        });
        unreachable!(); // forces a test failure and output of the lines above
    }
}
