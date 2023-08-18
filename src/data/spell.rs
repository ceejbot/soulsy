#![allow(non_snake_case, non_camel_case_types)]

use cxx::let_cxx_string;
use strum::Display;

use super::base::BaseType;
use super::color::InvColor;
use super::game_enums::{ActorValue, SpellArchetype};
use super::icons::Icon;
use super::weapon::WeaponType;
use super::HasIcon;
use crate::plugin::{formSpecToHudItem, Color};

// Spells must be classified by querying game data about actor values, resist types,
// and spell archetypes. SpellData holds Rust expressions of the C++ enum values.
// In all cases, we choose the primary actor value from the most expensive effect
// of a spell or potion.

/*
To get type of bound weapon:
look at effect.data-> associated item
bow vs sword vs axe vs battleaxe
archetype spawn hazard
look at asso item

chain lightning -> chain lightning (resist shock, skill level 50)

*/

#[derive(Default, Clone, Debug, Eq, Hash, PartialEq)]
pub struct SpellData {
    pub effect: ActorValue,
    pub secondary: ActorValue,
    pub twohanded: bool,
    pub school: School,
    pub level: MagicSpellLevel,
    pub archetype: SpellArchetype,
    pub damage: MagicDamageType,
    pub associated: String,
}

impl SpellData {
    pub fn from_game_data(
        effect: i32,
        effect2: i32,
        resist: i32,
        twohanded: bool,
        school: i32,
        level: u32,
        archetype: i32,
        associated: String,
    ) -> Self {
        let school = School::from(school);
        let effect = ActorValue::from(effect);
        let secondary = ActorValue::from(effect2);
        let resist = ActorValue::from(resist);
        let archetype = SpellArchetype::from(archetype);

        let damage = match resist {
            ActorValue::ResistFire => MagicDamageType::Fire,
            ActorValue::ResistFrost => MagicDamageType::Frost,
            ActorValue::ResistShock => MagicDamageType::Shock,
            ActorValue::ResistMagic => MagicDamageType::Magic,
            ActorValue::ResistDisease => MagicDamageType::Disease,
            ActorValue::PoisonResist => MagicDamageType::Poison,
            // ActorValue::SOMETHING => MagicDamageType::Sun, // TODO SSEdit inspection
            _ => MagicDamageType::None,
        };

        Self {
            effect,
            secondary,
            twohanded,
            school,
            archetype,
            level: level.into(),
            damage,
            associated: associated.clone(),
        }
    }
}

#[derive(Default, Clone, Debug, Eq, Hash, PartialEq)]
pub struct SpellType {
    pub data: SpellData,
    pub variant: SpellVariant,
}

