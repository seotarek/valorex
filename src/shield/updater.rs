use log::{info, warn};
use std::fs;
use std::path::PathBuf;

pub struct FilterUpdater {
    rules_dir: PathBuf,
}

impl FilterUpdater {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut path = dirs::config_dir().ok_or("Cannot locate config directory")?;
        path.push("valorex");
        path.push("rules");
        fs::create_dir_all(&path)?;
        Ok(Self { rules_dir: path })
    }

    pub fn rules_dir(&self) -> &PathBuf {
        &self.rules_dir
    }

    pub fn load_local_rules(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut combined = String::new();
        if self.rules_dir.exists() {
            for entry in fs::read_dir(&self.rules_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("txt") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        combined.push_str(&content);
                        combined.push('\n');
                    }
                }
            }
        }
        Ok(combined)
    }

    pub async fn update_filters(&self) -> Result<usize, Box<dyn std::error::Error>> {
        let filter_sources = [
            ("easylist.txt", "https://easylist.to/easylist/easylist.txt"),
            ("easyprivacy.txt", "https://easylist.to/easylist/easyprivacy.txt"),
        ];

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .build()?;

        let mut updated = 0;
        for (filename, url) in filter_sources {
            info!("Fetching latest filter list from: {}", url);
            match client.get(url).send().await {
                Ok(resp) if resp.status().is_success() => {
                    let content = resp.text().await?;
                    let target_path = self.rules_dir.join(filename);
                    fs::write(&target_path, content)?;
                    info!("Successfully updated {}", filename);
                    updated += 1;
                }
                Ok(resp) => {
                    warn!("Failed to download {}: status {}", filename, resp.status());
                }
                Err(e) => {
                    warn!("Network error downloading {}: {}", filename, e);
                }
            }
        }

        Ok(updated)
    }
}
