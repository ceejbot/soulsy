use super::color::InvColor;
use super::keywords::*;
use super::{strings_to_enumset, HasIcon};
use crate::images::Icon;
use crate::plugin::Color;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ShoutType {
    icon: Icon,
    color: InvColor,
    variant: ShoutVariant,
}

impl Default for ShoutType {
    fn default() -> Self {
        Self {
            icon: Icon::Shout,
            color: InvColor::default(),
            variant: ShoutVariant::default(),
        }
    }
}

impl HasIcon for ShoutType {
    fn color(&self) -> Color {
        self.color.color()
    }

    fn icon(&self) -> &Icon {
        &self.icon
    }
}

impl ShoutType {
    pub fn new(tags: Vec<String>) -> Self {
        let keywords = strings_to_enumset::<SpellKeywords>(&tags);

        let variant = if keywords.contains(SpellKeywords::Shout_AnimalAllegiance) {
            ShoutVariant::AnimalAllegiance
        } else if keywords.contains(SpellKeywords::Shout_AuraWhisper) {
            ShoutVariant::AuraWhisper
        } else if keywords.contains(SpellKeywords::Shout_BattleFury) {
            ShoutVariant::BattleFury
        } else if keywords.contains(SpellKeywords::Shout_BecomeEthereal) {
            ShoutVariant::BecomeEthereal
        } else if keywords.contains(SpellKeywords::Shout_BendWill) {
            ShoutVariant::BendWill
        } else if keywords.contains(SpellKeywords::Shout_CallDragon) {
            ShoutVariant::CallDragon
        } else if keywords.contains(SpellKeywords::Shout_CallOfValor) {
            ShoutVariant::CallOfValor
        } else if keywords.contains(SpellKeywords::Shout_ClearSkies) {
            ShoutVariant::ClearSkies
        } else if keywords.contains(SpellKeywords::Shout_Disarm) {
            ShoutVariant::Disarm
        } else if keywords.contains(SpellKeywords::Shout_Dismay) {
            ShoutVariant::Dismay
        } else if keywords.contains(SpellKeywords::Shout_DragonAspect) {
            ShoutVariant::DragonAspect
        } else if keywords.contains(SpellKeywords::Shout_Dragonrend) {
            ShoutVariant::Dragonrend
        } else if keywords.contains(SpellKeywords::Shout_DrainVitality) {
            ShoutVariant::DrainVitality
        } else if keywords.contains(SpellKeywords::Shout_ElementalFury) {
            ShoutVariant::ElementalFury
        } else if keywords.contains(SpellKeywords::Shout_FireBreath) {
            ShoutVariant::FireBreath
        } else if keywords.contains(SpellKeywords::Shout_FrostBreath) {
            ShoutVariant::FrostBreath
        } else if keywords.contains(SpellKeywords::Shout_IceForm) {
            ShoutVariant::IceForm
        } else if keywords.contains(SpellKeywords::Shout_KynesPeace) {
            ShoutVariant::KynesPeace
        } else if keywords.contains(SpellKeywords::Shout_MarkedForDeath) {
            ShoutVariant::MarkedForDeath
        } else if keywords.contains(SpellKeywords::Shout_Slowtime) {
            ShoutVariant::Slowtime
        } else if keywords.contains(SpellKeywords::Shout_SoulTear) {
            ShoutVariant::SoulTear
        } else if keywords.contains(SpellKeywords::Shout_StormCall) {
            ShoutVariant::StormCall
        } else if keywords.contains(SpellKeywords::Shout_SummonDurnehviir) {
            ShoutVariant::SummonDurnehviir
        } else if keywords.contains(SpellKeywords::Shout_ThrowVoice) {
            ShoutVariant::ThrowVoice
        } else if keywords.contains(SpellKeywords::Shout_UnrelentingForce) {
            ShoutVariant::UnrelentingForce
        } else if keywords.contains(SpellKeywords::Shout_WhirlwindSprint) {
            ShoutVariant::WhirlwindSprint
        } else {
            ShoutVariant::Unclassified
        };

        let color = match variant {
            ShoutVariant::AnimalAllegiance => InvColor::Green,
            ShoutVariant::AuraWhisper => InvColor::Eldritch,
            ShoutVariant::BecomeEthereal => InvColor::Eldritch,
            ShoutVariant::ClearSkies => InvColor::Blue,
            ShoutVariant::Cyclone => InvColor::Gray,
            ShoutVariant::FireBreath => InvColor::Fire,
            ShoutVariant::FrostBreath => InvColor::Frost,
            ShoutVariant::IceForm => InvColor::Frost,
            ShoutVariant::KynesPeace => InvColor::Green,
            ShoutVariant::MarkedForDeath => InvColor::Poison,
            ShoutVariant::StormCall => InvColor::Shock,
            _ => InvColor::White,
        };

        let icon = match variant {
            ShoutVariant::AnimalAllegiance => Icon::ShoutAnimalAllegiance,
            ShoutVariant::CallDragon => Icon::ShoutCallDragon,
            ShoutVariant::ClearSkies => Icon::ShoutClearSkies,
            ShoutVariant::Cyclone => Icon::ShoutCyclone,
            ShoutVariant::Dismay => Icon::ShoutDismay,
            // ShoutVariant::Dragonrend => Icon::ShoutDragonrend,
            ShoutVariant::ElementalFury => Icon::ShoutElementalFury,
            ShoutVariant::FireBreath => Icon::ShoutBreathAttack,
            ShoutVariant::FrostBreath => Icon::ShoutBreathAttack,
            ShoutVariant::IceForm => Icon::ShoutIceForm,
            ShoutVariant::MarkedForDeath => Icon::ShoutMarkedForDeath,
            ShoutVariant::Slowtime => Icon::SpellTime,
            ShoutVariant::StormCall => Icon::ShoutStormcall,
            ShoutVariant::UnrelentingForce => Icon::ShoutUnrelentingForce,
            ShoutVariant::WhirlwindSprint => Icon::SpellSprint,
            _ => Icon::Shout,
        };

        Self {
            icon,
            color,
            variant,
        }
    }

