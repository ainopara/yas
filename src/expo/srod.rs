use std::convert::From;
use std::fs::File;
use std::io::prelude::*;

use serde::ser::{Serialize, SerializeMap, Serializer};

use crate::artifact::internal_relic::{
    RelicSetName, RelicSlot, RelicStat, RelicStatName, InternalRelic,
};

struct SRODRelic<'a> {
    relic: &'a InternalRelic,
}

impl RelicStatName {
    pub fn to_srod(&self) -> String {
        let temp = match self {
            RelicStatName::HP => "hp",
            RelicStatName::HPPercentage => "hp_",
            RelicStatName::ATK => "atk",
            RelicStatName::ATKPercentage => "atk_",
            RelicStatName::DEFPercentage => "def_",
            RelicStatName::SPD => "spd",
            RelicStatName::CRITRate => "crit_",
            RelicStatName::CRITDMG => "crit_dmg_",
            RelicStatName::BreakEffect => "brEff_",
            RelicStatName::OutgoingHealingBoost => "heal_",
            RelicStatName::EnergyRegenerationRate => "enerRegen_",
            RelicStatName::EffectHitRate => "eff_",
            RelicStatName::PhysicalDMGBoost => "physical_dmg_",
            RelicStatName::FireDMGBoost => "fire_dmg_",
            RelicStatName::IceDMGBoost => "ice_dmg_",
            RelicStatName::LightningDMGBoost => "lightning_dmg_",
            RelicStatName::WindDMGBoost => "wind_dmg_",
            RelicStatName::QuantumDMGBoost => "quantum_dmg_",
            RelicStatName::ImaginaryDMGBoost => "imaginary_dmg_",
            RelicStatName::DEF => "def",
            RelicStatName::EffectRES => "eff_res_",
        };
        String::from(temp)
    }
}

impl RelicSetName {
    pub fn to_srod(&self) -> String {
        match self {
            RelicSetName::PanCosmicCommercialEnterprise => String::from("PanGalacticCommercialEnterprise"),
            _ => self.to_string()
        }
    }
}

impl RelicSlot {
    pub fn to_srod(&self) -> String {
        let temp = match self {
            RelicSlot::Head => "head",
            RelicSlot::Hands => "hand",
            RelicSlot::Body => "body",
            RelicSlot::Feet => "feet",
            RelicSlot::PlanarSphere => "sphere",
            RelicSlot::LinkRope => "rope",
        };
        String::from(temp)
    }
}

struct SRODRelicStat<'a> {
    stat: &'a RelicStat,
}

impl<'a> Serialize for SRODRelicStat<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut root = serializer.serialize_map(Some(2))?;
        root.serialize_entry("key", &self.stat.name.to_srod())?;
        root.serialize_entry("value", &self.stat.value)?;
        root.end()
    }
}

impl<'a> Serialize for SRODRelic<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut root = serializer.serialize_map(Some(7))?;

        root.serialize_entry("setKey", &self.relic.set_name.to_srod())?;
        root.serialize_entry("slotKey", &self.relic.slot.to_srod())?;
        root.serialize_entry("level", &self.relic.level)?;
        root.serialize_entry("rarity", &self.relic.star)?;
        root.serialize_entry("mainStatKey", &self.relic.main_stat.name.to_srod())?;
        root.serialize_entry("location", &self.relic.equip)?;
        root.serialize_entry("lock", &self.relic.lock)?;

        let mut substats: Vec<SRODRelicStat> = vec![];
        if let Some(ref s) = self.relic.sub_stat_1 {
            substats.push(SRODRelicStat { stat: s });
        }
        if let Some(ref s) = self.relic.sub_stat_2 {
            substats.push(SRODRelicStat { stat: s });
        }
        if let Some(ref s) = self.relic.sub_stat_3 {
            substats.push(SRODRelicStat { stat: s });
        }
        if let Some(ref s) = self.relic.sub_stat_4 {
            substats.push(SRODRelicStat { stat: s });
        }

        root.serialize_entry("substats", &substats)?;

        root.end()
    }
}

pub struct SRODFormat<'a> {
    format: String,
    version: u32,
    source: String,
    relics: Vec<SRODRelic<'a>>,
}

impl<'a> Serialize for SRODFormat<'a> {
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

impl<'a> SRODFormat<'a> {
    pub fn new(results: &Vec<InternalRelic>) -> SRODFormat {

        let relics = results
            .iter()
            .map(| internal_relic | SRODRelic { relic: internal_relic } )
            .collect();


        SRODFormat {
            format: String::from("SROD"),
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
