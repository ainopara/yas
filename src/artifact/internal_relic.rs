use regex::Regex;
use std::hash::{Hash, Hasher};
use edit_distance;
use log::error;
use strum_macros::Display;
use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum RelicStatName {
    HP,
    HPPercentage,
    ATK,
    ATKPercentage,
    DEFPercentage,
    SPD,
    CRITRate,
    CRITDMG,
    BreakEffect,
    OutgoingHealingBoost,
    EnergyRegenerationRate,
    EffectHitRate,
    PhysicalDMGBoost,
    FireDMGBoost,
    IceDMGBoost,
    LightningDMGBoost,
    WindDMGBoost,
    QuantumDMGBoost,
    ImaginaryDMGBoost,
    DEF,
    EffectRES,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum RelicSlot {
    Head,
    Hands,
    Body,
    Feet,
    PlanarSphere,
    LinkRope,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
#[derive(Display)]
pub enum RelicSetName {
    PasserbyofWanderingCloud,  // 云无留迹的过客
    MusketeerofWildWheat,  // 野穗伴行的快枪手
    KnightofPurityPalace,  // 净庭教宗的圣骑士
    HunterofGlacialForest,  // 密林卧雪的猎人
    ChampionofStreetwiseBoxing,  // 街头出身的拳王
    GuardofWutheringSnow,  // 戍卫风雪的铁卫
    FiresmithofLavaForging,  // 熔岩锻铸的火匠
    GeniusofBrilliantStars,  // 繁星璀璨的天才
    BandofSizzlingThunder,  // 激奏雷电的乐队
    EagleofTwilightLine,  // 晨昏交界的翔鹰
    ThiefofShootingMeteor,  // 流星追迹的怪盗
    WastelanderofBanditryDesert,  // 盗匪荒漠的废土客
    LongevousDisciple,  // 宝命长存的莳者
    MessengerTraversingHackerspace,  // 骇域漫游的信使
    TheAshblazingGrandDuke,  // 毁烬焚骨的大公
    PrisonerinDeepConfinement,  // 幽锁深牢的系囚
    PioneerDiverofDeadWaters,  // 死水深潜的先驱
    WatchmakerMasterofDreamMachinations,  // 机心戏梦的钟表匠
    IronCavalryAgainsttheScourge,  // 荡除蠹灾的铁骑
    TheWindSoaringValorous,  // 风举云飞的勇烈
    SpaceSealingStation,  // 太空封印站
    FleetoftheAgeless,  // 不老者的仙舟
    PanCosmicCommercialEnterprise,  // 泛银河商业公司
    BelobogoftheArchitects,  // 筑城者的贝洛伯格
    CelestialDifferentiator,  // 星体差分机
    InertSalsotto,  // 停止转动的萨尔索图
    TaliaKingdomofBanditry,  // 盗贼公国塔利亚
    SprightlyVonwacq,  // 生命的翁瓦克
    RutilantArena,  // 繁星竞技场
    BrokenKeel,  // 折断的龙骨
    FirmamentFrontlineGlamoth,  // 苍穹战线格拉默
    PenaconyLandoftheDreams,  // 梦想之地匹诺康尼
    DuranDynastyofRunningWolves,  // 奔狼的都蓝王朝
    ForgeoftheKalpagniLantern,  // 劫火莲灯铸炼宫
}

#[derive(Debug, Clone)]
pub struct RelicStat {
    pub name: RelicStatName,
    pub value: f64,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct InternalRelic {
    pub set_name: RelicSetName,
    pub slot: RelicSlot,
    pub star: u32,
    pub level: u32,
    pub lock: bool,
    pub main_stat: RelicStat,
    pub sub_stat_1: Option<RelicStat>,
    pub sub_stat_2: Option<RelicStat>,
    pub sub_stat_3: Option<RelicStat>,
    pub sub_stat_4: Option<RelicStat>,
    pub equip: Option<String>,
}

impl Hash for RelicStat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        let v = (self.value * 1000.0) as i32;
        v.hash(state);
    }
}

impl PartialEq for RelicStat {
    fn eq(&self, other: &Self) -> bool {
        if self.name != other.name {
            return false;
        }

        let v1 = (self.value * 1000.0) as i32;
        let v2 = (other.value * 1000.0) as i32;

        v1 == v2
    }
}

impl Eq for RelicStat {}

impl RelicStatName {
    pub fn from_zh_cn(name: &str, is_percentage: bool) -> Option<RelicStatName> {
        match name {
            "生命值" => if is_percentage { Some(RelicStatName::HPPercentage) } else { Some(RelicStatName::HP) },
            "攻击力" => if is_percentage { Some(RelicStatName::ATKPercentage) } else { Some(RelicStatName::ATK) },
            "防御力" => if is_percentage { Some(RelicStatName::DEFPercentage) } else { Some(RelicStatName::DEF) },
            "速度" => Some(RelicStatName::SPD),
            "暴击率" => Some(RelicStatName::CRITRate),
            "暴击伤害" => Some(RelicStatName::CRITDMG),
            "击破特攻" => Some(RelicStatName::BreakEffect),
            "治疗量加成" => Some(RelicStatName::OutgoingHealingBoost),
            "能量恢复效率" => Some(RelicStatName::EnergyRegenerationRate),
            "效果命中" => Some(RelicStatName::EffectHitRate),
            "物理属性伤害提高" => Some(RelicStatName::PhysicalDMGBoost),
            "火属性伤害提高" => Some(RelicStatName::FireDMGBoost),
            "冰属性伤害提高" => Some(RelicStatName::IceDMGBoost),
            "雷属性伤害提高" => Some(RelicStatName::LightningDMGBoost),
            "风属性伤害提高" => Some(RelicStatName::WindDMGBoost),
            "量子属性伤害提高" => Some(RelicStatName::QuantumDMGBoost),
            "虚数属性伤害提高" => Some(RelicStatName::ImaginaryDMGBoost),
            "效果抵抗" => Some(RelicStatName::EffectRES),
            _ => None,
        }
    }
}

impl RelicStat {
    // e.g "生命值+4,123", "暴击率+10%"
    pub fn from_zh_cn_raw(s: &str) -> Option<RelicStat> {
        let temp: Vec<&str> = s.split("+").collect();
        if temp.len() != 2 {
            return None;
        }

        let is_percentage = temp[1].contains("%");
        let stat_name = match RelicStatName::from_zh_cn(temp[0], is_percentage) {
            Some(v) => v,
            None => return None,
        };

        let re = Regex::new("[%,]").unwrap();
        let mut value = match re.replace_all(temp[1], "").parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                error!("stat `{}` parse error", s);
                return None;
            },
        };
        if is_percentage {
            value /= 100.0;
        }

        Some(RelicStat {
            name: stat_name,
            value,
        })
    }
}

