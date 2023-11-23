use strum::{Display, EnumString, EnumVariantNames};

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, EnumString, EnumVariantNames, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Icon {
    Alteration,
    AmmoArrow,
    AmmoBullet,
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
    ShoutBreathAttack,
    ShoutCallDragon,
    ShoutClearSkies,
    ShoutCyclone,
    ShoutDismay,
    ShoutElementalFury,
    ShoutIceForm,
    ShoutMarkedForDeath,
    ShoutStormcall,
    ShoutUnrelentingForce,
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
    pub fn icon_file(&self) -> String {
        format!("{}.svg", self.to_string())
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
            Icon::ShoutBreathAttack => Icon::Shout,
            Icon::ShoutClearSkies => Icon::Shout,
            Icon::ShoutCyclone => Icon::Shout,
            Icon::ShoutCallDragon => Icon::Shout,
            Icon::ShoutDismay => Icon::Shout,
            Icon::ShoutElementalFury => Icon::Shout,
            Icon::ShoutIceForm => Icon::Destruction,
            Icon::ShoutMarkedForDeath => Icon::Shout,
            Icon::ShoutStormcall => Icon::Shout,
            Icon::ShoutUnrelentingForce => Icon::Shout,

            Icon::Soulgem => Icon::Conjuration,

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

#[cfg(test)]
pub fn random_icon() -> Icon {
    use rand::prelude::*;
    use std::str::FromStr;
    use strum::VariantNames;

    if let Some(variant) = Icon::VARIANTS.choose(&mut rand::thread_rng()) {
        Icon::from_str(variant).unwrap_or(Icon::WeaponSwordTwoHanded)
    } else {
        Icon::WeaponSwordOneHanded
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::str::FromStr;
    use strum::VariantNames;

    #[test]
    fn soulsy_pack_complete() {
        let icon_paths = vec![
            "data/SKSE/plugins/resources/icons/",
            "layouts/icon-pack-soulsy",
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
        assert!(missing.len() == 0);
    }

    #[test]
    #[ignore]
    fn thicc_pack_complete() {
        let icon_paths = vec![
            "data/SKSE/plugins/resources/icons/",
            "layouts/icon-pack-thicc",
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
        assert!(missing.len() == 0, "{missing:#?}");
    }

    #[test]
    #[ignore]
    fn emit_icon_files() {
        Icon::VARIANTS.iter().for_each(|xs| {
            eprintln!("{xs}.svg");
        });
        assert!(false);
    }
}
