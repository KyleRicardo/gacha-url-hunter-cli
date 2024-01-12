use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    IoError(#[from] io::Error),
    #[error("{0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("无法定位游戏日志目录，请确认您选择了正确的游戏和区服\ngame log file not exist in user profile directory, please ensure you selected the correct game and region")]
    LogFileNotExist,
    #[error("无法定位游戏目录\ncannot locate game dir")]
    CannotLocateGameDir,
    #[error("无法定位抽卡缓存目录\ncannot locate cache dir")]
    CannotLocateCacheDir,
    #[error("缓存文件不存在\ncache file not exist in game directory")]
    CacheFileNotExist,
    #[error("抽卡链接中未能获取到authkey\nauthkey not found in gacha url")]
    AuthkeyNotFound,
    #[error("未获取到有效抽卡链接，请先在游戏内查看抽卡记录！\nno valid gacha url found, please check gacha history in game first")]
    NoValidGachaUrl,
}