lazy_static! {
    static ref RELIC_INFO: HashMap<&'static str, (RelicSetName, RelicSlot)> = {
        let mut m = HashMap::new();
        // 云无留迹的过客 - Version 1.0
        m.insert("过客的逢春木簪", (RelicSetName::PasserbyofWanderingCloud, RelicSlot::Head));
        m.insert("过客的游龙臂鞲", (RelicSetName::PasserbyofWanderingCloud, RelicSlot::Hands));
        m.insert("过客的残绣风衣", (RelicSetName::PasserbyofWanderingCloud, RelicSlot::Body));
        m.insert("过客的冥途游履", (RelicSetName::PasserbyofWanderingCloud, RelicSlot::Feet));
        // 野穗伴行的快枪手 - Version 1.0
        m.insert("快枪手的野穗毡帽", (RelicSetName::MusketeerofWildWheat, RelicSlot::Head));
        m.insert("快枪手的粗革手套", (RelicSetName::MusketeerofWildWheat, RelicSlot::Hands));
        m.insert("快枪手的猎风披肩", (RelicSetName::MusketeerofWildWheat, RelicSlot::Body));
        m.insert("快枪手的铆钉马靴", (RelicSetName::MusketeerofWildWheat, RelicSlot::Feet));
        // 净庭教宗的圣骑士 - Version 1.0
        m.insert("圣骑的宽恕盔面", (RelicSetName::KnightofPurityPalace, RelicSlot::Head));
        m.insert("圣骑的沉默誓环", (RelicSetName::KnightofPurityPalace, RelicSlot::Hands));
        m.insert("圣骑的肃穆胸甲", (RelicSetName::KnightofPurityPalace, RelicSlot::Body));
        m.insert("圣骑的秩序铁靴", (RelicSetName::KnightofPurityPalace, RelicSlot::Feet));
        // 密林卧雪的猎人 - Version 1.0
        m.insert("雪猎的荒神兜帽", (RelicSetName::HunterofGlacialForest, RelicSlot::Head));
        m.insert("雪猎的巨蜥手套", (RelicSetName::HunterofGlacialForest, RelicSlot::Hands));
        m.insert("雪猎的冰龙披风", (RelicSetName::HunterofGlacialForest, RelicSlot::Body));
        m.insert("雪猎的鹿皮软靴", (RelicSetName::HunterofGlacialForest, RelicSlot::Feet));
        // 街头出身的拳王 - Version 1.0
        m.insert("拳王的冠军护头", (RelicSetName::ChampionofStreetwiseBoxing, RelicSlot::Head));
        m.insert("拳王的重炮拳套", (RelicSetName::ChampionofStreetwiseBoxing, RelicSlot::Hands));
        m.insert("拳王的贴身护胸", (RelicSetName::ChampionofStreetwiseBoxing, RelicSlot::Body));
        m.insert("拳王的弧步战靴", (RelicSetName::ChampionofStreetwiseBoxing, RelicSlot::Feet));
        // 戍卫风雪的铁卫 - Version 1.0
        m.insert("铁卫的铸铁面盔", (RelicSetName::GuardofWutheringSnow, RelicSlot::Head));
        m.insert("铁卫的银鳞手甲", (RelicSetName::GuardofWutheringSnow, RelicSlot::Hands));
        m.insert("铁卫的旧制军服", (RelicSetName::GuardofWutheringSnow, RelicSlot::Body));
        m.insert("铁卫的白银护胫", (RelicSetName::GuardofWutheringSnow, RelicSlot::Feet));
        // 熔岩锻铸的火匠 - Version 1.0
        m.insert("火匠的黑曜目镜", (RelicSetName::FiresmithofLavaForging, RelicSlot::Head));
        m.insert("火匠的御火戒指", (RelicSetName::FiresmithofLavaForging, RelicSlot::Hands));
        m.insert("火匠的阻燃围裙", (RelicSetName::FiresmithofLavaForging, RelicSlot::Body));
        m.insert("火匠的合金义肢", (RelicSetName::FiresmithofLavaForging, RelicSlot::Feet));
        // 繁星璀璨的天才 - Version 1.0
        m.insert("天才的超距遥感", (RelicSetName::GeniusofBrilliantStars, RelicSlot::Head));
        m.insert("天才的频变捕手", (RelicSetName::GeniusofBrilliantStars, RelicSlot::Hands));
        m.insert("天才的元域深潜", (RelicSetName::GeniusofBrilliantStars, RelicSlot::Body));
        m.insert("天才的引力漫步", (RelicSetName::GeniusofBrilliantStars, RelicSlot::Feet));
        // 激奏雷电的乐队 - Version 1.0
        m.insert("乐队的偏光墨镜", (RelicSetName::BandofSizzlingThunder, RelicSlot::Head));
        m.insert("乐队的巡演手绳", (RelicSetName::BandofSizzlingThunder, RelicSlot::Hands));
        m.insert("乐队的钉刺皮衣", (RelicSetName::BandofSizzlingThunder, RelicSlot::Body));
        m.insert("乐队的铆钉短靴", (RelicSetName::BandofSizzlingThunder, RelicSlot::Feet));
        // 晨昏交界的翔鹰 - Version 1.0
        m.insert("翔鹰的长喙头盔", (RelicSetName::EagleofTwilightLine, RelicSlot::Head));
        m.insert("翔鹰的鹰击指环", (RelicSetName::EagleofTwilightLine, RelicSlot::Hands));
        m.insert("翔鹰的翼装束带", (RelicSetName::EagleofTwilightLine, RelicSlot::Body));
        m.insert("翔鹰的绒羽绑带", (RelicSetName::EagleofTwilightLine, RelicSlot::Feet));
        // 流星追迹的怪盗 - Version 1.0
        m.insert("怪盗的千人假面", (RelicSetName::ThiefofShootingMeteor, RelicSlot::Head));
        m.insert("怪盗的绘纹手套", (RelicSetName::ThiefofShootingMeteor, RelicSlot::Hands));
        m.insert("怪盗的纤钢爪钩", (RelicSetName::ThiefofShootingMeteor, RelicSlot::Body));
        m.insert("怪盗的流星快靴", (RelicSetName::ThiefofShootingMeteor, RelicSlot::Feet));
        // 盗匪荒漠的废土客 - Version 1.0
        m.insert("废土客的呼吸面罩", (RelicSetName::WastelanderofBanditryDesert, RelicSlot::Head));
        m.insert("废土客的荒漠终端", (RelicSetName::WastelanderofBanditryDesert, RelicSlot::Hands));
        m.insert("废土客的修士长袍", (RelicSetName::WastelanderofBanditryDesert, RelicSlot::Body));
        m.insert("废土客的动力腿甲", (RelicSetName::WastelanderofBanditryDesert, RelicSlot::Feet));
        // 宝命长存的莳者 - Version 1.2
        m.insert("莳者的复明义眼", (RelicSetName::LongevousDisciple, RelicSlot::Head));
        m.insert("莳者的机巧木手", (RelicSetName::LongevousDisciple, RelicSlot::Hands));
        m.insert("莳者的承露羽衣", (RelicSetName::LongevousDisciple, RelicSlot::Body));
        m.insert("莳者的天人丝履", (RelicSetName::LongevousDisciple, RelicSlot::Feet));
        // 骇域漫游的信使 - Version 1.2
        m.insert("信使的全息目镜", (RelicSetName::MessengerTraversingHackerspace, RelicSlot::Head));
        m.insert("信使的百变义手", (RelicSetName::MessengerTraversingHackerspace, RelicSlot::Hands));
        m.insert("信使的密信挎包", (RelicSetName::MessengerTraversingHackerspace, RelicSlot::Body));
        m.insert("信使的酷跑板鞋", (RelicSetName::MessengerTraversingHackerspace, RelicSlot::Feet));
        // 毁烬焚骨的大公 - Version 1.5
        m.insert("大公的冥焰冠冕", (RelicSetName::TheAshblazingGrandDuke, RelicSlot::Head));
        m.insert("大公的绒火指套", (RelicSetName::TheAshblazingGrandDuke, RelicSlot::Hands));
        m.insert("大公的蒙恩长袍", (RelicSetName::TheAshblazingGrandDuke, RelicSlot::Body));
        m.insert("大公的绅雅礼靴", (RelicSetName::TheAshblazingGrandDuke, RelicSlot::Feet));
        // 幽锁深牢的系囚 - Version 1.5
        m.insert("系囚的合啮拘笼", (RelicSetName::PrisonerinDeepConfinement, RelicSlot::Head));
        m.insert("系囚的铅石梏铐", (RelicSetName::PrisonerinDeepConfinement, RelicSlot::Hands));
        m.insert("系囚的幽闭缚束", (RelicSetName::PrisonerinDeepConfinement, RelicSlot::Body));
        m.insert("系囚的绝足锁桎", (RelicSetName::PrisonerinDeepConfinement, RelicSlot::Feet));
        // 死水深潜的先驱 - Version 2.0
        m.insert("先驱的绝热围壳", (RelicSetName::PioneerDiverofDeadWaters, RelicSlot::Head));
        m.insert("先驱的虚极罗盘", (RelicSetName::PioneerDiverofDeadWaters, RelicSlot::Hands));
        m.insert("先驱的密合铅衣", (RelicSetName::PioneerDiverofDeadWaters, RelicSlot::Body));
        m.insert("先驱的泊星桩锚", (RelicSetName::PioneerDiverofDeadWaters, RelicSlot::Feet));
        // 机心戏梦的钟表匠 - Version 2.0
        m.insert("钟表匠的极目透镜", (RelicSetName::WatchmakerMasterofDreamMachinations, RelicSlot::Head));
        m.insert("钟表匠的交运腕表", (RelicSetName::WatchmakerMasterofDreamMachinations, RelicSlot::Hands));
        m.insert("钟表匠的空幻礼服", (RelicSetName::WatchmakerMasterofDreamMachinations, RelicSlot::Body));
        m.insert("钟表匠的隐梦革履", (RelicSetName::WatchmakerMasterofDreamMachinations, RelicSlot::Feet));
        // 荡除蠹灾的铁骑 - Version 2.3
        m.insert("铁骑的索敌战盔", (RelicSetName::IronCavalryAgainsttheScourge, RelicSlot::Head));
        m.insert("铁骑的摧坚铁腕", (RelicSetName::IronCavalryAgainsttheScourge, RelicSlot::Hands));
        m.insert("铁骑的银影装甲", (RelicSetName::IronCavalryAgainsttheScourge, RelicSlot::Body));
        m.insert("铁骑的行空护胫", (RelicSetName::IronCavalryAgainsttheScourge, RelicSlot::Feet));
        // 风举云飞的勇烈 - Version 2.3
        m.insert("勇烈的玄枵面甲", (RelicSetName::TheWindSoaringValorous, RelicSlot::Head));
        m.insert("勇烈的钩爪腕甲", (RelicSetName::TheWindSoaringValorous, RelicSlot::Hands));
        m.insert("勇烈的飞翎瓷甲", (RelicSetName::TheWindSoaringValorous, RelicSlot::Body));
        m.insert("勇烈的逐猎腿甲", (RelicSetName::TheWindSoaringValorous, RelicSlot::Feet));
        // 太空封印站 - Version 1.0
        m.insert("「黑塔」的空间站点", (RelicSetName::SpaceSealingStation, RelicSlot::PlanarSphere));
        m.insert("「黑塔」的漫历轨迹", (RelicSetName::SpaceSealingStation, RelicSlot::LinkRope));
        // 不老者的仙舟 - Version 1.0
        m.insert("罗浮仙舟的天外楼船", (RelicSetName::FleetoftheAgeless, RelicSlot::PlanarSphere));
        m.insert("罗浮仙舟的建木枝蔓", (RelicSetName::FleetoftheAgeless, RelicSlot::LinkRope));
        // 泛银河商业公司 - Version 1.0
        m.insert("公司的巨构总部", (RelicSetName::PanCosmicCommercialEnterprise, RelicSlot::PlanarSphere));
        m.insert("公司的贸易航道", (RelicSetName::PanCosmicCommercialEnterprise, RelicSlot::LinkRope));
        // 筑城者的贝洛伯格 - Version 1.0
        m.insert("贝洛伯格的存护堡垒", (RelicSetName::BelobogoftheArchitects, RelicSlot::PlanarSphere));
        m.insert("贝洛伯格的铁卫防线", (RelicSetName::BelobogoftheArchitects, RelicSlot::LinkRope));
        // 星体差分机 - Version 1.0
        m.insert("螺丝星的机械烈阳", (RelicSetName::CelestialDifferentiator, RelicSlot::PlanarSphere));
        m.insert("螺丝星的环星孔带", (RelicSetName::CelestialDifferentiator, RelicSlot::LinkRope));
        // 停止转动的萨尔索图 - Version 1.0
        m.insert("萨尔索图的移动城市", (RelicSetName::InertSalsotto, RelicSlot::PlanarSphere));
        m.insert("萨尔索图的晨昏界线", (RelicSetName::InertSalsotto, RelicSlot::LinkRope));
        // 盗贼公国塔利亚 - Version 1.0
        m.insert("塔利亚的钉壳小镇", (RelicSetName::TaliaKingdomofBanditry, RelicSlot::PlanarSphere));
        m.insert("塔利亚的裸皮电线", (RelicSetName::TaliaKingdomofBanditry, RelicSlot::LinkRope));
        // 生命的翁瓦克 - Version 1.0
        m.insert("翁瓦克的诞生之岛", (RelicSetName::SprightlyVonwacq, RelicSlot::PlanarSphere));
        m.insert("翁瓦克的环岛海岸", (RelicSetName::SprightlyVonwacq, RelicSlot::LinkRope));
        // 繁星竞技场 - Version 1.2
        m.insert("泰科铵的镭射球场", (RelicSetName::RutilantArena, RelicSlot::PlanarSphere));
        m.insert("泰科铵的弧光赛道", (RelicSetName::RutilantArena, RelicSlot::LinkRope));
        // 折断的龙骨 - Version 1.2
        m.insert("伊须磨洲的残船鲸落", (RelicSetName::BrokenKeel, RelicSlot::PlanarSphere));
        m.insert("伊须磨洲的坼裂缆索", (RelicSetName::BrokenKeel, RelicSlot::LinkRope));
        // 苍穹战线格拉默 - Version 1.5
        m.insert("格拉默的铁骑兵团", (RelicSetName::FirmamentFrontlineGlamoth, RelicSlot::PlanarSphere));
        m.insert("格拉默的寂静坟碑", (RelicSetName::FirmamentFrontlineGlamoth, RelicSlot::LinkRope));
        // 梦想之地匹诺康尼 - Version 1.5
        m.insert("匹诺康尼的堂皇饭店", (RelicSetName::PenaconyLandoftheDreams, RelicSlot::PlanarSphere));
        m.insert("匹诺康尼的逐梦轨道", (RelicSetName::PenaconyLandoftheDreams, RelicSlot::LinkRope));
        // 奔狼的都蓝王朝 - Version 2.3
        m.insert("都蓝的器兽缰辔", (RelicSetName::DuranDynastyofRunningWolves, RelicSlot::PlanarSphere));
        m.insert("都蓝的穹窿金帐", (RelicSetName::DuranDynastyofRunningWolves, RelicSlot::LinkRope));
        // 劫火莲灯铸炼宫 - Version 2.3
        m.insert("铸炼宫的焰轮天绸", (RelicSetName::ForgeoftheKalpagniLantern, RelicSlot::PlanarSphere));
        m.insert("铸炼宫的莲华灯芯", (RelicSetName::ForgeoftheKalpagniLantern, RelicSlot::LinkRope));
        m
    };
}

impl RelicSetName {
    pub fn from_zh_cn(s: &str) -> Option<RelicSetName> {
        RELIC_INFO.get(s).map(|&(relic_set_name, _)| relic_set_name)
    }
}

impl RelicSlot {
    pub fn from_zh_cn(s: &str) -> Option<RelicSlot> {
        RELIC_INFO.get(s).map(|&(_, relic_slot)| relic_slot)
    }
}
