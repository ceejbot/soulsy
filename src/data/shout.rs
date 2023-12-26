use std::collections::HashMap;

use once_cell::sync::Lazy;

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
        let (variant, icon) = SHOUT_MAPPING
            .iter()
            .find_map(|(k, v)| {
                if keywords.contains(*k) {
                    Some(v.clone())
                } else {
                    None
                }
            })
            .unwrap_or((ShoutVariant::Unclassified, Icon::Shout));

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
            ShoutVariant::Stormcall => InvColor::Shock,
            _ => InvColor::White,
        };

        Self {
            icon,
            color,
            variant,
        }
    }

    pub fn construct(icon: Icon, color: InvColor, variant: ShoutVariant) -> Self {
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
            ShoutVariant::Stormcall => "Strun-bah-qo!",
            ShoutVariant::SummonDurnehviir => "Dur-neh-viir!",
            ShoutVariant::ThrowVoice => "Zul-mey-gut!",
            ShoutVariant::UnrelentingForce => "Fus-ro-dah!",
            ShoutVariant::WhirlwindSprint => "Wuld-nah-kest!",
            ShoutVariant::SoulCairnSummon => "Diil-qoth-zaam!",
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
    MarkedForDeath,
    PhantomForm,
    Slowtime,
    SoulTear,
    Stormcall,
    SummonDurnehviir,
    ThrowVoice,
    UnrelentingForce,
    WhirlwindSprint,
    // unused dawnguard shout
    SoulCairnSummon,
    // Stormcrown
    LightningBreath,
    PoisonBreath,
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

static SHOUT_MAPPING: Lazy<HashMap<SpellKeywords, (ShoutVariant, Icon)>> = Lazy::new(|| {
    HashMap::from([
        (
            SpellKeywords::Shout_Curse,
            (ShoutVariant::Curse, Icon::ShoutCurse),
        ),
        (
            SpellKeywords::Shout_Warcry,
            (ShoutVariant::Warcry, Icon::ShoutWarcry),
        ),
        // vanilla shouts
        (
            SpellKeywords::Shout_AnimalAllegiance,
            (ShoutVariant::AnimalAllegiance, Icon::ShoutAnimalAllegiance),
        ),
        (
            SpellKeywords::Shout_AuraWhisper,
            (ShoutVariant::AuraWhisper, Icon::ShoutAuraWhisper),
        ),
        (
            SpellKeywords::Shout_BattleFury,
            (ShoutVariant::BattleFury, Icon::ShoutBattleFury),
        ),
        (
            SpellKeywords::Shout_BecomeEthereal,
            (ShoutVariant::BecomeEthereal, Icon::ShoutBecomeEthereal),
        ),
        (
            SpellKeywords::Shout_BendWill,
            (ShoutVariant::BendWill, Icon::ShoutBendWill),
        ),
        (
            SpellKeywords::Shout_CallDragon,
            (ShoutVariant::CallDragon, Icon::ShoutCallDragon),
        ),
        (
            SpellKeywords::Shout_CallOfValor,
            (ShoutVariant::CallOfValor, Icon::ShoutCallOfValor),
        ),
        (
            SpellKeywords::Shout_ClearSkies,
            (ShoutVariant::ClearSkies, Icon::ShoutClearSkies),
        ),
        (
            SpellKeywords::Shout_Disarm,
            (ShoutVariant::Disarm, Icon::ShoutDisarm),
        ),
        (
            SpellKeywords::Shout_Dismay,
            (ShoutVariant::Dismay, Icon::ShoutDismay),
        ),
        (
            SpellKeywords::Shout_DragonAspect,
            (ShoutVariant::DragonAspect, Icon::ShoutDragonAspect),
        ),
        (
            SpellKeywords::Shout_Dragonrend,
            (ShoutVariant::Dragonrend, Icon::ShoutDragonrend),
        ),
        (
            SpellKeywords::Shout_DrainVitality,
            (ShoutVariant::DrainVitality, Icon::ShoutDrainVitality),
        ),
        (
            SpellKeywords::Shout_ElementalFury,
            (ShoutVariant::ElementalFury, Icon::ShoutElementalFury),
        ),
        (
            SpellKeywords::Shout_FireBreath,
            (ShoutVariant::FireBreath, Icon::ShoutFireBreath),
        ),
        (
            SpellKeywords::Shout_FrostBreath,
            (ShoutVariant::FrostBreath, Icon::ShoutFrostBreath),
        ),
        (
            SpellKeywords::Shout_IceForm,
            (ShoutVariant::IceForm, Icon::ShoutIceForm),
        ),
        (
            SpellKeywords::Shout_KynesPeace,
            (ShoutVariant::KynesPeace, Icon::ShoutKynesPeace),
        ),
        (
            SpellKeywords::Shout_MarkedForDeath,
            (ShoutVariant::MarkedForDeath, Icon::ShoutMarkedForDeath),
        ),
        (
            SpellKeywords::Shout_Slowtime,
            (ShoutVariant::Slowtime, Icon::ShoutSlowtime),
        ),
        (
            SpellKeywords::Shout_SoulTear,
            (ShoutVariant::SoulTear, Icon::ShoutSoulTear),
        ),
        (
            SpellKeywords::Shout_Stormcall,
            (ShoutVariant::Stormcall, Icon::ShoutStormcall),
        ),
        (
            SpellKeywords::Shout_SummonDurnehviir,
            (ShoutVariant::SummonDurnehviir, Icon::ShoutSummonDurnehviir),
        ),
        (
            SpellKeywords::Shout_ThrowVoice,
            (ShoutVariant::ThrowVoice, Icon::ShoutThrowVoice),
        ),
        (
            SpellKeywords::Shout_UnrelentingForce,
            (ShoutVariant::UnrelentingForce, Icon::ShoutUnrelentingForce),
        ),
        (
            SpellKeywords::Shout_WhirlwindSprint,
            (ShoutVariant::WhirlwindSprint, Icon::ShoutWhirlwindSprint),
        ),
        (
            SpellKeywords::Shout_PhantomForm,
            (ShoutVariant::PhantomForm, Icon::ShoutPhantomForm),
        ),
        (
            SpellKeywords::Shout_AlessiasLove,
            (ShoutVariant::AlessiasLove, Icon::ShoutAlessiasLove),
        ),
        (
            SpellKeywords::Shout_Annihilate,
            (ShoutVariant::Annihilate, Icon::ShoutAnnihilate),
        ),
        (
            SpellKeywords::Shout_ArcaneHelix,
            (ShoutVariant::ArcaneHelix, Icon::ShoutArcaneHelix),
        ),
        (
            SpellKeywords::Shout_Armageddon,
            (ShoutVariant::Armageddon, Icon::ShoutArmageddon),
        ),
        (
            SpellKeywords::Shout_Curse,
            (ShoutVariant::Curse, Icon::ShoutCurse),
        ),
        (
            SpellKeywords::Shout_DanceOfTheDead,
            (ShoutVariant::DanceOfTheDead, Icon::ShoutDanceOfTheDead),
        ),
        (
            SpellKeywords::Shout_Earthquake,
            (ShoutVariant::Earthquake, Icon::ShoutEarthquake),
        ),
        (
            SpellKeywords::Shout_EssenceRip,
            (ShoutVariant::EssenceRip, Icon::ShoutEssenceRip),
        ),
        (
            SpellKeywords::Shout_Evocation,
            (ShoutVariant::Evocation, Icon::ShoutEvocation),
        ),
        (
            SpellKeywords::Shout_Geomagnetism,
            (ShoutVariant::Geomagnetism, Icon::ShoutGeomagnetism),
        ),
        (
            SpellKeywords::Shout_Iceborn,
            (ShoutVariant::Iceborn, Icon::ShoutIceborn),
        ),
        (
            SpellKeywords::Shout_JonesShadow,
            (ShoutVariant::JonesShadow, Icon::ShoutJonesShadow),
        ),
        (
            SpellKeywords::Shout_Kingsbane,
            (ShoutVariant::Kingsbane, Icon::ShoutKingsbane),
        ),
        (
            SpellKeywords::Shout_Lifestream,
            (ShoutVariant::Lifestream, Icon::ShoutLifestream),
        ),
        (
            SpellKeywords::Shout_LightningShield,
            (ShoutVariant::LightningShield, Icon::ShoutLightningShield),
        ),
        (
            SpellKeywords::Shout_Oblivion,
            (ShoutVariant::Oblivion, Icon::ShoutOblivion),
        ),
        (
            SpellKeywords::Shout_PhantomDecoy,
            (ShoutVariant::PhantomDecoy, Icon::ShoutPhantomDecoy),
        ),
        (
            SpellKeywords::Shout_Riftwalk,
            (ShoutVariant::Riftwalk, Icon::ShoutRiftwalk),
        ),
        (
            SpellKeywords::Shout_Shattersphere,
            (ShoutVariant::Shattersphere, Icon::ShoutShattersphere),
        ),
        (
            SpellKeywords::Shout_ShorsWrath,
            (ShoutVariant::ShorsWrath, Icon::ShoutShorsWrath),
        ),
        (
            SpellKeywords::Shout_ShroudOfSnowfall,
            (ShoutVariant::ShroudOfSnowfall, Icon::ShoutShroudOfSnowfall),
        ),
        (
            SpellKeywords::Shout_SpeakUntoTheStars,
            (
                ShoutVariant::SpeakUntoTheStars,
                Icon::ShoutSpeakUntoTheStars,
            ),
        ),
        (
            SpellKeywords::Shout_SplinterTwins,
            (ShoutVariant::SplinterTwins, Icon::ShoutSplinterTwins),
        ),
        (
            SpellKeywords::Shout_Stormblast,
            (ShoutVariant::Stormblast, Icon::ShoutStormblast),
        ),
        (
            SpellKeywords::Shout_TheConqueror,
            (ShoutVariant::TheConqueror, Icon::ShoutTheConqueror),
        ),
        (
            SpellKeywords::Shout_Trueshot,
            (ShoutVariant::Trueshot, Icon::ShoutTrueshot),
        ),
        (
            SpellKeywords::Shout_WailOfTheBanshee,
            (ShoutVariant::WailOfTheBanshee, Icon::ShoutWailOfTheBanshee),
        ),
        (
            SpellKeywords::Shout_Wanderlust,
            (ShoutVariant::Wanderlust, Icon::ShoutWanderlust),
        ),
        (
            SpellKeywords::Shout_LightningBreath,
            (ShoutVariant::LightningBreath, Icon::ShoutLightningBreath),
        ),
        (
            SpellKeywords::Shout_PoisonBreath,
            (ShoutVariant::PoisonBreath, Icon::ShoutPoisonBreath),
        ),
        (
            SpellKeywords::Shout_SoulCairnSummon,
            (ShoutVariant::SoulCairnSummon, Icon::ShoutSoulCairnSummon),
        ),
    ])
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_keywords_used() {
        let shoutwords: Vec<SpellKeywords> = vec![
            SpellKeywords::Shout_AnimalAllegiance,
            SpellKeywords::Shout_AuraWhisper,
            SpellKeywords::Shout_BattleFury,
            SpellKeywords::Shout_BecomeEthereal,
            SpellKeywords::Shout_BendWill,
            SpellKeywords::Shout_CallDragon,
            SpellKeywords::Shout_CallOfValor,
            SpellKeywords::Shout_ClearSkies,
            SpellKeywords::Shout_Disarm,
            SpellKeywords::Shout_Dismay,
            SpellKeywords::Shout_DragonAspect,
            SpellKeywords::Shout_Dragonrend,
            SpellKeywords::Shout_DrainVitality,
            SpellKeywords::Shout_ElementalFury,
            SpellKeywords::Shout_FireBreath,
            SpellKeywords::Shout_FrostBreath,
            SpellKeywords::Shout_IceForm,
            SpellKeywords::Shout_KynesPeace,
            SpellKeywords::Shout_MarkedForDeath,
            SpellKeywords::Shout_Slowtime,
            SpellKeywords::Shout_SoulTear,
            SpellKeywords::Shout_Stormcall,
            SpellKeywords::Shout_SummonDurnehviir,
            SpellKeywords::Shout_ThrowVoice,
            SpellKeywords::Shout_UnrelentingForce,
            SpellKeywords::Shout_WhirlwindSprint,
            // Dawnguard unused spell
            SpellKeywords::Shout_SoulCairnSummon,
            // ForcefulTongue
            SpellKeywords::Shout_PhantomForm,
            // Stormcrown
            SpellKeywords::Shout_LightningBreath,
            SpellKeywords::Shout_PoisonBreath,
            // Thunderchild
            SpellKeywords::Shout_AlessiasLove,
            SpellKeywords::Shout_Annihilate,
            SpellKeywords::Shout_ArcaneHelix,
            SpellKeywords::Shout_Armageddon,
            SpellKeywords::Shout_Curse,
            SpellKeywords::Shout_DanceOfTheDead,
            SpellKeywords::Shout_Earthquake,
            SpellKeywords::Shout_EssenceRip,
            SpellKeywords::Shout_Evocation,
            SpellKeywords::Shout_Geomagnetism,
            SpellKeywords::Shout_Iceborn,
            SpellKeywords::Shout_JonesShadow,
            SpellKeywords::Shout_Kingsbane,
            SpellKeywords::Shout_Lifestream,
            SpellKeywords::Shout_LightningShield,
            SpellKeywords::Shout_Oblivion,
            SpellKeywords::Shout_PhantomDecoy,
            SpellKeywords::Shout_Riftwalk,
            SpellKeywords::Shout_Shattersphere,
            SpellKeywords::Shout_ShorsWrath,
            SpellKeywords::Shout_ShroudOfSnowfall,
            SpellKeywords::Shout_SpeakUntoTheStars,
            SpellKeywords::Shout_SplinterTwins,
            SpellKeywords::Shout_Stormblast,
            SpellKeywords::Shout_TheConqueror,
            SpellKeywords::Shout_Trueshot,
            SpellKeywords::Shout_WailOfTheBanshee,
            SpellKeywords::Shout_Wanderlust,
            SpellKeywords::Shout_Warcry,
        ];

        let unused: Vec<&SpellKeywords> = shoutwords
            .iter()
            .filter(|xs| {
                let shout = ShoutType::new(vec![xs.to_string()]);
                if matches!(shout.variant, ShoutVariant::Unclassified) {
                    eprintln!("{xs} turned into unclassified shout");
                    true
                } else {
                    false
                }
            })
            .collect();
        assert!(unused.is_empty());
    }
}
