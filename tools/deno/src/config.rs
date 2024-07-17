#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct DenoPluginConfig {
    pub dist_url: String,
}

impl Default for DenoPluginConfig {
    fn default() -> Self {
        Self {
            dist_url: "https://dl.deno.land/release/v{version}/{file}".into(),
        }
    }
}