impl SpellType {
    pub fn from_spell_data(data: SpellData) -> Self {
        // well, this will be fun™

        let variant = match data.archetype {
            SpellArchetype::ValueModifier => {
                if matches!(data.effect, ActorValue::Health)
                    && matches!(data.school, School::Restoration)
                {
                    Some(SpellVariant::Heal)
                } else if matches!(data.school, School::Destruction)
                    && matches!(data.effect, ActorValue::Health)
                {
                    Some(SpellVariant::Damage(data.damage.clone()))
                } else {
                    log::info!(
                        "classifying DualValueModifier spell; AV={}; damage={};",
                        data.effect,
                        data.damage
                    );
                    None
                }
            }
            SpellArchetype::DualValueModifier => {
                if matches!(data.school, School::Destruction)
                    && matches!(data.effect, ActorValue::Health)
                {
                    Some(SpellVariant::Damage(data.damage.clone()))
                } else {
                    log::info!(
                        "classifying DualValueModifier spell; AV={}; damage={};",
                        data.effect,
                        data.damage
                    );
                    None
                }
            }
            //SpellArchetype::Absorb => todo!(),
            //SpellArchetype::Banish => todo!(),
            //SpellArchetype::Calm => SpellVariant::Calm, //do I have one?
            SpellArchetype::BoundWeapon => {
                if !data.associated.is_empty() {
                    let_cxx_string!(form_spec = data.associated.clone());
                    let assoc = formSpecToHudItem(&form_spec);
                    match assoc.kind() {
                        BaseType::Weapon(w) => match w {
                            WeaponType::AxeOneHanded(_, _) => {
                                Some(SpellVariant::BoundWeapon(BoundType::WarAxe))
                            }
                            WeaponType::AxeTwoHanded(_, _) => {
                                Some(SpellVariant::BoundWeapon(BoundType::BattleAxe))
                            }
                            WeaponType::BowShort(_, _) => {
                                Some(SpellVariant::BoundWeapon(BoundType::Bow))
                            }
                            WeaponType::Bow(_, _) => {
                                Some(SpellVariant::BoundWeapon(BoundType::Bow))
                            }
                            WeaponType::Crossbow(_, _) => {
                                Some(SpellVariant::BoundWeapon(BoundType::Bow))
                            }
                            WeaponType::Dagger(_, _) => {
                                Some(SpellVariant::BoundWeapon(BoundType::Dagger))
                            }
                            WeaponType::Hammer(_, _) => {
                                Some(SpellVariant::BoundWeapon(BoundType::Hammer))
                            }
                            WeaponType::Mace(_, _) => {
                                Some(SpellVariant::BoundWeapon(BoundType::Mace))
                            }
                            WeaponType::SwordOneHanded(_, _) => {
                                Some(SpellVariant::BoundWeapon(BoundType::Sword))
                            }
                            WeaponType::SwordTwoHanded(_, _) => {
                                Some(SpellVariant::BoundWeapon(BoundType::Greatsword))
                            }
                            _ => Some(SpellVariant::BoundWeapon(BoundType::Unknown)),
                        },
                        _ => Some(SpellVariant::BoundWeapon(BoundType::Unknown)),
                    }
                } else {
                    Some(SpellVariant::BoundWeapon(BoundType::Unknown))
                }
            }
            SpellArchetype::CureDisease => Some(SpellVariant::Cure),
            SpellArchetype::CurePoison => Some(SpellVariant::Cure),
            SpellArchetype::CureParalysis => Some(SpellVariant::Cure),
            SpellArchetype::Demoralize => Some(SpellVariant::Demoralize),
            SpellArchetype::DetectLife => Some(SpellVariant::Detect),
            SpellArchetype::Guide => Some(SpellVariant::Guide),
            SpellArchetype::Light => Some(SpellVariant::Light),
            SpellArchetype::Reanimate => Some(SpellVariant::Reanimate),
            SpellArchetype::SoulTrap => Some(SpellVariant::SoulTrap),
            SpellArchetype::SummonCreature => Some(SpellVariant::Summon),
            SpellArchetype::Cloak => Some(SpellVariant::Cloak(data.damage.clone())),
            //SpellArchetype::CommandSummoned => todo!(),
            //SpellArchetype::Darkness => todo!(),
            //SpellArchetype::Disarm => todo!(),
            //SpellArchetype::Disguise => todo!(),
            //SpellArchetype::Dispel => todo!(),
            SpellArchetype::EnhanceWeapon => Some(SpellVariant::EnhanceWeapon),
            //SpellArchetype::Etherealize => todo!(),
            //SpellArchetype::Frenzy => todo!(),
            //SpellArchetype::GrabActor => todo!(),
            //SpellArchetype::Invisibility => todo!(),
            //SpellArchetype::Lock => todo!(),
            //SpellArchetype::NightEye => todo!(),
            //SpellArchetype::Open => todo!(),
            //SpellArchetype::Paralysis => todo!(),
            //SpellArchetype::Rally => todo!(),
            SpellArchetype::SlowTime => Some(SpellVariant::SlowTime),
            //SpellArchetype::SpawnHazard => todo!(), // frostwall and firewall here?
            //SpellArchetype::Telekinesis => todo!(),
            //SpellArchetype::TurnUndead => todo!(),
            _ => None,
        };

        let variant = if let Some(v) = variant {
            v
        } else {
            log::debug!("default spell variant; data: {data:?}");
            SpellVariant::Unknown
        };

        Self { data, variant }
    }
}

impl HasIcon for SpellType {
    fn color(&self) -> Color {
        match &self.variant {
            SpellVariant::Unknown => Color::default(),
            SpellVariant::BoundWeapon(_) => InvColor::Eldritch.color(),
            SpellVariant::Burden => Color::default(),
            SpellVariant::Cure => InvColor::Green.color(),
            SpellVariant::Damage(t) => match t {
                MagicDamageType::None => Color::default(),
                MagicDamageType::Disease => InvColor::Green.color(),
                MagicDamageType::Fire => InvColor::Fire.color(),
                MagicDamageType::Frost => InvColor::Frost.color(),
                MagicDamageType::Magic => InvColor::Blue.color(),
                MagicDamageType::Poison => InvColor::Poison.color(),
                MagicDamageType::Shock => InvColor::Shock.color(),
                MagicDamageType::Sun => InvColor::Sun.color(),
            },
            SpellVariant::Demoralize => Color::default(),
            SpellVariant::Detect => Color::default(),
            SpellVariant::CarryWeight => Color::default(),
            SpellVariant::Guide => InvColor::Eldritch.color(),
            SpellVariant::Heal => InvColor::Green.color(),
            SpellVariant::Light => InvColor::Eldritch.color(),
            SpellVariant::Reanimate => Color::default(),
            SpellVariant::Reflect => Color::default(),
            SpellVariant::Rune => Color::default(),
            SpellVariant::SoulTrap => InvColor::Eldritch.color(),
            SpellVariant::Summon => Color::default(),
            SpellVariant::Teleport => Color::default(),
            SpellVariant::TurnUndead => InvColor::Sun.color(),
            SpellVariant::Ward => Color::default(),
            _ => Color::default(),
        }
    }

