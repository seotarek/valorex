use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserConfig {
    #[serde(default = "default_homepage")]
    pub homepage: String,
    #[serde(default = "default_true")]
    pub shields_enabled: bool,
    #[serde(default = "default_true")]
    pub block_trackers: bool,
    #[serde(default = "default_false")]
    pub block_third_party_scripts: bool,
    #[serde(default)]
    pub user_agent: Option<String>,
    #[serde(default = "default_lang")]
    pub language: String,
    #[serde(default = "default_true")]
    pub window_controls_visible: bool,
    #[serde(default = "default_true")]
    pub tab_close_visible: bool,
    #[serde(default = "default_false")]
    pub decorations_visible: bool,
    #[serde(default = "default_false")]
    pub incognito: bool,
}

fn default_homepage() -> String {
    "valorex://newtab".to_string()
}
fn default_true() -> bool {
    true
}
fn default_false() -> bool {
    false
}
fn default_lang() -> String {
    "ar".to_string()
}

impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            homepage: default_homepage(),
            shields_enabled: true,
            block_trackers: true,
            block_third_party_scripts: false,
            user_agent: None,
            language: default_lang(),
            window_controls_visible: true,
            tab_close_visible: true,
            decorations_visible: false,
            incognito: false,
        }
    }
}

impl BrowserConfig {
    pub fn new() -> Self {
        Self::load()
    }

    pub fn config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let mut path = dirs::config_dir().ok_or("Cannot locate user config directory")?;
        path.push("valorex");
        path.push("config.json");
        Ok(path)
    }

    pub fn load() -> Self {
        match Self::config_path() {
            Ok(path) if path.exists() => {
                match fs::read_to_string(&path) {
                    Ok(content) => match serde_json::from_str::<BrowserConfig>(&content) {
                        Ok(cfg) => {
                            info!("Loaded persistent configuration from {}", path.display());
                            cfg
                        }
                        Err(e) => {
                            warn!("Corrupted config.json: {}. Backing up and using defaults.", e);
                            let backup_path = path.with_extension(format!("backup.{}", chrono::Utc::now().timestamp()));
                            let _ = fs::rename(&path, backup_path);
                            Self::default()
                        }
                    },
                    Err(e) => {
                        warn!("Failed to read config file: {}. Using defaults.", e);
                        Self::default()
                    }
                }
            }
            _ => Self::default(),
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        let tmp_path = path.with_extension("tmp");
        fs::write(&tmp_path, json)?;
        fs::rename(&tmp_path, &path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults_and_json() {
        let default_cfg = BrowserConfig::default();
        assert_eq!(default_cfg.homepage, "valorex://newtab");
        assert!(default_cfg.shields_enabled);
        assert_eq!(default_cfg.language, "ar");
        assert!(default_cfg.window_controls_visible);

        let json = serde_json::to_string(&default_cfg).unwrap();
        let decoded: BrowserConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.language, "ar");
        assert_eq!(decoded.homepage, "valorex://newtab");
    }
}


