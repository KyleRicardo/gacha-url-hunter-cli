use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    IoError(#[from] io::Error),
    #[error("{0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("game log file not exist in user profile directory")]
    LogFileNotExist,
    #[error("cannot locate game dir")]
    CannotLocateGameDir,
    #[error("cannot locate cache dir")]
    CannotLocateCacheDir,
    #[error("cache file not exist in game directory")]
    CacheFileNotExist,
    #[error("authkey not found in gacha url")]
    AuthkeyNotFound,
    #[error("no valid gacha url found")]
    NoValidGachaUrl,
}