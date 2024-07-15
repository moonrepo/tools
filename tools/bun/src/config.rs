#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct BunPluginConfig {
    pub dist_url: String,
}

impl Default for BunPluginConfig {
    fn default() -> Self {
        Self {
            dist_url: "https://github.com/oven-sh/bun/releases/download/bun-v{version}/{file}"
                .into(),
        }
    }
}
