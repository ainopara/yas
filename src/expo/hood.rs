use std::convert::From;
use std::fs::File;
use std::io::prelude::*;

use serde::ser::{Serialize, SerializeMap, Serializer};

use crate::artifact::internal_relic::{
    RelicSetName, RelicSlot, RelicStat, RelicStatName, InternalRelic,
};

struct HoodRelic<'a> {
    relic: &'a InternalRelic,
}

impl RelicStatName {
    pub fn to_hood(&self) -> String {
        let temp = match self {
            RelicStatName::HP => "hp",
            RelicStatName::HPPercentage => "hp_",
            RelicStatName::ATK => "atk",
            RelicStatName::ATKPercentage => "atk_",
            RelicStatName::DEFPercentage => "def_",
            RelicStatName::SPD => "spd",
            RelicStatName::CRITRate => "critRate",
            RelicStatName::CRITDMG => "critDMG",
            RelicStatName::BreakEffect => "break",
            RelicStatName::OutgoingHealingBoost => "heal",
            RelicStatName::EnergyRegenerationRate => "enerRegen",
            RelicStatName::EffectHitRate => "eff",
            RelicStatName::PhysicalDMGBoost => "physicalDmg",
            RelicStatName::FireDMGBoost => "fireDmg",
            RelicStatName::IceDMGBoost => "iceDmg",
            RelicStatName::LightningDMGBoost => "lightningDmg",
            RelicStatName::WindDMGBoost => "windDmg",
            RelicStatName::QuantumDMGBoost => "quantumDmg",
            RelicStatName::ImaginaryDMGBoost => "imaginaryDmg",
            RelicStatName::DEF => "def",
            RelicStatName::EffectRES => "effRes",
        };
        String::from(temp)
    }
}

impl RelicSetName {
    pub fn to_hood(&self) -> String {
        let same = self.to_string();
        let temp = match self {
            RelicSetName::PasserbyofWanderingCloud => "PasserbyofWanderingCloud",
            RelicSetName::MusketeerofWildWheat => "MusketeerofWildWheat",
            RelicSetName::KnightofPurityPalace => "KnightofPurityPalace",
            RelicSetName::HunterofGlacialForest => "HunterofGlacialForest",
            RelicSetName::ChampionofStreetwiseBoxing => "ChampionofStreetwiseBoxing",
            RelicSetName::GuardofWutheringSnow => "GuardofWutheringSnow",
            RelicSetName::FiresmithofLavaForging => "FiresmithofLavaForging",
            RelicSetName::GeniusofBrilliantStars => "GeniusofBrilliantStars",
            RelicSetName::BandofSizzlingThunder => "BandofSizzlingThunder",
            RelicSetName::EagleofTwilightLine => "EagleofTwilightLine",
            RelicSetName::ThiefofShootingMeteor => "ThiefofShootingMeteor",
            RelicSetName::WastelanderofBanditryDesert => "WastelanderofBanditryDesert",
            RelicSetName::SpaceSealingStation => "SpaceSealingStation",
            RelicSetName::FleetoftheAgeless => "FleetoftheAgeless",
            RelicSetName::PanCosmicCommercialEnterprise => "PanCosmicCommercialEnterprise",
            RelicSetName::BelobogoftheArchitects => "BelobogoftheArchitects",
            RelicSetName::CelestialDifferentiator => "CelestialDifferentiator",
            RelicSetName::InertSalsotto => "InertSalsotto",
            RelicSetName::TaliaKingdomofBanditry => "TaliaKingdomofBanditry",
            RelicSetName::SprightlyVonwacq => "SprightlyVonwacq",
            RelicSetName::RutilantArena => "RutilantArena",
            RelicSetName::BrokenKeel => "BrokenKeel",
            RelicSetName::LongevousDisciple => "LongevousDisciple",
            RelicSetName::MessengerTraversingHackerspace => "MessengerTraversingHackerspace",
            _ => same.as_str(),
        };
        String::from(temp)
    }
}

impl RelicSlot {
    pub fn to_hood(&self) -> String {
        let temp = match self {
            RelicSlot::Head => "head",
            RelicSlot::Hands => "hands",
            RelicSlot::Body => "body",
            RelicSlot::Feet => "feet",
            RelicSlot::PlanarSphere => "planarSphere",
            RelicSlot::LinkRope => "linkRope",
        };
        String::from(temp)
    }
}

struct HoodRelicStat<'a> {
    stat: &'a RelicStat,
}

impl<'a> Serialize for HoodRelicStat<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut root = serializer.serialize_map(Some(2))?;
        root.serialize_entry("key", &self.stat.name.to_hood())?;
        root.serialize_entry("value", &self.stat.value)?;
        root.end()
    }
}

impl<'a> Serialize for HoodRelic<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut root = serializer.serialize_map(Some(7))?;

        root.serialize_entry("setKey", &self.relic.set_name.to_hood())?;
        root.serialize_entry("slotKey", &self.relic.slot.to_hood())?;
        root.serialize_entry("level", &self.relic.level)?;
        root.serialize_entry("rarity", &self.relic.star)?;
        root.serialize_entry("lock", &self.relic.lock)?;
        root.serialize_entry("mainStatKey", &self.relic.main_stat.name.to_hood())?;
        root.serialize_entry("mainTag", &self.relic.main_stat)?;

        let mut substats: Vec<HoodRelicStat> = vec![];
        if let Some(ref s) = self.relic.sub_stat_1 {
            substats.push(HoodRelicStat { stat: s });
        }
        if let Some(ref s) = self.relic.sub_stat_2 {
            substats.push(HoodRelicStat { stat: s });
        }
        if let Some(ref s) = self.relic.sub_stat_3 {
            substats.push(HoodRelicStat { stat: s });
        }
        if let Some(ref s) = self.relic.sub_stat_4 {
            substats.push(HoodRelicStat { stat: s });
        }

        root.serialize_entry("substats", &substats)?;
        root.serialize_entry("level", &self.relic.level)?;
        root.serialize_entry("star", &self.relic.star)?;
        root.serialize_entry("equip", &self.relic.equip)?;
        root.end()
    }
}

pub struct HoodFormat<'a> {
    format: String,
    version: u32,
    source: String,
    relics: Vec<HoodRelic<'a>>,
}

impl<'a> Serialize for HoodFormat<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut root = serializer.serialize_map(Some(4))?;
        root.serialize_entry("format", &self.format)?;
        root.serialize_entry("version", &self.version)?;
        root.serialize_entry("source", &self.source)?;
        root.serialize_entry("relics", &self.relics)?;
        root.end()
    }
}

impl<'a> HoodFormat<'a> {
    pub fn new(results: &Vec<InternalRelic>) -> HoodFormat {

        let relics = results
            .iter()
            .map(| internal_relic | HoodRelic { relic: internal_relic } )
            .collect();


        HoodFormat {
            format: String::from("HOOD"),
            version: 1,
            source: String::from("yas-lock"),
            relics
        }
    }

    pub fn save(&self, path: String) {
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", path, why),
            Ok(file) => file,
        };
        let s = serde_json::to_string(&self).unwrap();

        match file.write_all(s.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", path, why),
            _ => {},
        }
    }
}
