#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct NodeDepmanPluginConfig {
    pub dist_url: String,
    pub shared_globals_dir: bool,
}

impl Default for NodeDepmanPluginConfig {
    fn default() -> Self {
        Self {
            dist_url:
                "https://registry.npmjs.org/{package}/-/{package_without_scope}-{version}.tgz"
                    .into(),
            shared_globals_dir: false,
        }
    }
}
