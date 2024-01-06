use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use regex::bytes::Regex;
use crate::common::{GameRegion, MihoyoGame};
use crate::error::Error;
use crate::locator::locate_cache;
use crate::validator::validate;

pub fn hunt_gacha_url(game: MihoyoGame, region: GameRegion) -> Result<String, Error> {
    let cache_file_path = locate_cache(&game, &region)?;
    let path = Path::new(&cache_file_path);
    if !path.exists() {
        return Err(Error::CacheFileNotExist)
    }
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();

    reader.read_to_end(&mut buf)?;

    let split_re = Regex::new(r"1/0/").unwrap();
    let re = Regex::new(r"(https.+?getGachaLog.+?end_id=)").unwrap();
    let urls: Vec<_> = split_re.split(&buf)
        .filter_map(|s| re.captures(s).map(|c| c.get(1).unwrap().as_bytes()))
        .map(String::from_utf8_lossy)
        .collect();

    let mut valid_url: Option<String> = None;
    for url in urls {
        match validate(&url) {
            Ok(valid) => {
                if valid {
                    valid_url = Some(String::from(url))
                }
            }
            Err(e) => {
                eprintln!("{}", e)
            }
        }
    }

    return match valid_url {
        Some(url) => {
            Ok(url)
        }
        None => {
            Err(Error::NoValidGachaUrl)
        }
    }
}