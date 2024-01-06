use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use regex::Regex;
use crate::common::{GameRegion, MihoyoGame};
use crate::error::Error;
use crate::utils::expand_env_vars;

const GENSHIN_LOG_LOCATION_CN: &str = r"%userprofile%\AppData\LocalLow\miHoYo\原神\output_log.txt";
const GENSHIN_LOG_LOCATION_OS: &str = r"%userprofile%\AppData\LocalLow\miHoYo\Genshin Impact\output_log.txt";
const STARRAIL_LOG_LOCATION_CN: &str = r"%userprofile%\AppData\LocalLow\miHoYo\崩坏：星穹铁道\Player.log";
const STARRAIL_LOG_LOCATION_OS: &str = r"%userprofile%\AppData\LocalLow\Cognosphere\Star Rail\Player.log";
const ZZZ_LOG_LOCATION_CN: &str = r"%userprofile%\AppData\LocalLow\miHoYo\绝区零\output_log.txt";
const ZZZ_LOG_LOCATION_OS: &str = r"%userprofile%\AppData\LocalLow\miHoYo\Zenless Zone Zero\output_log.txt";

const GENSHIN_LOG_SEARCH_PATTERN: &str = r"(.:/.+(GenshinImpact_Data|YuanShen_Data))";
const STARRAIL_LOG_SEARCH_PATTERN: &str = r"Loading player data from (.:/.+)/data\.unity3d";
const ZZZ_LOG_SEARCH_PATTERN: &str = "";

fn get_log_location(game: &MihoyoGame, region: &GameRegion) -> &'static str {
    return match game {
        MihoyoGame::GenshinImpact => {
            match region {
                GameRegion::CN => GENSHIN_LOG_LOCATION_CN,
                GameRegion::OS => GENSHIN_LOG_LOCATION_OS,
            }
        }
        MihoyoGame::HonkaiStarRail => {
            match region {
                GameRegion::CN => STARRAIL_LOG_LOCATION_CN,
                GameRegion::OS => STARRAIL_LOG_LOCATION_OS,
            }
        }
        MihoyoGame::ZenlessZoneZero => {
            match region {
                GameRegion::CN => ZZZ_LOG_LOCATION_CN,
                GameRegion::OS => ZZZ_LOG_LOCATION_OS,
            }
        }
    }
}

fn get_search_pattern(game: &MihoyoGame) -> &'static str {
    return match game {
        MihoyoGame::GenshinImpact => GENSHIN_LOG_SEARCH_PATTERN,
        MihoyoGame::HonkaiStarRail => STARRAIL_LOG_SEARCH_PATTERN,
        MihoyoGame::ZenlessZoneZero => ZZZ_LOG_SEARCH_PATTERN,
    }
}

pub fn locate_game(game: &MihoyoGame, region: &GameRegion) -> Result<String, Error> {
    let p = expand_env_vars(get_log_location(game, region))?;
    let path = Path::new(&p);
    if !path.exists() {
        return Err(Error::LogFileNotExist);
    }
    let file = File::open(&p)?;
    let reader = BufReader::new(file);

    let re = Regex::new(get_search_pattern(game)).unwrap();
    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(captures) = re.captures(&line) {
                let extracted_string = captures.get(1).unwrap().as_str();
                return Ok(extracted_string.into())
            }
        }
    }
    Err(Error::CannotLocateGameDir)
}

pub fn locate_cache(game: &MihoyoGame, region: &GameRegion) -> Result<String, Error> {
    let game_dir = locate_game(game, region)?;
    let mut subdirs: Vec<_> = fs::read_dir(game_dir.clone() + "/webCaches")?
        .filter_map(|entry| {
            let entry = entry.expect("cannot read entry");
            let path = entry.path();
            if path.is_dir() {
                Some((path, entry.metadata().unwrap().modified().unwrap()))
            } else {
                None
            }
        })
        .collect();

    subdirs.sort_by_key(|&(_, modified)| modified);

    if let Some((latest_version, _)) = subdirs.pop() {
        let version = latest_version.file_name().unwrap();
        let cache_dir = format!("{}/webCaches/{}/Cache/Cache_Data/data_2", &game_dir, version.to_str().unwrap());
        Ok(cache_dir)
    } else {
        Err(Error::CannotLocateCacheDir)
    }
}