    fn icon_file(&self) -> String {
        match &self.variant {
            SpellVariant::Unknown => self.icon_fallback(),
            SpellVariant::BoundWeapon(w) => match w {
                BoundType::BattleAxe => Icon::WeaponAxeTwoHanded.icon_file(),
                BoundType::Bow => Icon::WeaponBow.icon_file(),
                BoundType::Dagger => Icon::WeaponDagger.icon_file(),
                BoundType::Greatsword => Icon::WeaponSwordOneHanded.icon_file(),
                BoundType::Hammer => Icon::WeaponHammer.icon_file(),
                BoundType::Mace => Icon::WeaponMace.icon_file(),
                BoundType::Shield => Icon::ArmorShieldHeavy.icon_file(),
                BoundType::Sword => Icon::WeaponSwordOneHanded.icon_file(),
                BoundType::WarAxe => Icon::WeaponAxeOneHanded.icon_file(),
                BoundType::Unknown => Icon::WeaponSwordOneHanded.icon_file(),
            },
            SpellVariant::Burden => self.icon_fallback(),
            SpellVariant::Cure => Icon::SpellCure.icon_file(),
            SpellVariant::Damage(t) => match t {
                // These spells have ONLY damage type as their distinguisher.
                MagicDamageType::None => self.icon_fallback(),
                MagicDamageType::Disease => self.icon_fallback(),
                MagicDamageType::Fire => Icon::SpellFire.icon_file(),
                MagicDamageType::Frost => Icon::SpellFrost.icon_file(),
                MagicDamageType::Magic => self.icon_fallback(),
                MagicDamageType::Poison => Icon::SpellPoison.icon_file(),
                MagicDamageType::Shock => Icon::SpellShock.icon_file(),
                MagicDamageType::Sun => Icon::SpellHoly.icon_file(),
            },
            SpellVariant::Banish => self.icon_fallback(),
            SpellVariant::Blizzard => self.icon_fallback(),
            SpellVariant::Calm => self.icon_fallback(),
            SpellVariant::CarryWeight => Icon::SpellFeather.icon_file(),
            SpellVariant::Cloak(_) => Icon::ArmorCloak.icon_file(),
            SpellVariant::Demoralize => Icon::SpellFear.icon_file(),
            SpellVariant::Detect => Icon::SpellDetect.icon_file(),
            SpellVariant::EnhanceWeapon => Icon::SpellSharpen.icon_file(),
            SpellVariant::Fear => Icon::SpellFear.icon_file(),
            SpellVariant::Fireball => Icon::SpellFireball.icon_file(),
            SpellVariant::Firebolt => Icon::SpellFireDual.icon_file(),
            SpellVariant::FireboltStorm => Icon::SpellMeteor.icon_file(),
            SpellVariant::FireWall => Icon::SpellFireWall.icon_file(),
            SpellVariant::Frost => Icon::SpellFrost.icon_file(),
            SpellVariant::FrostWall => Icon::SpellFrostWall.icon_file(), // TODO frostwall
            SpellVariant::Guide => Icon::SpellWisp.icon_file(),
            SpellVariant::Heal => Icon::SpellHeal.icon_file(),
            SpellVariant::IceSpike => Icon::SpellIceShard.icon_file(),
            SpellVariant::IceStorm => self.icon_fallback(),
            SpellVariant::IcySpear => Icon::SpellIceShard.icon_file(),
            SpellVariant::Invisibility => self.icon_fallback(),
            SpellVariant::Light => Icon::SpellLight.icon_file(),
            SpellVariant::LightningBolt => self.icon_fallback(),
            SpellVariant::LightningStorm => Icon::SpellChainLightning.icon_file(),
            SpellVariant::Mayhem => self.icon_fallback(),
            SpellVariant::Pacify => self.icon_fallback(),
            SpellVariant::Paralyze => self.icon_fallback(),
            SpellVariant::Rally => self.icon_fallback(),
            SpellVariant::Reanimate => Icon::SpellReanimate.icon_file(),
            SpellVariant::Reflect => Icon::SpellReflect.icon_file(),
            SpellVariant::Rout => Icon::SpellFear.icon_file(),
            SpellVariant::Rune => Icon::SpellRune.icon_file(),
            SpellVariant::Shock => Icon::SpellShockStrong.icon_file(),
            SpellVariant::SlowTime => Icon::SpellTime.icon_file(),
            SpellVariant::SoulTrap => Icon::SpellSoultrap.icon_file(),
            SpellVariant::Sparks => Icon::SpellShock.icon_file(),
            SpellVariant::StormWall => self.icon_fallback(),
            SpellVariant::Summon => Icon::SpellSummon.icon_file(),
            SpellVariant::Teleport => Icon::SpellTeleport.icon_file(),
            SpellVariant::Thunderbolt => Icon::SpellLightningBlast.icon_file(),
            SpellVariant::TurnUndead => Icon::SpellHoly.icon_file(),
            SpellVariant::Ward => Icon::SpellWard.icon_file(),
        }
    }

