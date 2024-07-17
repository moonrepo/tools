#![allow(dead_code)]

use regex::Regex;

pub fn from_python_version(version: String, regex: &Regex) -> Option<String> {
    let caps = regex.captures(&version)?;

    let mut version = format!(
        "{}.{}.{}",
        &caps["major"],
        &caps["minor"],
        caps.name("patch").map(|c| c.as_str()).unwrap_or("0"),
    );

    if let Some(pre) = caps.name("pre") {
        let preid = format!("-{}.{}", pre.as_str(), &caps["preid"]);
        version.push_str(&preid);
    }

    Some(version)
}
