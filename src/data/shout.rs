use super::color::InvColor;
use super::game_enums::{ActorValue, SpellArchetype};
use super::icons::Icon;
use super::spell::SpellData;
use super::spell::{MagicDamageType, School};
use super::HasIcon;
use crate::plugin::Color;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum ShoutVariant {
    AnimalAllegiance, // script, no AV, no resist / formids: 930c8, 9e0ca, 9e0cb
    AuraWhisper,      // script, no AV, 10e4fd
    CallDragon,       // acript
    CallOfValor,      // summon creature, assoc item is a summon 51963, 78B9d, 78b9f
    ClearSkies,       // script 789d, 789f, or 78ba1
    Disarm,           // 8bb26, cd088, cd089   archetype is disam
    Dismay,           // archetype demoralize, actor value confidence
    DragonRend,       // archetype is stagger
    ElementalFury,    // av is weapon speed mult,  arch is enhance weapon
    Ethereal,
    FireBall,    // destruction, resist is fire, av is health, archetype is value modifier
    FireBreath,  // resist is fire, value modifier, health
    FrostBreath, // arch dual value modifier, av health, resist is frost
    IceForm,     // hrmph
    IceStorm,    // dual value modifier, health/stam, resist is frost
    KynesPeace,
    MarkedForDeath, // archetype is value modifier, damage resist OR peak value, health
    Slowtime,
    StormCall,        //  form pairs are a1a58, e3f0a / a1a5c, e3f09 / a1a5b, d5e81
    ThrowVoice,       // spawn scripted ref, 7430d is only effect
    UnrelentingForce, // arch is stagger, forms are 13e08, 7f82e, 7f82f
    WhirlwindSprint,  // arch is script / 2f789, 4372f, 43730
    #[default]
    Unclassified,
}

impl ShoutVariant {
    pub fn from_spell_data(data: SpellData, form_string: String) -> Self {
        if matches!(data.archetype, SpellArchetype::SummonCreature) {
            Self::CallOfValor
        } else if matches!(data.archetype, SpellArchetype::Disarm) {
            Self::Disarm
        } else if matches!(data.archetype, SpellArchetype::Demoralize) {
            Self::Dismay
        } else if matches!(data.effect, ActorValue::DragonRend) {
            // is this even true? archetype is stagger?
            Self::DragonRend
        } else if matches!(data.archetype, SpellArchetype::EnhanceWeapon) {
            Self::ElementalFury
        } else if matches!(data.school, School::Destruction)
            && matches!(data.damage, MagicDamageType::Fire)
        {
            Self::FireBall
        } else if matches!(data.damage, MagicDamageType::Fire) {
            Self::FireBreath
        } else if matches!(data.damage, MagicDamageType::Frost) {
            Self::FrostBreath
        } else if matches!(data.archetype, SpellArchetype::Calm) {
            Self::KynesPeace
        } else if (matches!(data.archetype, SpellArchetype::PeakValueModifier)
            && matches!(data.effect, ActorValue::Health))
            || (matches!(data.archetype, SpellArchetype::ValueModifier)
                && matches!(data.effect, ActorValue::DamageResist))
        {
            Self::MarkedForDeath
        } else if matches!(data.archetype, SpellArchetype::SlowTime) {
            Self::Slowtime
        } else if form_string == "Skyrim.esm|0x0007430d" {
            Self::ThrowVoice
        } else if form_string.as_str() == "Skyrim.esp|0x0002f789" {
            Self::WhirlwindSprint
        } else {
            log::debug!("default shout; form_string={form_string}; spelldata={data:?}");
            Self::Unclassified
        }
    }
}

impl HasIcon for ShoutVariant {
    fn color(&self) -> Color {
        match &self {
            ShoutVariant::AnimalAllegiance => InvColor::Green.color(),
            ShoutVariant::AuraWhisper => InvColor::White.color(),
            ShoutVariant::CallDragon => InvColor::White.color(),
            ShoutVariant::CallOfValor => InvColor::White.color(),
            ShoutVariant::ClearSkies => InvColor::White.color(),
            ShoutVariant::Disarm => InvColor::White.color(),
            ShoutVariant::Dismay => InvColor::White.color(),
            ShoutVariant::DragonRend => InvColor::White.color(),
            ShoutVariant::ElementalFury => InvColor::White.color(),
            ShoutVariant::Ethereal => InvColor::Eldritch.color(),
            ShoutVariant::FireBall => InvColor::Fire.color(),
            ShoutVariant::FireBreath => InvColor::Fire.color(),
            ShoutVariant::FrostBreath => InvColor::Frost.color(),
            ShoutVariant::IceForm => InvColor::Frost.color(),
            ShoutVariant::IceStorm => InvColor::Frost.color(),
            ShoutVariant::KynesPeace => InvColor::Green.color(),
            ShoutVariant::MarkedForDeath => InvColor::Poison.color(),
            ShoutVariant::Slowtime => InvColor::White.color(),
            ShoutVariant::StormCall => InvColor::Shock.color(),
            ShoutVariant::ThrowVoice => InvColor::White.color(),
            ShoutVariant::UnrelentingForce => InvColor::White.color(),
            ShoutVariant::WhirlwindSprint => InvColor::White.color(),
            ShoutVariant::Unclassified => InvColor::White.color(),
        }
    }

    fn icon_file(&self) -> String {
        match self {
            ShoutVariant::AnimalAllegiance => Icon::Shout.icon_file(),
            ShoutVariant::AuraWhisper => Icon::Shout.icon_file(),
            ShoutVariant::CallDragon => Icon::Shout.icon_file(),
            ShoutVariant::CallOfValor => Icon::Shout.icon_file(),
            ShoutVariant::ClearSkies => Icon::Shout.icon_file(),
            ShoutVariant::Disarm => Icon::Shout.icon_file(),
            ShoutVariant::Dismay => Icon::Shout.icon_file(),
            ShoutVariant::DragonRend => Icon::Shout.icon_file(),
            ShoutVariant::ElementalFury => Icon::Shout.icon_file(),
            ShoutVariant::Ethereal => Icon::Shout.icon_file(),
            ShoutVariant::FireBall => Icon::SpellFireball.icon_file(),
            ShoutVariant::FireBreath => Icon::SpellBreathAttack.icon_file(),
            ShoutVariant::FrostBreath => Icon::SpellBreathAttack.icon_file(),
            ShoutVariant::IceForm => Icon::Shout.icon_file(),
            ShoutVariant::IceStorm => Icon::Shout.icon_file(),
            ShoutVariant::KynesPeace => Icon::Shout.icon_file(),
            ShoutVariant::MarkedForDeath => Icon::SpellDeath.icon_file(),
            ShoutVariant::Slowtime => Icon::SpellTime.icon_file(),
            ShoutVariant::StormCall => Icon::Shout.icon_file(),
            ShoutVariant::ThrowVoice => Icon::Shout.icon_file(),
            ShoutVariant::UnrelentingForce => Icon::Shout.icon_file(),
            ShoutVariant::WhirlwindSprint => Icon::Shout.icon_file(),
            ShoutVariant::Unclassified => Icon::Shout.icon_file(),
        }
    }

    fn icon_fallback(&self) -> String {
        Icon::Shout.icon_file()
    }
}
