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
            ShoutVariant::AuraWhisper => Icon::ShoutAuraWhisper,
            ShoutVariant::BattleFury => Icon::ShoutBattleFury,
            ShoutVariant::BecomeEthereal => Icon::Shout,
            ShoutVariant::BendWill => Icon::ShoutBendWill,
            ShoutVariant::CallDragon => Icon::ShoutCallDragon,
            ShoutVariant::CallOfValor => Icon::ShoutCallOfValor,
            ShoutVariant::ClearSkies => Icon::ShoutClearSkies,
            ShoutVariant::Cyclone => Icon::ShoutCyclone,
            ShoutVariant::Disarm => Icon::ShoutDisarm,
            ShoutVariant::Dismay => Icon::ShoutDismay,
            ShoutVariant::DragonAspect => Icon::ShoutDragonAspect,
            ShoutVariant::Dragonrend => Icon::ShoutDragonrend,
            ShoutVariant::DrainVitality => Icon::ShoutDrainVitality,
            ShoutVariant::ElementalFury => Icon::ShoutElementalFury,
            ShoutVariant::FireBreath => Icon::ShoutBreathAttack,
            ShoutVariant::FrostBreath => Icon::ShoutBreathAttack,
            ShoutVariant::IceForm => Icon::ShoutIceForm,
            ShoutVariant::KynesPeace => Icon::ShoutKynesPeace,
            ShoutVariant::MarkedForDeath => Icon::ShoutMarkedForDeath,
            ShoutVariant::PhantomForm => Icon::ShoutPhantomForm,
            ShoutVariant::Slowtime => Icon::ShoutSlowtime,
            ShoutVariant::SoulTear => Icon::ShoutSoulTear,
            ShoutVariant::StormCall => Icon::ShoutStormcall,
            ShoutVariant::SummonDurnehviir => Icon::ShoutSummonDurnehviir,
            ShoutVariant::ThrowVoice => Icon::ShoutThrowVoice,
            ShoutVariant::UnrelentingForce => Icon::ShoutUnrelentingForce,
            ShoutVariant::WhirlwindSprint => Icon::ShoutWhirlwindSprint,
            // unused dawnguard shout
            ShoutVariant::SummonUndead => Icon::Shout,
            // stormcrown
            ShoutVariant::LightningBreath => Icon::ShoutLightningBreath,
            ShoutVariant::PoisonBreath => Icon::ShoutPoisonBreath,

            // thunderchild shouts
            ShoutVariant::AlessiasLove => Icon::ShoutAlessiasLove,
            ShoutVariant::Annihilate => Icon::ShoutAnnihilate,
            ShoutVariant::ArcaneHelix => Icon::ShoutArcaneHelix,
            ShoutVariant::Armageddon => Icon::ShoutArmageddon,
            ShoutVariant::Curse => Icon::ShoutCurse,
            ShoutVariant::DanceOfTheDead => Icon::ShoutDanceOfTheDead,
            ShoutVariant::Earthquake => Icon::ShoutEarthquake,
            ShoutVariant::EssenceRip => Icon::ShoutEssenceRip,
            ShoutVariant::Evocation => Icon::ShoutEvocation,
            ShoutVariant::Geomagnetism => Icon::ShoutGeomagnetism,
            ShoutVariant::Iceborn => Icon::ShoutIceborn,
            ShoutVariant::JonesShadow => Icon::ShoutJonesShadow,
            ShoutVariant::Kingsbane => Icon::ShoutKingsbane,
            ShoutVariant::Lifestream => Icon::ShoutLifestream,
            ShoutVariant::LightningShield => Icon::ShoutLightningShield,
            ShoutVariant::Oblivion => Icon::ShoutOblivion,
            ShoutVariant::PhantomDecoy => Icon::ShoutPhantomDecoy,
            ShoutVariant::Riftwalk => Icon::ShoutRiftwalk,
            ShoutVariant::Shattersphere => Icon::ShoutShattersphere,
            ShoutVariant::ShorsWrath => Icon::ShoutShorsWrath,
            ShoutVariant::ShroudOfSnowfall => Icon::ShoutShroudOfSnowfall,
            ShoutVariant::SpeakUntoTheStars => Icon::ShoutSpeakUntoTheStars,
            ShoutVariant::SplinterTwins => Icon::ShoutSplinterTwins,
            ShoutVariant::Stormblast => Icon::ShoutStormblast,
            ShoutVariant::TheConqueror => Icon::ShoutTheConqueror,
            ShoutVariant::Trueshot => Icon::ShoutTrueshot,
            ShoutVariant::WailOfTheBanshee => Icon::ShoutWailOfTheBanshee,
            ShoutVariant::Wanderlust => Icon::ShoutWanderlust,
            ShoutVariant::Warcry => Icon::ShoutWarcry,
            ShoutVariant::Unclassified => Icon::Shout,
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
            ShoutVariant::PhantomForm => "Fiik-lo-sah!",
            ShoutVariant::Slowtime => "Tiid-klo-ui!",
            ShoutVariant::SoulTear => "Rii-vaaz-zol!",
            ShoutVariant::StormCall => "Strun-bah-qo!",
            ShoutVariant::SummonDurnehviir => "Dur-neh-viir!",
            ShoutVariant::ThrowVoice => "Zul-mey-gut!",
            ShoutVariant::UnrelentingForce => "Fus-ro-dah!",
            ShoutVariant::WhirlwindSprint => "Wuld-nah-kest!",
            ShoutVariant::SummonUndead => "Diil-qoth-zaam!",
            // stormcrown
            ShoutVariant::LightningBreath => "Strun-gaar-kest",
            ShoutVariant::PoisonBreath => "Laas-slen-aus",
            // thunderchild
            ShoutVariant::AlessiasLove => "Juoor-drem-ov",
            ShoutVariant::Annihilate => "Fii-gaar-nos",
            ShoutVariant::ArcaneHelix => "Vol-nah-kest",
            ShoutVariant::Armageddon => "Wuld-toor-shul",
            ShoutVariant::Curse => "Fiik-zii-gron!",
            ShoutVariant::DanceOfTheDead => "Raan-vaaz-sol",
            ShoutVariant::Earthquake => "Fus-klo-ul",
            ShoutVariant::EssenceRip => "Laaz-ro-dah",
            ShoutVariant::Evocation => "Ven-lah-haas",
            ShoutVariant::Geomagnetism => "Gol-yah-nir",
            ShoutVariant::Iceborn => "Iiz-ah-viing",
            ShoutVariant::JonesShadow => "Zul-lun-aus",
            ShoutVariant::Kingsbane => "Mul-neh-viir",
            ShoutVariant::Lifestream => "Gaan-vur-shaan",
            ShoutVariant::LightningShield => "Strun-slen-nus",
            ShoutVariant::Oblivion => "Dur-hah-dov",
            ShoutVariant::PhantomDecoy => "Fiik-lo-sah",
            ShoutVariant::Riftwalk => "Su-ru-maar",
            ShoutVariant::Shattersphere => "Fo-mey-gut",
            ShoutVariant::ShorsWrath => "Hun-haal-viik",
            ShoutVariant::ShroudOfSnowfall => "Feim-krah-diin",
            ShoutVariant::SpeakUntoTheStars => "Tiid-mir-tah",
            ShoutVariant::SplinterTwins => "Frii-lo-sah",
            ShoutVariant::Stormblast => "Lok-bah-qo",
            ShoutVariant::TheConqueror => "Mid-quah-diiv",
            ShoutVariant::Trueshot => "Kaan-grah-dun",
            ShoutVariant::WailOfTheBanshee => "Faaz-zah-frul",
            ShoutVariant::Wanderlust => "Od-vah-koor",
            ShoutVariant::Warcry => "Zun-kaal-zoor",
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
    LightningBreath,
    MarkedForDeath,
    PhantomForm,
    PoisonBreath,
    Slowtime,
    SoulTear,
    StormCall,
    SummonDurnehviir,
    ThrowVoice,
    UnrelentingForce,
    WhirlwindSprint,
    // unused dawnguard shout
    SummonUndead,
    // Thunderchild shouts
    AlessiasLove,
    Annihilate,
    ArcaneHelix,
    Armageddon,
    Curse,
    DanceOfTheDead,
    Earthquake,
    EssenceRip,
    Evocation,
    Geomagnetism,
    Iceborn,
    JonesShadow,
    Kingsbane,
    Lifestream,
    LightningShield,
    Oblivion,
    PhantomDecoy,
    Riftwalk,
    Shattersphere,
    ShorsWrath,
    ShroudOfSnowfall,
    SpeakUntoTheStars,
    SplinterTwins,
    Stormblast,
    TheConqueror,
    Trueshot,
    WailOfTheBanshee,
    Wanderlust,
    Warcry,
    #[default]
    Unclassified,
}
