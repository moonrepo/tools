#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct GoPluginConfig {
    pub dist_url: String,
    pub gobin: bool,
}

impl Default for GoPluginConfig {
    fn default() -> Self {
        Self {
            dist_url: "https://dl.google.com/go/{file}".into(),
            gobin: true,
        }
    }
}
