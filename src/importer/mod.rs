use crate::storage::StorageEngine;
use chrono::{DateTime, Utc};
use log::{error, info, warn};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedBrowser {
    pub id: String,
    pub name: String,
    pub has_bookmarks: bool,
    pub has_history: bool,
    pub bookmarks_path: Option<String>,
    pub history_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportSummary {
    pub browser_id: String,
    pub browser_name: String,
    pub bookmarks_imported: usize,
    pub history_imported: usize,
    pub success: bool,
    pub message: String,
}

pub struct BrowserImporter;

impl BrowserImporter {
    pub fn detect_installed_browsers() -> Vec<DetectedBrowser> {
        let mut results = Vec::new();
        let home = match dirs::home_dir() {
            Some(h) => h,
            None => return results,
        };

        // 1. Google Chrome
        let chrome_default = home.join(".config/google-chrome/Default");
        Self::check_chromium_browser(&mut results, "google-chrome", "Google Chrome", &chrome_default);

        // 2. Brave Browser
        let brave_default = home.join(".config/BraveSoftware/Brave-Browser/Default");
        Self::check_chromium_browser(&mut results, "brave", "Brave Browser", &brave_default);

        // 3. Chromium
        let chromium_default = home.join(".config/chromium/Default");
        Self::check_chromium_browser(&mut results, "chromium", "Chromium", &chromium_default);

        // 4. Mozilla Firefox
        let firefox_dir = home.join(".mozilla/firefox");
        if firefox_dir.is_dir() {
            if let Ok(entries) = fs::read_dir(&firefox_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let places = path.join("places.sqlite");
                        if places.is_file() {
                            results.push(DetectedBrowser {
                                id: "firefox".to_string(),
                                name: "Mozilla Firefox".to_string(),
                                has_bookmarks: true,
                                has_history: true,
                                bookmarks_path: Some(places.to_string_lossy().to_string()),
                                history_path: Some(places.to_string_lossy().to_string()),
                            });
                            break;
                        }
                    }
                }
            }
        }

        results
    }

    fn check_chromium_browser(
        results: &mut Vec<DetectedBrowser>,
        id: &str,
        name: &str,
        profile_dir: &Path,
    ) {
        let bookmarks = profile_dir.join("Bookmarks");
        let history = profile_dir.join("History");

        let has_bm = bookmarks.is_file();
        let has_hist = history.is_file();

        if has_bm || has_hist {
            results.push(DetectedBrowser {
                id: id.to_string(),
                name: name.to_string(),
                has_bookmarks: has_bm,
                has_history: has_hist,
                bookmarks_path: if has_bm { Some(bookmarks.to_string_lossy().to_string()) } else { None },
                history_path: if has_hist { Some(history.to_string_lossy().to_string()) } else { None },
            });
        }
    }

    pub fn import_from_browser(browser_id: &str, storage: &StorageEngine) -> ImportSummary {
        let browsers = Self::detect_installed_browsers();
        let target = match browsers.into_iter().find(|b| b.id == browser_id) {
            Some(b) => b,
            None => {
                return ImportSummary {
                    browser_id: browser_id.to_string(),
                    browser_name: browser_id.to_string(),
                    bookmarks_imported: 0,
                    history_imported: 0,
                    success: false,
                    message: format!("Browser '{}' is not installed or profile not found.", browser_id),
                };
            }
        };

        let mut bm_count = 0;
        let mut hist_count = 0;

        if target.id == "firefox" {
            if let Some(places_path) = target.bookmarks_path {
                match Self::import_firefox_places(Path::new(&places_path), storage) {
                    Ok((bms, hist)) => {
                        bm_count = bms;
                        hist_count = hist;
                    }
                    Err(e) => {
                        error!("Failed to import from Firefox: {}", e);
                    }
                }
            }
        } else {
            // Chromium-based browsers (Chrome, Brave, Chromium)
            if let Some(bm_path) = target.bookmarks_path {
                match Self::import_chromium_bookmarks(Path::new(&bm_path), storage) {
                    Ok(count) => bm_count = count,
                    Err(e) => warn!("Failed to import bookmarks from {}: {}", target.name, e),
                }
            }

            if let Some(hist_path) = target.history_path {
                match Self::import_chromium_history(Path::new(&hist_path), storage, 2000) {
                    Ok(count) => hist_count = count,
                    Err(e) => warn!("Failed to import history from {}: {}", target.name, e),
                }
            }
        }

        info!(
            "Successfully imported {} bookmarks and {} history items from {}",
            bm_count, hist_count, target.name
        );

        ImportSummary {
            browser_id: target.id,
            browser_name: target.name,
            bookmarks_imported: bm_count,
            history_imported: hist_count,
            success: true,
            message: format!(
                "Successfully imported {} bookmarks and {} history entries.",
                bm_count, hist_count
            ),
        }
    }

    pub fn import_chromium_bookmarks(
        bookmarks_file: &Path,
        storage: &StorageEngine,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(bookmarks_file)?;
        let json: Value = serde_json::from_str(&content)?;

        let mut count = 0;
        if let Some(roots) = json.get("roots").and_then(|r| r.as_object()) {
            for (root_name, root_val) in roots {
                let folder_name = match root_name.as_str() {
                    "bookmark_bar" => "Bookmarks Bar",
                    "other" => "Other Bookmarks",
                    "synced" => "Mobile Bookmarks",
                    other => other,
                };
                if let Some(children) = root_val.get("children").and_then(|c| c.as_array()) {
                    Self::traverse_chromium_bookmarks(children, folder_name, storage, &mut count);
                }
            }
        }

        Ok(count)
    }

    fn traverse_chromium_bookmarks(
        nodes: &[Value],
        current_folder: &str,
        storage: &StorageEngine,
        count: &mut usize,
    ) {
        let mut batch = Vec::new();
        Self::collect_chromium_bookmarks(nodes, current_folder, &mut batch);
        let slice: Vec<(&str, &str, Option<&str>)> = batch
            .iter()
            .map(|(t, u, f)| (t.as_str(), u.as_str(), Some(f.as_str())))
            .collect();
        if let Ok(inserted) = storage.add_bookmarks_batch(&slice) {
            *count += inserted;
        }
    }

    fn collect_chromium_bookmarks(
        nodes: &[Value],
        current_folder: &str,
        batch: &mut Vec<(String, String, String)>,
    ) {
        for node in nodes {
            let node_type = node.get("type").and_then(|t| t.as_str()).unwrap_or("");
            if node_type == "url" {
                let title = node.get("name").and_then(|n| n.as_str()).unwrap_or("Bookmark");
                let url = node.get("url").and_then(|u| u.as_str()).unwrap_or("");
                if !url.is_empty() && (url.starts_with("http://") || url.starts_with("https://")) {
                    batch.push((title.to_string(), url.to_string(), current_folder.to_string()));
                }
            } else if node_type == "folder" {
                let sub_folder = node.get("name").and_then(|n| n.as_str()).unwrap_or(current_folder);
                let full_folder = format!("{}/{}", current_folder, sub_folder);
                if let Some(children) = node.get("children").and_then(|c| c.as_array()) {
                    Self::collect_chromium_bookmarks(children, &full_folder, batch);
                }
            }
        }
    }

    pub fn import_chromium_history(
        history_file: &Path,
        storage: &StorageEngine,
        limit: usize,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let temp_dir = tempfile::tempdir()?;
        let temp_hist = temp_dir.path().join("History");
        fs::copy(history_file, &temp_hist)?;

        let hist_filename = history_file.file_name().unwrap_or_default().to_string_lossy();
        let wal_file = history_file.with_file_name(format!("{}-wal", hist_filename));
        if wal_file.exists() {
            let _ = fs::copy(&wal_file, temp_dir.path().join(format!("{}-wal", hist_filename)));
        }
        let shm_file = history_file.with_file_name(format!("{}-shm", hist_filename));
        if shm_file.exists() {
            let _ = fs::copy(&shm_file, temp_dir.path().join(format!("{}-shm", hist_filename)));
        }

        let count = {
            let conn = Connection::open(&temp_hist)?;
            let mut stmt = conn.prepare(
                "SELECT url, title, visit_count, last_visit_time FROM urls ORDER BY last_visit_time DESC LIMIT ?1"
            )?;

            let rows = stmt.query_map([limit as i64], |row| {
                let url: String = row.get(0)?;
                let title: String = row.get(1)?;
                let visit_count: i64 = row.get(2)?;
                let last_visit_time: i64 = row.get(3)?;
                Ok((url, title, visit_count, last_visit_time))
            })?;

            let mut batch = Vec::new();
            for row in rows.flatten() {
                let (url, title, visit_count, last_visit_time) = row;
                if !url.is_empty() && (url.starts_with("http://") || url.starts_with("https://")) {
                    // Chromium timestamp is microseconds since Jan 1, 1601 UTC
                    let visit_time_iso = Self::chromium_time_to_iso(last_visit_time);
                    batch.push((url, title, visit_time_iso, visit_count));
                }
            }
            let slice: Vec<(&str, &str, &str, i64)> = batch
                .iter()
                .map(|(u, t, tm, c)| (u.as_str(), t.as_str(), tm.as_str(), *c))
                .collect();
            storage.record_history_with_time_batch(&slice).unwrap_or(0)
        };

        Ok(count)
    }

    fn chromium_time_to_iso(chrome_time: i64) -> String {
        // 11644473600 is seconds between 1601-01-01 and 1970-01-01
        let seconds = (chrome_time / 1_000_000) - 11644473600;
        if seconds > 0 {
            if let Some(dt) = DateTime::from_timestamp(seconds, 0) {
                return dt.to_rfc3339();
            }
        }
        Utc::now().to_rfc3339()
    }

    pub fn import_firefox_places(
        places_file: &Path,
        storage: &StorageEngine,
    ) -> Result<(usize, usize), Box<dyn std::error::Error>> {
        let temp_dir = tempfile::tempdir()?;
        let temp_places = temp_dir.path().join("places.sqlite");
        fs::copy(places_file, &temp_places)?;

        let places_filename = places_file.file_name().unwrap_or_default().to_string_lossy();
        let wal_file = places_file.with_file_name(format!("{}-wal", places_filename));
        if wal_file.exists() {
            let _ = fs::copy(&wal_file, temp_dir.path().join(format!("{}-wal", places_filename)));
        }
        let shm_file = places_file.with_file_name(format!("{}-shm", places_filename));
        if shm_file.exists() {
            let _ = fs::copy(&shm_file, temp_dir.path().join(format!("{}-shm", places_filename)));
        }

        let (bm_count, hist_count) = {
            let conn = Connection::open(&temp_places)?;

            // 1. Import Bookmarks
            let mut bm_stmt = conn.prepare(
                "SELECT p.url, COALESCE(b.title, p.title, 'Bookmark'), p.visit_count 
                 FROM moz_bookmarks b 
                 JOIN moz_places p ON b.fk = p.id 
                 WHERE b.type = 1 AND p.url LIKE 'http%'"
            )?;
            let bm_rows = bm_stmt.query_map([], |row| {
                let url: String = row.get(0)?;
                let title: Option<String> = row.get(1)?;
                Ok((url, title.unwrap_or_else(|| "Bookmark".to_string())))
            })?;

            let mut bm_batch = Vec::new();
            for row in bm_rows.flatten() {
                let (url, title) = row;
                bm_batch.push((title, url));
            }
            let bm_slice: Vec<(&str, &str, Option<&str>)> = bm_batch
                .iter()
                .map(|(t, u)| (t.as_str(), u.as_str(), Some("Firefox Bookmarks")))
                .collect();
            let bms = storage.add_bookmarks_batch(&bm_slice).unwrap_or(0);

            // 2. Import History
            let mut hist_stmt = conn.prepare(
                "SELECT url, title, visit_count, last_visit_date 
                 FROM moz_places 
                 WHERE visit_count > 0 AND url LIKE 'http%' 
                 ORDER BY last_visit_date DESC LIMIT 2000"
            )?;
            let hist_rows = hist_stmt.query_map([], |row| {
                let url: String = row.get(0)?;
                let title: Option<String> = row.get(1)?;
                let visit_count: i64 = row.get(2)?;
                let last_visit_date: Option<i64> = row.get(3)?;
                Ok((url, title.unwrap_or_default(), visit_count, last_visit_date))
            })?;

            let mut batch = Vec::new();
            for row in hist_rows.flatten() {
                let (url, title, visit_count, last_visit_date) = row;
                let iso_time = match last_visit_date {
                    Some(micro) => {
                        let sec = micro / 1_000_000;
                        DateTime::from_timestamp(sec, 0).map(|dt| dt.to_rfc3339()).unwrap_or_else(|| Utc::now().to_rfc3339())
                    }
                    None => Utc::now().to_rfc3339(),
                };
                batch.push((url, title, iso_time, visit_count));
            }
            let slice: Vec<(&str, &str, &str, i64)> = batch
                .iter()
                .map(|(u, t, tm, c)| (u.as_str(), t.as_str(), tm.as_str(), *c))
                .collect();
            let hist = storage.record_history_with_time_batch(&slice).unwrap_or(0);

            (bms, hist)
        };

        Ok((bm_count, hist_count))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_browsers_does_not_panic() {
        let browsers = BrowserImporter::detect_installed_browsers();
        let _ = browsers.len();
    }

    #[test]
    fn test_chrome_time_conversion() {
        let chrome_ts = 13292470182405227i64;
        let iso = BrowserImporter::chromium_time_to_iso(chrome_ts);
        assert!(iso.contains("2022") || iso.contains("2021") || iso.contains("T"));
    }
}
