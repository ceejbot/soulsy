use super::color::InvColor;
use super::icons::Icon;
use super::keywords::*;
use super::HasIcon;
use crate::plugin::Color;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum ShoutVariant {
    AnimalAllegiance,
    AuraWhisper,
    BattleFury,
    BecomeEthereal,
    BendWill,
    CallDragon,
    CallOfValor,
    ClearSkies,
    Cyclone,
    Disarm,
    Dismay,
    DragonAspect,
    Dragonrend,
    DrainVitality,
    ElementalFury,
    FireBreath,
    FrostBreath,
    IceForm,
    KynesPeace,
    MarkedForDeath,
    Slowtime,
    SoulTear,
    StormCall,
    SummonDurnehviir,
    ThrowVoice,
    UnrelentingForce,
    WhirlwindSprint,
    #[default]
    Unclassified,
}

impl ShoutVariant {
    pub fn new(tags: Vec<String>) -> Self {
        let keywords = strings_to_keywords(&tags);

        if keywords.contains(&SpellEffectKeywords::Shout_AnimalAllegiance) {
            ShoutVariant::AnimalAllegiance
        } else if keywords.contains(&SpellEffectKeywords::Shout_AuraWhisper) {
            ShoutVariant::AuraWhisper
        } else if keywords.contains(&SpellEffectKeywords::Shout_BattleFury) {
            ShoutVariant::BattleFury
        } else if keywords.contains(&SpellEffectKeywords::Shout_BecomeEthereal) {
            ShoutVariant::BecomeEthereal
        } else if keywords.contains(&SpellEffectKeywords::Shout_BendWill) {
            ShoutVariant::BendWill
        } else if keywords.contains(&SpellEffectKeywords::Shout_CallDragon) {
            ShoutVariant::CallDragon
        } else if keywords.contains(&SpellEffectKeywords::Shout_CallOfValor) {
            ShoutVariant::CallOfValor
        } else if keywords.contains(&SpellEffectKeywords::Shout_ClearSkies) {
            ShoutVariant::ClearSkies
        } else if keywords.contains(&SpellEffectKeywords::Shout_Disarm) {
            ShoutVariant::Disarm
        } else if keywords.contains(&SpellEffectKeywords::Shout_Dismay) {
            ShoutVariant::Dismay
        } else if keywords.contains(&SpellEffectKeywords::Shout_DragonAspect) {
            ShoutVariant::DragonAspect
        } else if keywords.contains(&SpellEffectKeywords::Shout_Dragonrend) {
            ShoutVariant::Dragonrend
        } else if keywords.contains(&SpellEffectKeywords::Shout_DrainVitality) {
            ShoutVariant::DrainVitality
        } else if keywords.contains(&SpellEffectKeywords::Shout_ElementalFury) {
            ShoutVariant::ElementalFury
        } else if keywords.contains(&SpellEffectKeywords::Shout_FireBreath) {
            ShoutVariant::FireBreath
        } else if keywords.contains(&SpellEffectKeywords::Shout_FrostBreath) {
            ShoutVariant::FrostBreath
        } else if keywords.contains(&SpellEffectKeywords::Shout_IceForm) {
            ShoutVariant::IceForm
        } else if keywords.contains(&SpellEffectKeywords::Shout_KynesPeace) {
            ShoutVariant::KynesPeace
        } else if keywords.contains(&SpellEffectKeywords::Shout_MarkedForDeath) {
            ShoutVariant::MarkedForDeath
        } else if keywords.contains(&SpellEffectKeywords::Shout_Slowtime) {
            ShoutVariant::Slowtime
        } else if keywords.contains(&SpellEffectKeywords::Shout_SoulTear) {
            ShoutVariant::SoulTear
        } else if keywords.contains(&SpellEffectKeywords::Shout_StormCall) {
            ShoutVariant::StormCall
        } else if keywords.contains(&SpellEffectKeywords::Shout_SummonDurnehviir) {
            ShoutVariant::SummonDurnehviir
        } else if keywords.contains(&SpellEffectKeywords::Shout_ThrowVoice) {
            ShoutVariant::ThrowVoice
        } else if keywords.contains(&SpellEffectKeywords::Shout_UnrelentingForce) {
            ShoutVariant::UnrelentingForce
        } else if keywords.contains(&SpellEffectKeywords::Shout_WhirlwindSprint) {
            ShoutVariant::WhirlwindSprint
        } else {
            ShoutVariant::Unclassified
        }
    }
}

