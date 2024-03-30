use crate::common::GameRegion::{CN, OS};
use crate::common::MihoyoGame::{GenshinImpact, HonkaiStarRail, ZenlessZoneZero};
use std::fmt::{Display, Formatter};
use crate::common::ExtractOption::{CompleteUrl, UrlDecodedAuthkeyOnly, UrlEncodedAuthkeyOnly};

#[derive(PartialEq, Eq)]
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

#[derive(PartialEq, Eq)]
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

#[derive(PartialEq, Eq)]
pub enum ExtractOption {
    CompleteUrl,
    UrlEncodedAuthkeyOnly,
    UrlDecodedAuthkeyOnly,
}

impl ExtractOption {
    pub fn vec() -> Vec<ExtractOption> {
        vec![CompleteUrl, UrlEncodedAuthkeyOnly, UrlDecodedAuthkeyOnly]
    }
}

impl Display for ExtractOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CompleteUrl => write!(f, "完整URL Complete URL"),
            UrlEncodedAuthkeyOnly => write!(f, "仅Authkey（URL编码） Authkey Only(Url Encoded)"),
            UrlDecodedAuthkeyOnly => write!(f, "仅Authkey（URL解码） Authkey Only(Url Decoded)"),
        }
    }
}