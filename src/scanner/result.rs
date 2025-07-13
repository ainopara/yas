use crate::artifact::internal_artifact::{ArtifactSetKey, ArtifactSlotKey, ArtifactStat, CharacterKey, InternalArtifact};
use crate::artifact::internal_relic::{InternalRelic, RelicSetName, RelicSlot, RelicStat};

#[derive(Debug)]
pub struct YasScanResult {
    pub name: String,
    pub main_stat_name: String,
    pub main_stat_value: String,
    pub sub_stat_1: String,
    pub sub_stat_2: String,
    pub sub_stat_3: String,
    pub sub_stat_4: String,
    pub level: String,
    pub location: String,
    pub rarity: u32,
    pub lock: bool,
}

impl YasScanResult {
    pub fn to_internal_artifact(&self, cnt: i32) -> Option<InternalArtifact> {
        let set_key = ArtifactSetKey::from_zh_cn(&self.name)?;
        let slot_key = ArtifactSlotKey::from_zh_cn(&self.name)?;
        let rarity = self.rarity;
        if !self.level.contains("+") {
            return None;
        }
        let level = self
            .level
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<u32>()
            .ok()?;
        let main_stat = ArtifactStat::from_zh_cn_raw(
            (self.main_stat_name.replace("+", "?") + "+" + self.main_stat_value.as_str()).as_str(),
        )?;
        let sub1 = ArtifactStat::from_zh_cn_raw(&self.sub_stat_1);
        let sub2 = ArtifactStat::from_zh_cn_raw(&self.sub_stat_2);
        let sub3 = ArtifactStat::from_zh_cn_raw(&self.sub_stat_3);
        let sub4 = ArtifactStat::from_zh_cn_raw(&self.sub_stat_4);

        let location = if self.location.contains("已装备") {
            let len = self.location.chars().count();
            if let Some(key) = CharacterKey::from_zh_cn(&self.location.chars().take(len - 3).collect::<String>()) {
                Some(key)
            } else {
                println!("Unknown character key for set {} {} {} {} {} {} {} {}", cnt, self.name, self.main_stat_name, self.sub_stat_1, self.sub_stat_2, self.sub_stat_3, self.sub_stat_4, self.location);
                None
            }
        } else {
            None
        };

        let art = InternalArtifact {
            set_key,
            slot_key,
            rarity,
            level,
            location,
            lock: self.lock,
            main_stat,
            sub_stat_1: sub1,
            sub_stat_2: sub2,
            sub_stat_3: sub3,
            sub_stat_4: sub4,
        };
        Some(art)
    }

    pub fn to_internal_relic(&self) -> Option<InternalRelic> {
        let set_name = RelicSetName::from_zh_cn(&self.name)?;
        let slot = RelicSlot::from_zh_cn(&self.name)?;
        let star = self.rarity;
        if !self.level.contains("+") {
            return None;
        }
        let level = self
            .level
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<u32>()
            .ok()?;
        let main_stat = RelicStat::from_zh_cn_raw(
            (self.main_stat_name.clone() + "+" + self.main_stat_value.as_str()).as_str(),
        )?;
        let sub1 = RelicStat::from_zh_cn_raw(&self.sub_stat_1);
        let sub2 = RelicStat::from_zh_cn_raw(&self.sub_stat_2);
        let sub3 = RelicStat::from_zh_cn_raw(&self.sub_stat_3);
        let sub4 = RelicStat::from_zh_cn_raw(&self.sub_stat_4);

        let equip = None;

        let relic = InternalRelic {
            set_name,
            slot,
            star,
            level,
            lock: self.lock,
            main_stat,
            sub_stat_1: sub1,
            sub_stat_2: sub2,
            sub_stat_3: sub3,
            sub_stat_4: sub4,
            equip,
        };
        Some(relic)
    }
}