use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserConfig {
    pub homepage: String,
    pub shields_enabled: bool,
    pub block_trackers: bool,
    pub block_third_party_scripts: bool,
    pub user_agent: Option<String>,
}

impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            homepage: "https://duckduckgo.com".to_string(),
            shields_enabled: true,
            block_trackers: true,
            block_third_party_scripts: false,
            user_agent: None,
        }
    }
}

impl BrowserConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
