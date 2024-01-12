use crate::common::GameRegion::{CN, OS};
use crate::common::MihoyoGame::{GenshinImpact, HonkaiStarRail, ZenlessZoneZero};
use std::fmt::{Display, Formatter};

pub enum MihoyoGame {
    GenshinImpact,
    HonkaiStarRail,
    ZenlessZoneZero,
}

impl MihoyoGame {
    pub fn vec() -> Vec<MihoyoGame> {
        vec![GenshinImpact, HonkaiStarRail, ZenlessZoneZero]
    }
}

impl Display for MihoyoGame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GenshinImpact => write!(f, "原神 Genshin Impact"),
            HonkaiStarRail => write!(f, "崩坏：星穹铁道 Honkai: Star Rail"),
            ZenlessZoneZero => write!(f, "绝区零: Zenless Zone Zero"),
        }
    }
}

pub enum GameRegion {
    CN,
    OS,
}

impl GameRegion {
    pub fn vec() -> Vec<GameRegion> {
        vec![CN, OS]
    }
}

impl Display for GameRegion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CN => write!(f, "国服（包含官服和B服） China"),
            OS => write!(f, "外服 Overseas"),
        }
    }
}