    pub fn variant(&self) -> &ShoutVariant {
        &self.variant
    }

    pub fn translation(&self) -> &str {
        match self.variant {
            ShoutVariant::AnimalAllegiance => "Raan-mir-tah!",
            ShoutVariant::AuraWhisper => "Laas-ya-nir.",
            ShoutVariant::BattleFury => "Mid-vur-shaan!",
            ShoutVariant::BecomeEthereal => "Feim-zii-gron!",
            ShoutVariant::BendWill => "Gol-hah-dov!",
            ShoutVariant::CallDragon => "Oh-ah-viing!",
            ShoutVariant::CallOfValor => "Hun-kaal-zoor!",
            ShoutVariant::ClearSkies => "Lok-vah-koor!",
            ShoutVariant::Cyclone => "Ven-gaar-nos!",
            ShoutVariant::Disarm => "Zun-haal-viik!",
            ShoutVariant::Dismay => "Faas-ru-maar!",
            ShoutVariant::DragonAspect => "Mul-qah-diiv!",
            ShoutVariant::Dragonrend => "Joor-zah-frul!",
            ShoutVariant::DrainVitality => "Gaan-lah-haas!",
            ShoutVariant::ElementalFury => "Su-grah-dun!",
            ShoutVariant::FireBreath => "Yol-toor-shul!",
            ShoutVariant::FrostBreath => "Fo-krah-diin!!",
            ShoutVariant::IceForm => "Iiz-slen-nus!",
            ShoutVariant::KynesPeace => "Kaan-drem-ov!",
            ShoutVariant::MarkedForDeath => "Krii-lun-aus!",
            ShoutVariant::Slowtime => "Tiid-klo-ui!",
            ShoutVariant::SoulTear => "Rii-vaaz-zol!",
            ShoutVariant::StormCall => "Strun-bah-qo!",
            ShoutVariant::SummonDurnehviir => "Dur-neh-viir!",
            ShoutVariant::ThrowVoice => "Zul-mey-gut!",
            ShoutVariant::UnrelentingForce => "Fus-ro-dah!",
            ShoutVariant::WhirlwindSprint => "Wuld-nah-kest!!",
            ShoutVariant::Unclassified => "This shout is new to me!",
        }
    }
}

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
