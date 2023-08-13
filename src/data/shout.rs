use super::color::InvColor;
use super::game_enums::{ActorValue, SpellArchetype};
use super::HasIcon;
use super::icons::Icon;
use crate::plugin::Color;


#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum ShoutVariants {
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
    UnrelentingForce,
    WhirlwindSprint,
}
