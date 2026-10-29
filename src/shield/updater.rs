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
                        let clean = content.trim_start_matches('\u{feff}');
                        combined.push_str(clean);
                        combined.push('\n');
                    }
                }
            }
        }
        Ok(combined)
    }

    pub const MIN_FILTER_SIZE: usize = 10_000;
    pub const MAX_FILTER_SIZE: usize = 20 * 1024 * 1024; // 20 MB

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
                    if let Some(cl) = resp.content_length() {
                        if cl as usize > Self::MAX_FILTER_SIZE {
                            warn!("Filter {} Content-Length ({} bytes) exceeds maximum allowed {} bytes, skipping", filename, cl, Self::MAX_FILTER_SIZE);
                            continue;
                        }
                    }
                    let bytes = match resp.bytes().await {
                        Ok(b) => b,
                        Err(e) => {
                            warn!("Failed to read response body for {}: {}", filename, e);
                            continue;
                        }
                    };
                    if bytes.len() < Self::MIN_FILTER_SIZE || bytes.len() > Self::MAX_FILTER_SIZE {
                        warn!("Downloaded filter {} size ({} bytes) out of bounds [{}, {}], preserving existing rules", filename, bytes.len(), Self::MIN_FILTER_SIZE, Self::MAX_FILTER_SIZE);
                        continue;
                    }
                    let content = match std::str::from_utf8(&bytes) {
                        Ok(s) => s,
                        Err(e) => {
                            warn!("Downloaded filter {} is not valid UTF-8: {}", filename, e);
                            continue;
                        }
                    };
                    let trimmed = content.trim_start_matches('\u{feff}');
                    if !trimmed.starts_with("[Adblock Plus") && !trimmed.starts_with('!') {
                        warn!("Downloaded filter {} lacks valid adblock rule header, preserving existing rules", filename);
                        continue;
                    }
                    let target_path = self.rules_dir.join(filename);
                    let tmp_path = self.rules_dir.join(format!("{}.tmp", filename));
                    if let Err(e) = fs::write(&tmp_path, trimmed) {
                        warn!("Failed to write tmp rules file {}: {}", filename, e);
                        let _ = fs::remove_file(&tmp_path);
                        continue;
                    }
                    if let Err(e) = fs::rename(&tmp_path, &target_path) {
                        warn!("Failed to rename tmp rules file {}: {}", filename, e);
                        let _ = fs::remove_file(&tmp_path);
                        continue;
                    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_size_bounds_and_constants() {
        assert_eq!(FilterUpdater::MIN_FILTER_SIZE, 10_000);
        assert_eq!(FilterUpdater::MAX_FILTER_SIZE, 20 * 1024 * 1024);
        assert!(FilterUpdater::MAX_FILTER_SIZE > FilterUpdater::MIN_FILTER_SIZE);
    }

    #[test]
    fn test_filter_header_validation() {
        let valid_header_1 = "[Adblock Plus 2.0]\n! Title: Test List\n||doubleclick.net^";
        let valid_header_2 = "! Title: EasyPrivacy\n||analytics.google.com^";
        let invalid_header = "<html><body>Error 404</body></html>";

        assert!(valid_header_1.starts_with("[Adblock Plus") || valid_header_1.starts_with('!'));
        assert!(valid_header_2.starts_with("[Adblock Plus") || valid_header_2.starts_with('!'));
        assert!(!invalid_header.starts_with("[Adblock Plus") && !invalid_header.starts_with('!'));
    }

    #[test]
    fn test_filter_updater_directory_creation() {
        let updater = FilterUpdater::new();
        assert!(updater.is_ok());
        let u = updater.unwrap();
        assert!(u.rules_dir().exists());
    }
}
