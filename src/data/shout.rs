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
        } else if matches!(data.archetype, SpellArchetype::PeakValueModifier) && matches!(data.effect, ActorValue::Health) {
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
            ShoutVariant::AnimalAllegiance => InvColor::OCF_InvColorWhite.color(),
            ShoutVariant::AuraWhisper => InvColor::OCF_InvColorWhite.color(),
            ShoutVariant::CallDragon => InvColor::OCF_InvColorWhite.color(),
            ShoutVariant::CallOfValor => InvColor::OCF_InvColorWhite.color(),
            ShoutVariant::ClearSkies => InvColor::OCF_InvColorWhite.color(),
            ShoutVariant::Disarm => InvColor::OCF_InvColorWhite.color(),
            ShoutVariant::Dismay => InvColor::OCF_InvColorWhite.color(),
            ShoutVariant::DragonRend => InvColor::OCF_InvColorWhite.color(),
            ShoutVariant::ElementalFury => InvColor::OCF_InvColorWhite.color(),
            ShoutVariant::Ethereal => InvColor::OCF_InvColorWhite.color(),
            ShoutVariant::FireBall => InvColor::OCF_InvColorFire.color(),
            ShoutVariant::FireBreath => InvColor::OCF_InvColorFire.color(),
            ShoutVariant::FrostBreath => InvColor::OCF_InvColorFrost.color(),
            ShoutVariant::IceForm => InvColor::OCF_InvColorFrost.color(),
            ShoutVariant::IceStorm => InvColor::OCF_InvColorFrost.color(),
            ShoutVariant::KynesPeace => InvColor::OCF_InvColorWhite.color(),
            ShoutVariant::MarkedForDeath => InvColor::OCF_InvColorWhite.color(),
            ShoutVariant::Slowtime => InvColor::OCF_InvColorWhite.color(),
            ShoutVariant::StormCall => InvColor::OCF_InvColorWhite.color(),
            ShoutVariant::ThrowVoice => InvColor::OCF_InvColorWhite.color(),
            ShoutVariant::UnrelentingForce => InvColor::OCF_InvColorWhite.color(),
            ShoutVariant::WhirlwindSprint => InvColor::OCF_InvColorWhite.color(),
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
