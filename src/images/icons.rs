use strum::{Display, EnumString, EnumVariantNames};

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, EnumString, EnumVariantNames, Display)]
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
    ShoutIceForm,
    ShoutMarkedForDeath,
    ShoutStormcall,
    Soulgem,
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
    SpellDrain,
    SpellEagleEye,
    SpellEarth,
    SpellElementalFury,
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
        match &self {
            // Icon::SpellArcane => "spell_arcane.svg".to_string(),
            // Icon::SpellBlast => "spell_blast.svg".to_string(),
            // Icon::SpellBolt => "spell_bolt.svg".to_string(),
            // Icon::SpellChainLightning => "spell_chain_lightning.svg".to_string(),
            // Icon::SpellFireDual => "spell_fire_dual.svg".to_string(),
            // Icon::SpellFireWall => "spell_fire_wall.svg".to_string(),
            // Icon::SpellFireball => "spell_fireball.svg".to_string(),
            // Icon::SpellFrostWall => "spell_frost_wall.svg".to_string(),
            // Icon::SpellIceShard => "spell_ice_shard.svg".to_string(),
            // Icon::SpellShockStrong => "spell_shock_strong.svg".to_string(),
            // Icon::SpellSmoke => "spell_smoke.svg".to_string(),
            // Icon::SpellWave => "spell_wave.svg".to_string(),
            Icon::Alteration => "alteration.svg".to_string(),
            Icon::AmmoArrow => "ammo_arrow.svg".to_string(),
            Icon::AmmoBullet => "ammo_bullet.svg".to_string(),
            Icon::ArmorAmulet => "armor_amulet.svg".to_string(),
            Icon::ArmorBackpack => "armor_backpack.svg".to_string(),
            Icon::ArmorBelt => "armor_belt.svg".to_string(),
            Icon::ArmorBracelet => "armor_bracelet.svg".to_string(),
            Icon::ArmorCirclet => "armor_circlet.svg".to_string(),
            Icon::ArmorCloak => "armor_cloak.svg".to_string(),
            Icon::ArmorClothing => "armor_clothing.svg".to_string(),
            Icon::ArmorClothingFeet => "armor_clothing_feet.svg".to_string(),
            Icon::ArmorClothingHands => "armor_clothing_hands.svg".to_string(),
            Icon::ArmorClothingHead => "armor_clothing_head.svg".to_string(),
            Icon::ArmorEarring => "armor_earring.svg".to_string(),
            Icon::ArmorHeavy => "armor_heavy.svg".to_string(),
            Icon::ArmorHeavyFeet => "armor_heavy_feet.svg".to_string(),
            Icon::ArmorHeavyHands => "armor_heavy_hands.svg".to_string(),
            Icon::ArmorHeavyHead => "armor_heavy_head.svg".to_string(),
            Icon::ArmorLight => "armor_light.svg".to_string(),
            Icon::ArmorLightFeet => "armor_light_feet.svg".to_string(),
            Icon::ArmorLightHands => "armor_light_hands.svg".to_string(),
            Icon::ArmorLightHead => "armor_light_head.svg".to_string(),
            Icon::ArmorMask => "armor_mask.svg".to_string(),
            Icon::ArmorQuiver => "armor_quiver.svg".to_string(),
            Icon::ArmorRing => "armor_ring.svg".to_string(),
            Icon::ArmorRobes => "armor_robes.svg".to_string(),
            Icon::ArmorShieldHeavy => "armor_shield_heavy.svg".to_string(),
            Icon::ArmorShieldLight => "armor_shield_light.svg".to_string(),
            Icon::Conjuration => "conjuration.svg".to_string(),
            Icon::Destruction => "destruction.svg".to_string(),
            Icon::DrinkMead => "drink_mead.svg".to_string(),
            Icon::DrinkTea => "drink_tea.svg".to_string(),
            Icon::DrinkWater => "drink_water.svg".to_string(),
            Icon::DrinkWine => "drink_wine.svg".to_string(),
            Icon::Food => "food.svg".to_string(),
            Icon::FoodBread => "food_bread.svg".to_string(),
            Icon::FoodCarrot => "food_carrot.svg".to_string(),
            Icon::FoodCheese => "food_cheese.svg".to_string(),
            Icon::FoodFish => "food_fish.svg".to_string(),
            Icon::FoodMeat => "food_meat.svg".to_string(),
            Icon::FoodPie => "food_pie.svg".to_string(),
            Icon::FoodStew => "food_stew.svg".to_string(),
            Icon::HandToHand => "hand_to_hand.svg".to_string(),
            Icon::IconDefault => "icon_default.svg".to_string(),
            Icon::Illusion => "illusion.svg".to_string(),
            Icon::MiscLantern => "misc_lantern.svg".to_string(),
            Icon::MiscCampfire => "misc_campfire.svg".to_string(),
            Icon::MiscLute => "misc_lute.svg".to_string(),
            Icon::MiscTent => "misc_tent.svg".to_string(),
            Icon::PotionDefault => "potion_default.svg".to_string(),
            Icon::PotionHealth => "potion_health.svg".to_string(),
            Icon::PotionMagicka => "potion_magicka.svg".to_string(),
            Icon::PotionPoison => "potion_poison.svg".to_string(),
            Icon::PotionResist => "potion_resist.svg".to_string(),
            Icon::PotionResistFire => "potion_resist_fire.svg".to_string(),
            Icon::PotionResistFrost => "potion_resist_frost.svg".to_string(),
            Icon::PotionResistShock => "potion_resist_shock.svg".to_string(),
            Icon::PotionSkooma => "potion_skooma.svg".to_string(),
            Icon::PotionStamina => "potion_stamina.svg".to_string(),
            Icon::Power => "power.svg".to_string(),
            Icon::PowerCraft => "power_craft.svg".to_string(),
            Icon::PowerFillBottles => "power_fill_bottles.svg".to_string(),
            Icon::PowerHorse => "power_horse.svg".to_string(),
            Icon::PowerPeek => "power_horse.svg".to_string(),
            Icon::PowerPray => "power_pray.svg".to_string(),
            Icon::PowerWash => "power_wash.svg".to_string(),
            Icon::PowerWerebear => "power_werebear.svg".to_string(),
            Icon::PowerWerewolf => "power_werewolf.svg".to_string(),
            Icon::Restoration => "restoration.svg".to_string(),
            Icon::Scroll => "scroll.svg".to_string(),
            Icon::Shout => "shout.svg".to_string(),
            Icon::ShoutAnimalAllegiance => "shout_animal_allegiance.svg".to_string(),
            Icon::ShoutBreathAttack => "shout_breath_attack.svg".to_string(),
            Icon::ShoutCallDragon => "shout_call_dragon.svg".to_string(),
            Icon::ShoutClearSkies => "shout_clear_skies.svg".to_string(),
            Icon::ShoutCyclone => "shout_cyclone.svg".to_string(),
            Icon::ShoutIceForm => "shout_ice_form.svg".to_string(),
            Icon::ShoutMarkedForDeath => "shout_marked_death.svg".to_string(),
            Icon::ShoutStormcall => "shout_stormcall.svg".to_string(),
            Icon::Soulgem => "soulgem.svg".to_string(),
            Icon::SpellArclight => "spell_arclight.svg".to_string(),
            Icon::SpellBleed => "spell_bleed.svg".to_string(),
            Icon::SpellCircle => "spell_circle.svg".to_string(),
            Icon::SpellConstellation => "spell_constellation.svg".to_string(),
            Icon::SpellControl => "spell_control.svg".to_string(),
            Icon::SpellCure => "spell_cure.svg".to_string(),
            Icon::SpellDesecration => "spell_necrotic.svg".to_string(),
            Icon::SpellDetect => "spell_detect.svg".to_string(),
            Icon::SpellDrain => "spell_drain.svg".to_string(),
            Icon::SpellEagleEye => "spell_eagleeye.svg".to_string(),
            Icon::SpellEarth => "spell_earth.svg".to_string(),
            Icon::SpellElementalFury => "spell_elementalfury.svg".to_string(),
            Icon::SpellEvade => "spell_evade.svg".to_string(),
            Icon::SpellFear => "spell_fear.svg".to_string(),
            Icon::SpellFeather => "spell_feather.svg".to_string(),
            Icon::SpellFire => "spell_fire.svg".to_string(),
            Icon::SpellFrost => "spell_frost.svg".to_string(),
            Icon::SpellHeal => "spell_heal.svg".to_string(),
            Icon::SpellHoly => "spell_holy.svg".to_string(),
            Icon::SpellLeaf => "spell_leaf.svg".to_string(),
            Icon::SpellLeaves => "spell_leaves.svg".to_string(),
            Icon::SpellLight => "spell_light.svg".to_string(),
            // Icon::SpellLightning => "spell_lightning.svg".to_string(),
            // Icon::SpellLightningBall => "spell_lightning_ball.svg".to_string(),
            Icon::SpellLightningBlast => "spell_lightning_blast.svg".to_string(),
            // Icon::SpellMeteor => "spell_meteor.svg".to_string(),
            Icon::SpellMoon => "spell_moon.svg".to_string(),
            Icon::SpellParalyze => "spell_paralyze.svg".to_string(),
            Icon::SpellPoison => "spell_poison.svg".to_string(),
            Icon::SpellReanimate => "spell_reanimate.svg".to_string(),
            Icon::SpellReflect => "spell_reflect.svg".to_string(),
            Icon::SpellRoot => "spell_root.svg".to_string(),
            Icon::SpellRune => "spell_rune.svg".to_string(),
            Icon::SpellShadow => "spell_shadow.svg".to_string(),
            Icon::SpellSharpen => "spell_sharpen.svg".to_string(),
            Icon::SpellShock => "spell_shock.svg".to_string(),
            Icon::SpellSilence => "spell_silence.svg".to_string(),
            Icon::SpellSlow => "spell_slow.svg".to_string(),
            Icon::SpellSoultrap => "spell_soultrap.svg".to_string(),
            Icon::SpellSprint => "spell_sprint.svg".to_string(),
            Icon::SpellStamina => "spell_stamina.svg".to_string(),
            Icon::SpellStars => "spell_stars.svg".to_string(),
            Icon::SpellSummon => "spell_summon.svg".to_string(),
            Icon::SpellSun => "spell_sun.svg".to_string(),
            Icon::SpellTeleport => "spell_teleport.svg".to_string(),
            Icon::SpellTime => "spell_time.svg".to_string(),
            Icon::SpellVampire => "spell_vampire.svg".to_string(),
            Icon::SpellWard => "spell_ward.svg".to_string(),
            Icon::SpellWater => "spell_water.svg".to_string(),
            Icon::SpellWind => "spell_wind.svg".to_string(),
            Icon::SpellWisp => "spell_wisp.svg".to_string(),
            Icon::ToolFishingRod => "tool_fishingrod.svg".to_string(),
            Icon::ToolPickaxe => "weapon_pickaxe.svg".to_string(),
            Icon::ToolShovel => "tool_shovel.svg".to_string(),
            Icon::ToolSickle => "tool_sickle.svg".to_string(),
            Icon::MiscTorch => "misc_torch.svg".to_string(),
            Icon::WeaponAxeOneHanded => "weapon_axe_one_handed.svg".to_string(),
            Icon::WeaponAxeTwoHanded => "weapon_axe_two_handed.svg".to_string(),
            Icon::WeaponBow => "weapon_bow.svg".to_string(),
            Icon::WeaponClaw => "weapon_claw.svg".to_string(),
            Icon::WeaponCrossbow => "weapon_crossbow.svg".to_string(),
            Icon::WeaponDagger => "weapon_dagger.svg".to_string(),
            Icon::WeaponFlail => "weapon_flail.svg".to_string(),
            Icon::WeaponGrenade => "weapon_grenade.svg".to_string(),
            Icon::WeaponGun => "weapon_gun.svg".to_string(),
            Icon::WeaponHalberd => "weapon_halberd.svg".to_string(),
            Icon::WeaponHammer => "weapon_hammer.svg".to_string(),
            Icon::WeaponKatana => "weapon_katana.svg".to_string(),
            Icon::WeaponLance => "weapon_lance.svg".to_string(),
            Icon::WeaponMace => "weapon_mace.svg".to_string(),
            Icon::WeaponPike => "weapon_pike.svg".to_string(),
            Icon::WeaponQuarterstaff => "weapon_quarterstaff.svg".to_string(),
            Icon::WeaponRapier => "weapon_rapier.svg".to_string(),
            Icon::WeaponScythe => "weapon_scythe.svg".to_string(),
            Icon::WeaponStaff => "weapon_staff.svg".to_string(),
            Icon::WeaponSwordOneHanded => "weapon_sword_one_handed.svg".to_string(),
            Icon::WeaponSwordTwoHanded => "weapon_sword_two_handed.svg".to_string(),
            Icon::WeaponWhip => "weapon_whip.svg".to_string(),
            Icon::WeaponWoodAxe => "weapon_woodaxe.svg".to_string(),
        }
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
            Icon::ShoutIceForm => Icon::Destruction,
            Icon::ShoutMarkedForDeath => Icon::Shout,
            Icon::ShoutStormcall => Icon::Shout,

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
            Icon::SpellDrain => Icon::Destruction,
            Icon::SpellEagleEye => Icon::Alteration,
            Icon::SpellEarth => Icon::Destruction,
            Icon::SpellElementalFury => Icon::Illusion,
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
    use std::str::FromStr;

    use rand::prelude::*;
    use strum::VariantNames;

    if let Some(variant) = Icon::VARIANTS.choose(&mut rand::thread_rng()) {
        Icon::from_str(variant).unwrap_or(Icon::WeaponSwordTwoHanded)
    } else {
        Icon::WeaponSwordOneHanded
    }
}