impl HasIcon for ShoutVariant {
    fn color(&self) -> Color {
        match &self {
            ShoutVariant::AnimalAllegiance => InvColor::Green.color(),
            ShoutVariant::AuraWhisper => InvColor::Eldritch.color(),
            ShoutVariant::BattleFury => InvColor::White.color(),
            ShoutVariant::BecomeEthereal => InvColor::Eldritch.color(),
            ShoutVariant::BendWill => InvColor::White.color(),
            ShoutVariant::CallDragon => InvColor::White.color(),
            ShoutVariant::CallOfValor => InvColor::White.color(),
            ShoutVariant::ClearSkies => InvColor::Blue.color(),
            ShoutVariant::Cyclone => InvColor::Gray.color(),
            ShoutVariant::Disarm => InvColor::White.color(),
            ShoutVariant::Dismay => InvColor::White.color(),
            ShoutVariant::DragonAspect => InvColor::White.color(),
            ShoutVariant::Dragonrend => InvColor::White.color(),
            ShoutVariant::DrainVitality => InvColor::White.color(),
            ShoutVariant::ElementalFury => InvColor::White.color(),
            ShoutVariant::FireBreath => InvColor::Fire.color(),
            ShoutVariant::FrostBreath => InvColor::Frost.color(),
            ShoutVariant::IceForm => InvColor::Frost.color(),
            ShoutVariant::KynesPeace => InvColor::Green.color(),
            ShoutVariant::MarkedForDeath => InvColor::Poison.color(),
            ShoutVariant::Slowtime => InvColor::White.color(),
            ShoutVariant::SoulTear => InvColor::White.color(),
            ShoutVariant::StormCall => InvColor::Shock.color(),
            ShoutVariant::ThrowVoice => InvColor::White.color(),
            ShoutVariant::Unclassified => InvColor::White.color(),
            ShoutVariant::UnrelentingForce => InvColor::White.color(),
            ShoutVariant::WhirlwindSprint => InvColor::White.color(),
            ShoutVariant::SummonDurnehviir => InvColor::White.color(),
        }
    }

    fn icon_file(&self) -> String {
        match self {
            ShoutVariant::AnimalAllegiance => Icon::SpellBear.icon_file(),
            ShoutVariant::AuraWhisper => Icon::Shout.icon_file(),
            ShoutVariant::BattleFury => Icon::Shout.icon_file(),
            ShoutVariant::BecomeEthereal => Icon::Shout.icon_file(),
            ShoutVariant::BendWill => Icon::Shout.icon_file(),
            ShoutVariant::CallDragon => Icon::Shout.icon_file(),
            ShoutVariant::CallOfValor => Icon::Shout.icon_file(),
            ShoutVariant::ClearSkies => Icon::SpellSun.icon_file(),
            ShoutVariant::Cyclone => Icon::SpellTornado.icon_file(),
            ShoutVariant::Disarm => Icon::Shout.icon_file(),
            ShoutVariant::Dismay => Icon::SpellFear.icon_file(),
            ShoutVariant::DragonAspect => Icon::Shout.icon_file(),
            ShoutVariant::Dragonrend => Icon::SpellBleed.icon_file(),
            ShoutVariant::DrainVitality => Icon::Shout.icon_file(),
            ShoutVariant::ElementalFury => Icon::SpellElementalFury.icon_file(),
            ShoutVariant::FireBreath => Icon::SpellBreathAttack.icon_file(),
            ShoutVariant::FrostBreath => Icon::SpellBreathAttack.icon_file(),
            ShoutVariant::IceForm => Icon::SpellFreeze.icon_file(),
            ShoutVariant::KynesPeace => Icon::Shout.icon_file(),
            ShoutVariant::MarkedForDeath => Icon::SpellDeath.icon_file(),
            ShoutVariant::Slowtime => Icon::SpellTime.icon_file(),
            ShoutVariant::SoulTear => Icon::Shout.icon_file(),
            ShoutVariant::StormCall => Icon::SpellStormblast.icon_file(),
            ShoutVariant::SummonDurnehviir => Icon::Shout.icon_file(),
            ShoutVariant::ThrowVoice => Icon::Shout.icon_file(),
            ShoutVariant::UnrelentingForce => Icon::Shout.icon_file(),
            ShoutVariant::WhirlwindSprint => Icon::SpellSprint.icon_file(),
            ShoutVariant::Unclassified => Icon::Shout.icon_file(),
        }
    }

    fn icon_fallback(&self) -> String {
        Icon::Shout.icon_file()
    }
}
