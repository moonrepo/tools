#![allow(dead_code)]

use extism_pdk::{json, Error, HttpResponse};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct RegistryVersion {
    pub version: String, // No v prefix
}

#[derive(Deserialize)]
pub struct RegistryResponse {
    pub name: String,
    #[serde(rename = "dist-tags")]
    pub dist_tags: HashMap<String, String>,
    pub versions: HashMap<String, RegistryVersion>,
}

pub fn parse_registry_response(
    res: HttpResponse,
    is_yarn: bool,
) -> Result<RegistryResponse, Error> {
    if !is_yarn {
        return res.json();
    }

    // https://github.com/moonrepo/proto/issues/257
    let pattern = regex::bytes::Regex::new("[\u{0000}-\u{001F}]+").unwrap();
    let body = res.body();

    Ok(json::from_slice(&pattern.replace_all(&body, b""))?)
}
