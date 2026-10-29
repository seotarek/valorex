use chrono::Utc;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: i64,
    pub url: String,
    pub title: String,
    pub visit_time: String,
    pub visit_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkEntry {
    pub id: i64,
    pub title: String,
    pub url: String,
    pub created_at: String,
    pub folder: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadEntry {
    pub id: i64,
    pub filename: String,
    pub url: String,
    pub path: String,
    pub timestamp: String,
    pub status: String,
}

#[derive(Clone)]
pub struct StorageEngine {
    conn: Arc<Mutex<Connection>>,
}

impl StorageEngine {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db_path = Self::default_db_path()?;
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(&db_path)?;
        let engine = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        engine.init_schema()?;
        Ok(engine)
    }

    pub fn new_in_memory() -> Result<Self, Box<dyn std::error::Error>> {
        let conn = Connection::open_in_memory()?;
        let engine = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        engine.init_schema()?;
        Ok(engine)
    }

    fn default_db_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let mut path = dirs::config_dir().ok_or("Failed to locate user config directory")?;
        path.push("valorex");
        path.push("valorex.db");
        Ok(path)
    }

    fn init_schema(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        conn.execute_batch(
            "
            PRAGMA journal_mode = WAL;
            PRAGMA busy_timeout = 5000;
            PRAGMA synchronous = NORMAL;
            PRAGMA foreign_keys = ON;

            CREATE TABLE IF NOT EXISTS history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                url TEXT NOT NULL UNIQUE,
                title TEXT NOT NULL,
                visit_time TEXT NOT NULL,
                visit_count INTEGER NOT NULL DEFAULT 1
            );

            CREATE TABLE IF NOT EXISTS bookmarks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                url TEXT NOT NULL UNIQUE,
                created_at TEXT NOT NULL,
                folder TEXT NOT NULL DEFAULT 'default'
            );

            CREATE TABLE IF NOT EXISTS site_shields (
                domain TEXT PRIMARY KEY,
                shields_enabled INTEGER NOT NULL DEFAULT 1,
                aggressive_mode INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS downloads (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                filename TEXT NOT NULL,
                url TEXT NOT NULL,
                path TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'completed'
            );

            CREATE INDEX IF NOT EXISTS idx_history_time ON history(visit_time DESC);
            CREATE INDEX IF NOT EXISTS idx_downloads_time ON downloads(timestamp DESC);
            ",
        )?;
        Ok(())
    }

    pub fn record_history(&self, url: &str, title: &str) -> Result<(), Box<dyn std::error::Error>> {
        if url.trim().is_empty() || url.starts_with("valorex://") {
            return Ok(());
        }
        let now = Utc::now().to_rfc3339();
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        conn.execute(
            "
            INSERT INTO history (url, title, visit_time, visit_count)
            VALUES (?1, ?2, ?3, 1)
            ON CONFLICT(url) DO UPDATE SET
                title = CASE WHEN length(?2) > 0 THEN ?2 ELSE history.title END,
                visit_time = ?3,
                visit_count = history.visit_count + 1
            ",
            params![url, title, now],
        )?;
        Ok(())
    }

    pub fn record_history_with_time(&self, url: &str, title: &str, visit_time: &str, visit_count: i64) -> Result<(), Box<dyn std::error::Error>> {
        if url.trim().is_empty() || url.starts_with("valorex://") {
            return Ok(());
        }
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        conn.execute(
            "
            INSERT INTO history (url, title, visit_time, visit_count)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(url) DO UPDATE SET
                title = CASE WHEN length(?2) > 0 THEN ?2 ELSE history.title END,
                visit_time = MAX(history.visit_time, ?3),
                visit_count = MAX(history.visit_count, ?4)
            ",
            params![url, title, visit_time, visit_count],
        )?;
        Ok(())
    }

    pub fn record_history_with_time_batch(&self, entries: &[(&str, &str, &str, i64)]) -> Result<usize, Box<dyn std::error::Error>> {
        if entries.is_empty() {
            return Ok(0);
        }
        let mut conn = self.conn.lock().map_err(|e| e.to_string())?;
        let tx = conn.transaction()?;
        let mut count = 0;
        {
            let mut stmt = tx.prepare(
                "INSERT INTO history (url, title, visit_time, visit_count)
                 VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT(url) DO UPDATE SET
                     title = CASE WHEN length(?2) > 0 THEN ?2 ELSE history.title END,
                     visit_time = MAX(history.visit_time, ?3),
                     visit_count = MAX(history.visit_count, ?4)",
            )?;
            for (url, title, visit_time, visit_count) in entries {
                if url.trim().is_empty() || url.starts_with("valorex://") {
                    continue;
                }
                stmt.execute(params![url, title, visit_time, visit_count])?;
                count += 1;
            }
        }
        tx.commit()?;
        Ok(count)
    }

    pub fn update_history_title(&self, url: &str, title: &str) -> Result<(), Box<dyn std::error::Error>> {
        if url.trim().is_empty() || title.trim().is_empty() || url.starts_with("valorex://") {
            return Ok(());
        }
        let now = Utc::now().to_rfc3339();
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        conn.execute(
            "INSERT INTO history (url, title, visit_time, visit_count)
             VALUES (?1, ?2, ?3, 1)
             ON CONFLICT(url) DO UPDATE SET
                 title = CASE WHEN length(?2) > 0 THEN ?2 ELSE history.title END",
            params![url, title, now],
        )?;
        Ok(())
    }

    pub fn get_recent_history(&self, limit: usize) -> Result<Vec<HistoryEntry>, Box<dyn std::error::Error>> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        let mut stmt = conn.prepare(
            "SELECT id, url, title, visit_time, visit_count FROM history ORDER BY visit_time DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit as i64], |row| {
            Ok(HistoryEntry {
                id: row.get(0)?,
                url: row.get(1)?,
                title: row.get(2)?,
                visit_time: row.get(3)?,
                visit_count: row.get(4)?,
            })
        })?;

        let mut entries = Vec::new();
        for entry in rows {
            entries.push(entry?);
        }
        Ok(entries)
    }

    pub fn search_history(&self, query: &str) -> Result<Vec<HistoryEntry>, Box<dyn std::error::Error>> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        let escaped = query.replace('\\', "\\\\").replace('%', "\\%").replace('_', "\\_");
        let pattern = format!("%{}%", escaped);
        let mut stmt = conn.prepare(
            "SELECT id, url, title, visit_time, visit_count FROM history 
             WHERE url LIKE ?1 ESCAPE '\\' OR title LIKE ?1 ESCAPE '\\' 
             ORDER BY visit_time DESC LIMIT 100",
        )?;
        let rows = stmt.query_map(params![pattern], |row| {
            Ok(HistoryEntry {
                id: row.get(0)?,
                url: row.get(1)?,
                title: row.get(2)?,
                visit_time: row.get(3)?,
                visit_count: row.get(4)?,
            })
        })?;

        let mut entries = Vec::new();
        for entry in rows {
            entries.push(entry?);
        }
        Ok(entries)
    }

    pub fn clear_history(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        conn.execute("DELETE FROM history", [])?;
        Ok(())
    }

    pub fn add_bookmark(&self, title: &str, url: &str, folder: Option<&str>) -> Result<i64, Box<dyn std::error::Error>> {
        let now = Utc::now().to_rfc3339();
        let target_folder = folder.unwrap_or("default");
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        let id: i64 = conn.query_row(
            "
            INSERT INTO bookmarks (title, url, created_at, folder)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(url) DO UPDATE SET
                title = ?1,
                folder = ?4
            RETURNING id
            ",
            params![title, url, now, target_folder],
            |row| row.get(0),
        )?;
        Ok(id)
    }

    pub fn add_bookmarks_batch(&self, bookmarks: &[(&str, &str, Option<&str>)]) -> Result<usize, Box<dyn std::error::Error>> {
        if bookmarks.is_empty() {
            return Ok(0);
        }
        let now = Utc::now().to_rfc3339();
        let mut conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        let tx = conn.transaction()?;
        let mut count = 0;
        {
            let mut stmt = tx.prepare(
                "INSERT INTO bookmarks (title, url, created_at, folder)
                 VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT(url) DO UPDATE SET
                     title = ?1,
                     folder = ?4",
            )?;
            for (title, url, folder) in bookmarks {
                if url.trim().is_empty() {
                    continue;
                }
                let target_folder = folder.unwrap_or("default");
                stmt.execute(params![title, url, now, target_folder])?;
                count += 1;
            }
        }
        tx.commit()?;
        Ok(count)
    }

    pub fn get_bookmarks(&self) -> Result<Vec<BookmarkEntry>, Box<dyn std::error::Error>> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        let mut stmt = conn.prepare("SELECT id, title, url, created_at, folder FROM bookmarks ORDER BY id DESC")?;
        let rows = stmt.query_map([], |row| {
            Ok(BookmarkEntry {
                id: row.get(0)?,
                title: row.get(1)?,
                url: row.get(2)?,
                created_at: row.get(3)?,
                folder: row.get(4)?,
            })
        })?;

        let mut entries = Vec::new();
        for entry in rows {
            entries.push(entry?);
        }
        Ok(entries)
    }

    pub fn delete_bookmark(&self, id: i64) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        conn.execute("DELETE FROM bookmarks WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn delete_bookmark_by_url(&self, url: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        let affected = conn.execute("DELETE FROM bookmarks WHERE url = ?1", params![url])?;
        Ok(affected > 0)
    }

    pub fn is_bookmarked(&self, url: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM bookmarks WHERE url = ?1")?;
        let count: i64 = stmt.query_row(params![url], |row| row.get(0))?;
        Ok(count > 0)
    }

    pub fn get_site_shield(&self, domain: &str) -> Result<Option<bool>, Box<dyn std::error::Error>> {
        let norm_domain = domain.trim().to_lowercase();
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        let mut stmt = conn.prepare("SELECT shields_enabled FROM site_shields WHERE domain = ?1")?;
        let mut rows = stmt.query(params![norm_domain])?;
        if let Some(row) = rows.next()? {
            let val: i64 = row.get(0)?;
            Ok(Some(val != 0))
        } else {
            Ok(None)
        }
    }

    pub fn set_site_shield(&self, domain: &str, enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
        let norm_domain = domain.trim().to_lowercase();
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        conn.execute(
            "
            INSERT INTO site_shields (domain, shields_enabled, aggressive_mode)
            VALUES (?1, ?2, 0)
            ON CONFLICT(domain) DO UPDATE SET shields_enabled = ?2
            ",
            params![norm_domain, if enabled { 1 } else { 0 }],
        )?;
        Ok(())
    }

    pub fn record_download(&self, filename: &str, url: &str, path: &str) -> Result<i64, Box<dyn std::error::Error>> {
        self.record_download_with_status(filename, url, path, "completed")
    }

    pub fn record_download_with_status(&self, filename: &str, url: &str, path: &str, status: &str) -> Result<i64, Box<dyn std::error::Error>> {
        let now = Utc::now().to_rfc3339();
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        conn.execute(
            "
            INSERT INTO downloads (filename, url, path, timestamp, status)
            VALUES (?1, ?2, ?3, ?4, ?5)
            ",
            params![filename, url, path, now, status],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_downloads(&self) -> Result<Vec<DownloadEntry>, Box<dyn std::error::Error>> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        let mut stmt = conn.prepare("SELECT id, filename, url, path, timestamp, status FROM downloads ORDER BY id DESC")?;
        let rows = stmt.query_map([], |row| {
            Ok(DownloadEntry {
                id: row.get(0)?,
                filename: row.get(1)?,
                url: row.get(2)?,
                path: row.get(3)?,
                timestamp: row.get(4)?,
                status: row.get(5)?,
            })
        })?;

        let mut entries = Vec::new();
        for entry in rows {
            entries.push(entry?);
        }
        Ok(entries)
    }

    pub fn clear_downloads(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        conn.execute("DELETE FROM downloads", [])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_flow() {
        let storage = StorageEngine::new_in_memory().unwrap();
        storage.record_history("https://rust-lang.org", "Rust Programming").unwrap();
        storage.record_history("https://duckduckgo.com", "DuckDuckGo").unwrap();
        storage.record_history("https://rust-lang.org", "Rust Programming Language").unwrap();

        let entries = storage.get_recent_history(10).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].url, "https://rust-lang.org");
        assert_eq!(entries[0].visit_count, 2);
        assert_eq!(entries[0].title, "Rust Programming Language");

        let search = storage.search_history("duck").unwrap();
        assert_eq!(search.len(), 1);
        assert_eq!(search[0].url, "https://duckduckgo.com");

        storage.clear_history().unwrap();
        assert!(storage.get_recent_history(10).unwrap().is_empty());
    }

    #[test]
    fn test_bookmarks_flow() {
        let storage = StorageEngine::new_in_memory().unwrap();
        let id = storage.add_bookmark("Google", "https://google.com", None).unwrap();
        assert!(id > 0);
        assert!(storage.is_bookmarked("https://google.com").unwrap());
        assert!(!storage.is_bookmarked("https://not-bookmarked.com").unwrap());

        let bookmarks = storage.get_bookmarks().unwrap();
        assert_eq!(bookmarks.len(), 1);
        assert_eq!(bookmarks[0].title, "Google");

        storage.delete_bookmark(bookmarks[0].id).unwrap();
        assert!(!storage.is_bookmarked("https://google.com").unwrap());
        assert!(storage.get_bookmarks().unwrap().is_empty());
    }

    #[test]
    fn test_site_shields_persistence() {
        let storage = StorageEngine::new_in_memory().unwrap();
        assert_eq!(storage.get_site_shield("example.com").unwrap(), None);

        storage.set_site_shield("example.com", false).unwrap();
        assert_eq!(storage.get_site_shield("example.com").unwrap(), Some(false));

        storage.set_site_shield("example.com", true).unwrap();
        assert_eq!(storage.get_site_shield("example.com").unwrap(), Some(true));
    }

    #[test]
    fn test_downloads_flow() {
        let storage = StorageEngine::new_in_memory().unwrap();
        let id = storage.record_download("sample.pdf", "https://example.com/sample.pdf", "/home/user/Downloads/sample.pdf").unwrap();
        assert!(id > 0);

        let list = storage.get_downloads().unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].filename, "sample.pdf");
        assert_eq!(list[0].status, "completed");

        storage.clear_downloads().unwrap();
        assert!(storage.get_downloads().unwrap().is_empty());
    }

    #[test]
    fn test_delete_bookmark_by_url_and_update_title() {
        let storage = StorageEngine::new_in_memory().unwrap();
        storage.add_bookmark("Google", "https://google.com", None).unwrap();
        assert!(storage.is_bookmarked("https://google.com").unwrap());

        let deleted = storage.delete_bookmark_by_url("https://google.com").unwrap();
        assert!(deleted);
        assert!(!storage.is_bookmarked("https://google.com").unwrap());

        storage.record_history("https://news.com", "Old Title").unwrap();
        storage.update_history_title("https://news.com", "New Updated Title").unwrap();
        let history = storage.get_recent_history(10).unwrap();
        assert_eq!(history[0].title, "New Updated Title");
    }

    #[test]
    fn test_bookmark_returning_id_on_conflict() {
        let storage = StorageEngine::new_in_memory().unwrap();
        let id1 = storage.add_bookmark("Google", "https://google.com", None).unwrap();
        let id2 = storage.add_bookmark("Google Updated", "https://google.com", None).unwrap();
        assert_eq!(id1, id2);
        let bms = storage.get_bookmarks().unwrap();
        assert_eq!(bms.len(), 1);
        assert_eq!(bms[0].title, "Google Updated");
    }

    #[test]
    fn test_record_history_batch() {
        let storage = StorageEngine::new_in_memory().unwrap();
        let batch = vec![
            ("https://site1.com", "Site 1", "2026-10-29T00:00:00Z", 2),
            ("https://site2.com", "Site 2", "2026-10-29T00:01:00Z", 5),
            ("valorex://settings", "Settings", "2026-10-29T00:02:00Z", 1),
        ];
        let inserted = storage.record_history_with_time_batch(&batch).unwrap();
        assert_eq!(inserted, 2);
        let recent = storage.get_recent_history(10).unwrap();
        assert_eq!(recent.len(), 2);
    }

    #[test]
    fn test_add_bookmarks_batch() {
        let storage = StorageEngine::new_in_memory().unwrap();
        let batch = vec![
            ("Site 1", "https://site1.com", Some("Folder 1")),
            ("Site 2", "https://site2.com", None),
        ];
        let inserted = storage.add_bookmarks_batch(&batch).unwrap();
        assert_eq!(inserted, 2);
        let bms = storage.get_bookmarks().unwrap();
        assert_eq!(bms.len(), 2);
    }

    #[test]
    fn test_search_history_wildcards() {
        let storage = StorageEngine::new_in_memory().unwrap();
        storage.record_history("https://100%real.com", "100% Real Site").unwrap();
        storage.record_history("https://other.com", "Other Site").unwrap();

        let res = storage.search_history("100%").unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].url, "https://100%real.com");
    }
}

