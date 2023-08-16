use super::color::InvColor;
use super::game_enums::{ActorValue, SpellArchetype};
use super::icons::Icon;
use super::spell::SpellData;
use super::HasIcon;
use crate::plugin::Color;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum ShoutVariant {
    AnimalAllegiance,
    AuraWhisper,
    CallDragon,
    CallOfValor,
    ClearSkies,
    Disarm,
    Dismay,
    DragonRend,
    ElementalFury,
    Ethereal,
    FireBall,
    FireBreath,
    FrostBreath,
    IceForm,
    IceStorm,
    KynesPeace,
    MarkedForDeath,
    Slowtime,
    StormCall,
    ThrowVoice,
    #[default]
    UnrelentingForce,
    WhirlwindSprint,
}

impl ShoutVariant {
    pub fn from_spell_data(data: SpellData) -> Self {
        if matches!(data.archetype, SpellArchetype::SlowTime) {
            Self::Slowtime
        } else if matches!(data.archetype, SpellArchetype::PeakValueModifier)
            && matches!(data.effect, ActorValue::Health)
        {
            Self::MarkedForDeath
        } else if matches!(data.effect, ActorValue::DragonRend) {
            Self::DragonRend
        } else {
            log::debug!("default shout; spelldata={data:?}");
            Self::UnrelentingForce
        }
    }
}

impl HasIcon for ShoutVariant {
    fn color(&self) -> Color {
        match &self {
            ShoutVariant::AnimalAllegiance => InvColor::White.color(),
            ShoutVariant::AuraWhisper => InvColor::White.color(),
            ShoutVariant::CallDragon => InvColor::White.color(),
            ShoutVariant::CallOfValor => InvColor::White.color(),
            ShoutVariant::ClearSkies => InvColor::White.color(),
            ShoutVariant::Disarm => InvColor::White.color(),
            ShoutVariant::Dismay => InvColor::White.color(),
            ShoutVariant::DragonRend => InvColor::White.color(),
            ShoutVariant::ElementalFury => InvColor::White.color(),
            ShoutVariant::Ethereal => InvColor::White.color(),
            ShoutVariant::FireBall => InvColor::Fire.color(),
            ShoutVariant::FireBreath => InvColor::Fire.color(),
            ShoutVariant::FrostBreath => InvColor::Frost.color(),
            ShoutVariant::IceForm => InvColor::Frost.color(),
            ShoutVariant::IceStorm => InvColor::Frost.color(),
            ShoutVariant::KynesPeace => InvColor::White.color(),
            ShoutVariant::MarkedForDeath => InvColor::White.color(),
            ShoutVariant::Slowtime => InvColor::White.color(),
            ShoutVariant::StormCall => InvColor::White.color(),
            ShoutVariant::ThrowVoice => InvColor::White.color(),
            ShoutVariant::UnrelentingForce => InvColor::White.color(),
            ShoutVariant::WhirlwindSprint => InvColor::White.color(),
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
            ShoutVariant::MarkedForDeath => Icon::Shout.icon_file(),
            ShoutVariant::Slowtime => Icon::SpellTime.icon_file(),
            ShoutVariant::StormCall => Icon::Shout.icon_file(),
            ShoutVariant::ThrowVoice => Icon::Shout.icon_file(),
            ShoutVariant::UnrelentingForce => Icon::Shout.icon_file(),
            ShoutVariant::WhirlwindSprint => Icon::Shout.icon_file(),
        }
    }

    fn icon_fallback(&self) -> String {
        Icon::Shout.icon_file()
    }
}
