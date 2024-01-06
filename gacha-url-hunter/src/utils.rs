use std::env;

use lazy_static::lazy_static;
use regex::{Captures, Regex};
use url::Url;

use crate::error::Error;

pub fn expand_env_vars(s:&str) -> Result<String, Error>  {
    lazy_static! {
        static ref ENV_VAR: Regex = Regex::new("%([[:word:]]*)%").expect("Invalid Regex");
    }

    let result: String = ENV_VAR.replace_all(s, |c:&Captures| match &c[1] {
        "" => String::from("%"),
        varname => env::var(varname).unwrap()
    }).into();

    Ok(result)
}


pub fn extract_authkey(url: String) -> Result<String, Error> {
    let url = Url::parse(&url)?;
    let pairs = url.query_pairs();
    for (k, v) in pairs {
        if k == "authkey" {
            return Ok(v.to_string())
        }
    }
    return Err(Error::AuthkeyNotFound)
}
