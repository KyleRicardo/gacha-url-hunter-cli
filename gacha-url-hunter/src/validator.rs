use serde::Deserialize;
use std::error::Error;
use url::Url;

pub trait UrlValidator {
    fn new() -> Self;
    fn validate_cn(&self, url: &str) -> Result<bool, Box<dyn Error>>;
    fn validate_os(&self, url: &str) -> Result<bool, Box<dyn Error>>;
}

#[derive(Deserialize)]
struct GachaResponse {
    retcode: i32,
}

pub fn validate(url: &str) -> Result<bool, Box<dyn Error>> {
    let url = Url::parse(url)?;
    let response: GachaResponse = reqwest::blocking::get(url)?.json()?;
    Ok(response.retcode == 0)
}
