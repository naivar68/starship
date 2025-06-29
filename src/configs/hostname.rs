use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct HostnameConfig<'a> {
    pub ssh_only: bool,
    pub ssh_symbol: &'a str,
    pub trim_at: &'a str,
    pub detect_env_vars: Vec<&'a str>,
    pub format: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub aliases: IndexMap<String, &'a str>,
}

impl Default for HostnameConfig<'_> {
    fn default() -> Self {
        Self {
            ssh_only: true,
            ssh_symbol: "🌐 ",
            trim_at: ".",
            detect_env_vars: vec![],
            format: "[$ssh_symbol$hostname]($style) in ",
            style: "green dimmed bold",
            disabled: false,
            aliases: IndexMap::new(),
        }
    }
}