    fn icon_fallback(&self) -> String {
        match &self.data.school {
            School::Alteration => Icon::Alteration.icon_file(),
            School::Conjuration => Icon::Conjuration.icon_file(),
            School::Destruction => Icon::Destruction.icon_file(),
            School::Illusion => Icon::Illusion.icon_file(),
            School::Restoration => Icon::Restoration.icon_file(),
            School::None => Icon::IconDefault.icon_file(),
        }
    }
}

#[derive(Debug, Default, Clone, Hash, Display, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum School {
    Alteration = 18,
    Conjuration,
    Destruction,
    Illusion,
    Restoration,
    #[default]
    None,
}

impl From<i32> for School {
    fn from(value: i32) -> Self {
        match value {
            18 => School::Alteration,
            19 => School::Conjuration,
            20 => School::Destruction,
            21 => School::Illusion,
            22 => School::Restoration,
            _ => School::None,
        }
    }
}

#[derive(Debug, Default, Clone, Hash, Display, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum MagicSpellLevel {
    #[default]
    Novice,
    Apprentice,
    Adept,
    Master,
    Expert,
}

#[derive(Clone, Debug, Default, Display, Hash, Eq, PartialEq)]
pub enum MagicDamageType {
    #[default]
    None,
    Disease,
    Fire,
    Frost,
    Magic,
    Poison,
    Shock,
    Sun,
}

impl From<u32> for MagicSpellLevel {
    fn from(skill_level: u32) -> Self {
        if skill_level >= 100 {
            MagicSpellLevel::Master
        } else if skill_level >= 75 {
            MagicSpellLevel::Expert
        } else if skill_level >= 50 {
            MagicSpellLevel::Adept
        } else if skill_level >= 25 {
            MagicSpellLevel::Apprentice
        } else {
            MagicSpellLevel::Novice
        }
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum BoundType {
    BattleAxe,
    Bow,
    Dagger,
    Greatsword,
    Hammer,
    Mace,
    Shield,
    Sword,
    WarAxe,
    #[default]
    Unknown,
}

// Some magic overhauls move spells from one school to another, so this
// classification should be used for all schools even if you reasonably think
// that healing spells will never be destruction spells. Also, this is as
// ad-hoc as the game spell types themselves.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum SpellVariant {
    #[default]
    Unknown,
    Banish,
    Blizzard,
    BoundWeapon(BoundType),
    Burden,
    Calm,                   // effects will include av calm
    CarryWeight,            // feather
    Cloak(MagicDamageType), // might need to be more general than damage? also resists
    // CorrodeArmor, DisintegrateWeapon
    Cure,
    Damage(MagicDamageType),
    Demoralize,
    Detect,
    // Drain,
    EnhanceWeapon,
    Fear,
    Fireball,
    Firebolt,
    FireWall,
    FireboltStorm,
    // Font (Life, Strength, Wisdom)
    Frost,
    FrostWall,
    Guide,
    Heal,
    IceSpike,
    IceStorm,
    IcySpear,
    Invisibility,
    Light,
    LightningBolt,
    LightningStorm,
    Mayhem,
    // Muffle,
    Pacify,
    Paralyze,
    Rally, // CallToArms
    Reanimate,
    Reflect,
    Rout,
    Rune,
    Shock,
    SlowTime,
    Sparks,
    SoulTrap,
    StormWall,
    Summon,
    Teleport,
    Thunderbolt,
    // Transmute,
    TurnUndead,
    Ward,
    // Waterbreathing,
    // Waterwalking,
}
