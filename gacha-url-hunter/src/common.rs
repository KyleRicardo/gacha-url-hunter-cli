use std::fmt::{Display, Formatter};
use crate::common::GameRegion::{CN, OS};
use crate::common::MihoyoGame::{GenshinImpact, HonkaiStarRail, ZenlessZoneZero};

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
            MihoyoGame::GenshinImpact => write!(f, "原神 Genshin Impact"),
            MihoyoGame::HonkaiStarRail => write!(f, "崩坏：星穹铁道 Honkai: Star Rail"),
            MihoyoGame::ZenlessZoneZero => write!(f, "绝区零: Zenless Zone Zero"),
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
            GameRegion::CN => write!(f, "国服（包含官服和B服） China"),
            GameRegion::OS => write!(f, "外服 Overseas"),
        }
    }
}