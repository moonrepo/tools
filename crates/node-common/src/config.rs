#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct NodePluginConfig {
    pub bundled_npm: bool,
    pub dist_url: String,
}

impl Default for NodePluginConfig {
    fn default() -> Self {
        Self {
            bundled_npm: false,
            dist_url: "https://nodejs.org/download/release/v{version}/{file}".into(),
        }
    }
}

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct NodeDepmanPluginConfig {
    pub shared_globals_dir: bool,
}
