use crate::storage::StorageEngine;

pub struct InternalPages;

impl InternalPages {
    pub fn render_newtab(blocked_total: u64) -> String {
        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>New Tab - Valorex</title>
<style>
  :root {{
    --bg: #0f1012;
    --card-bg: #1a1b1f;
    --card-border: #282a30;
    --accent: #00d2ff;
    --text-primary: #f3f4f6;
    --text-secondary: #9ca3af;
    --font: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  }}
  * {{ box-sizing: border-box; margin: 0; padding: 0; }}
  body {{
    background: radial-gradient(circle at 50% 20%, #171920 0%, var(--bg) 80%);
    color: var(--text-primary);
    font-family: var(--font);
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 40px 20px;
  }}
  .logo {{
    font-size: 42px;
    font-weight: 800;
    letter-spacing: -1px;
    background: linear-gradient(135deg, #00d2ff 0%, #ff5500 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    margin-bottom: 8px;
  }}
  .tagline {{
    color: var(--text-secondary);
    font-size: 14px;
    margin-bottom: 30px;
  }}
  .stats-card {{
    background: var(--card-bg);
    border: 1px solid var(--card-border);
    border-radius: 16px;
    padding: 16px 32px;
    display: flex;
    gap: 40px;
    margin-bottom: 36px;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.4);
  }}
  .stat-item {{
    text-align: center;
  }}
  .stat-num {{
    font-size: 28px;
    font-weight: 700;
    color: var(--accent);
  }}
  .stat-label {{
    font-size: 12px;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-top: 4px;
  }}
  .search-box {{
    width: 100%;
    max-width: 580px;
    margin-bottom: 40px;
    position: relative;
  }}
  .search-input {{
    width: 100%;
    height: 50px;
    background: var(--card-bg);
    border: 1px solid var(--card-border);
    border-radius: 25px;
    padding: 0 24px;
    color: var(--text-primary);
    font-size: 15px;
    outline: none;
    transition: border-color 0.2s, box-shadow 0.2s;
  }}
  .search-input:focus {{
    border-color: var(--accent);
    box-shadow: 0 0 0 3px rgba(0, 210, 255, 0.2);
  }}
  .shortcuts-grid {{
    display: flex;
    flex-wrap: wrap;
    gap: 16px;
    justify-content: center;
    max-width: 600px;
  }}
  .shortcut {{
    width: 90px;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-decoration: none;
    color: var(--text-primary);
    padding: 12px 8px;
    border-radius: 12px;
    background: var(--card-bg);
    border: 1px solid transparent;
    transition: transform 0.15s, border-color 0.15s, background-color 0.15s;
  }}
  .shortcut:hover {{
    transform: translateY(-3px);
    border-color: var(--card-border);
    background: #22242a;
  }}
  .shortcut-icon {{
    width: 40px;
    height: 40px;
    border-radius: 10px;
    background: #2a2c34;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 20px;
    margin-bottom: 8px;
  }}
  .shortcut-title {{
    font-size: 12px;
    text-align: center;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    width: 100%;
  }}
</style>
</head>
<body>
  <div class="logo">VALOREX</div>
  <div class="tagline">The Fast, Private & Ad-Free Rust Browser</div>

  <div class="stats-card">
    <div class="stat-item">
      <div class="stat-num">{}</div>
      <div class="stat-label">Trackers Blocked</div>
    </div>
    <div class="stat-item">
      <div class="stat-num">100%</div>
      <div class="stat-label">Memory Safe</div>
    </div>
    <div class="stat-item">
      <div class="stat-num">0 ms</div>
      <div class="stat-label">Telemetry Latency</div>
    </div>
  </div>

  <div class="search-box">
    <form action="https://duckduckgo.com/" method="GET">
      <input type="text" name="q" class="search-input" placeholder="Search the web with DuckDuckGo..." autofocus>
    </form>
  </div>

  <div class="shortcuts-grid">
    <a href="https://duckduckgo.com" class="shortcut">
      <div class="shortcut-icon">🦆</div>
      <div class="shortcut-title">DuckDuckGo</div>
    </a>
    <a href="https://wikipedia.org" class="shortcut">
      <div class="shortcut-icon">📚</div>
      <div class="shortcut-title">Wikipedia</div>
    </a>
    <a href="https://rust-lang.org" class="shortcut">
      <div class="shortcut-icon">🦀</div>
      <div class="shortcut-title">Rust Lang</div>
    </a>
    <a href="https://github.com" class="shortcut">
      <div class="shortcut-icon">🐙</div>
      <div class="shortcut-title">GitHub</div>
    </a>
    <a href="https://news.ycombinator.com" class="shortcut">
      <div class="shortcut-icon">⚡</div>
      <div class="shortcut-title">Hacker News</div>
    </a>
  </div>
</body>
</html>
"#,
            blocked_total
        )
    }

    pub fn render_history(storage: &StorageEngine) -> String {
        let entries = storage.get_recent_history(100).unwrap_or_default();
        let mut list_html = String::new();
        for item in entries {
            list_html.push_str(&format!(
                r#"<div class="history-item">
                    <div class="history-time">{}</div>
                    <a href="{}" class="history-title">{}</a>
                    <div class="history-url">{}</div>
                </div>"#,
                item.visit_time.chars().take(10).collect::<String>(),
                item.url,
                if item.title.is_empty() { &item.url } else { &item.title },
                item.url
            ));
        }

        if list_html.is_empty() {
            list_html = r#"<div class="empty">No browsing history recorded yet.</div>"#.to_string();
        }

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Browsing History - Valorex</title>
<style>
  :root {{
    --bg: #121316;
    --card: #1c1d22;
    --border: #2b2d35;
    --accent: #00d2ff;
    --text: #f0f2f5;
    --secondary: #9aa0ad;
    --font: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  }}
  * {{ box-sizing: border-box; margin: 0; padding: 0; }}
  body {{
    background: var(--bg);
    color: var(--text);
    font-family: var(--font);
    padding: 40px;
    max-width: 900px;
    margin: 0 auto;
  }}
  .header {{
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
    padding-bottom: 16px;
    border-bottom: 1px solid var(--border);
  }}
  h1 {{ font-size: 26px; }}
  .clear-btn {{
    background: #ef4444;
    color: white;
    border: none;
    padding: 8px 16px;
    border-radius: 8px;
    cursor: pointer;
    font-weight: 600;
    transition: opacity 0.2s;
  }}
  .clear-btn:hover {{ opacity: 0.85; }}
  .history-list {{
    display: flex;
    flex-direction: column;
    gap: 8px;
  }}
  .history-item {{
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 12px 16px;
    display: flex;
    align-items: center;
    gap: 16px;
  }}
  .history-time {{
    font-size: 12px;
    color: var(--secondary);
    min-width: 80px;
  }}
  .history-title {{
    color: var(--text);
    font-weight: 500;
    text-decoration: none;
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }}
  .history-title:hover {{
    color: var(--accent);
    text-decoration: underline;
  }}
  .history-url {{
    font-size: 12px;
    color: var(--secondary);
    max-width: 250px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }}
  .empty {{
    text-align: center;
    padding: 60px 0;
    color: var(--secondary);
    font-size: 16px;
  }}
</style>
</head>
<body>
  <div class="header">
    <h1>Browsing History</h1>
    <button class="clear-btn" onclick="clearAllHistory()">Clear Browsing Data</button>
  </div>
  <div class="history-list">
    {}
  </div>
  <script>
    function clearAllHistory() {{
      if (confirm('Are you sure you want to clear all history?')) {{
        if (window.ipc) {{
          window.ipc.postMessage(JSON.stringify({{ action: 'clear_history' }}));
          location.reload();
        }}
      }}
    }}
  </script>
</body>
</html>
"#,
            list_html
        )
    }

    pub fn render_bookmarks(storage: &StorageEngine) -> String {
        let entries = storage.get_bookmarks().unwrap_or_default();
        let mut list_html = String::new();
        for item in entries {
            list_html.push_str(&format!(
                r#"<div class="bookmark-item">
                    <span class="bookmark-icon">⭐</span>
                    <a href="{}" class="bookmark-title">{}</a>
                    <span class="bookmark-url">{}</span>
                    <button class="del-btn" onclick="deleteBookmark({})">Remove</button>
                </div>"#,
                item.url, item.title, item.url, item.id
            ));
        }

        if list_html.is_empty() {
            list_html = r#"<div class="empty">No bookmarks saved yet. Click the star icon in the address bar to bookmark any page!</div>"#.to_string();
        }

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Bookmarks - Valorex</title>
<style>
  :root {{
    --bg: #121316;
    --card: #1c1d22;
    --border: #2b2d35;
    --accent: #fbbf24;
    --text: #f0f2f5;
    --secondary: #9aa0ad;
    --font: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  }}
  * {{ box-sizing: border-box; margin: 0; padding: 0; }}
  body {{
    background: var(--bg);
    color: var(--text);
    font-family: var(--font);
    padding: 40px;
    max-width: 900px;
    margin: 0 auto;
  }}
  h1 {{ font-size: 26px; margin-bottom: 24px; padding-bottom: 16px; border-bottom: 1px solid var(--border); }}
  .bookmarks-list {{ display: flex; flex-direction: column; gap: 8px; }}
  .bookmark-item {{
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 12px 16px;
    display: flex;
    align-items: center;
    gap: 14px;
  }}
  .bookmark-icon {{ color: var(--accent); }}
  .bookmark-title {{
    color: var(--text);
    font-weight: 500;
    text-decoration: none;
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }}
  .bookmark-title:hover {{ text-decoration: underline; }}
  .bookmark-url {{
    font-size: 12px;
    color: var(--secondary);
    max-width: 250px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }}
  .del-btn {{
    background: transparent;
    border: 1px solid var(--border);
    color: #ef4444;
    padding: 4px 10px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 11px;
  }}
  .del-btn:hover {{ background: rgba(239, 68, 68, 0.1); }}
  .empty {{ text-align: center; padding: 60px 0; color: var(--secondary); font-size: 16px; }}
</style>
</head>
<body>
  <h1>Saved Bookmarks</h1>
  <div class="bookmarks-list">
    {}
  </div>
  <script>
    function deleteBookmark(id) {{
      if (window.ipc) {{
        window.ipc.postMessage(JSON.stringify({{ action: 'delete_bookmark', id: id }}));
        location.reload();
      }}
    }}
  </script>
</body>
</html>
"#,
            list_html
        )
    }

    pub fn render_settings() -> String {
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Settings - Valorex</title>
<style>
  :root {
    --bg: #121316;
    --card: #1c1d22;
    --border: #2b2d35;
    --accent: #00d2ff;
    --text: #f0f2f5;
    --secondary: #9aa0ad;
    --font: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  }
  * { box-sizing: border-box; margin: 0; padding: 0; }
  body {
    background: var(--bg);
    color: var(--text);
    font-family: var(--font);
    padding: 40px;
    max-width: 800px;
    margin: 0 auto;
  }
  h1 { font-size: 26px; margin-bottom: 24px; padding-bottom: 16px; border-bottom: 1px solid var(--border); }
  .section {
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 20px;
    margin-bottom: 20px;
  }
  .section-title { font-size: 16px; font-weight: 600; margin-bottom: 12px; }
  .setting-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 0;
    border-bottom: 1px solid rgba(255,255,255,0.05);
  }
  .setting-row:last-child { border-bottom: none; }
  .setting-desc { font-size: 12px; color: var(--secondary); margin-top: 4px; }
  .badge {
    background: rgba(0, 210, 255, 0.15);
    color: var(--accent);
    padding: 4px 8px;
    border-radius: 6px;
    font-size: 11px;
    font-weight: 600;
  }
</style>
</head>
<body>
  <h1>Valorex Settings</h1>

  <div class="section">
    <div class="section-title">Shields & Privacy Protection</div>
    <div class="setting-row">
      <div>
        <div>Global Ad & Tracker Blocking</div>
        <div class="setting-desc">Blocks third-party trackers, beacons, and invasive advertising networks.</div>
      </div>
      <span class="badge">ACTIVE</span>
    </div>
    <div class="setting-row">
      <div>
        <div>Cosmetic Element Hiding</div>
        <div class="setting-desc">Removes empty placeholders left by blocked advertisements.</div>
      </div>
      <span class="badge">ACTIVE</span>
    </div>
  </div>

  <div class="section">
    <div class="section-title">Search Engine</div>
    <div class="setting-row">
      <div>
        <div>Default Search Provider</div>
        <div class="setting-desc">Search query engine used in the address bar.</div>
      </div>
      <span class="badge">DuckDuckGo (Privacy Default)</span>
    </div>
  </div>

  <div class="section">
    <div class="section-title">Browser Engine Information</div>
    <div class="setting-row">
      <div>
        <div>Engine Architecture</div>
        <div class="setting-desc">Built with Rust, Wry (WebKitGTK 4.1), and Brave Adblock Engine.</div>
      </div>
      <span class="badge">v0.1.0</span>
    </div>
  </div>
</body>
</html>
"#.to_string()
    }

    pub fn render_downloads(storage: &StorageEngine) -> String {
        let entries = storage.get_downloads().unwrap_or_default();
        let mut list_html = String::new();
        for item in entries {
            list_html.push_str(&format!(
                r#"<div class="download-item">
                    <span class="file-icon">📥</span>
                    <div class="file-info">
                      <div class="filename">{}</div>
                      <div class="filepath">{}</div>
                    </div>
                    <span class="badge">COMPLETED</span>
                </div>"#,
                item.filename, item.path
            ));
        }

        if list_html.is_empty() {
            list_html = r#"<div class="empty">No downloaded files recorded yet.</div>"#.to_string();
        }

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Downloads - Valorex</title>
<style>
  :root {{
    --bg: #121316;
    --card: #1c1d22;
    --border: #2b2d35;
    --accent: #00d2ff;
    --text: #f0f2f5;
    --secondary: #9aa0ad;
    --font: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  }}
  * {{ box-sizing: border-box; margin: 0; padding: 0; }}
  body {{
    background: var(--bg);
    color: var(--text);
    font-family: var(--font);
    padding: 40px;
    max-width: 900px;
    margin: 0 auto;
  }}
  h1 {{ font-size: 26px; margin-bottom: 24px; padding-bottom: 16px; border-bottom: 1px solid var(--border); }}
  .downloads-list {{ display: flex; flex-direction: column; gap: 8px; }}
  .download-item {{
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 14px 18px;
    display: flex;
    align-items: center;
    gap: 16px;
  }}
  .file-icon {{ font-size: 24px; }}
  .file-info {{ flex: 1; min-width: 0; }}
  .filename {{ font-weight: 600; color: var(--text); }}
  .filepath {{ font-size: 12px; color: var(--secondary); margin-top: 3px; word-break: break-all; }}
  .badge {{
    background: rgba(16, 185, 129, 0.15);
    color: #10b981;
    padding: 4px 10px;
    border-radius: 6px;
    font-size: 11px;
    font-weight: 700;
  }}
  .empty {{ text-align: center; padding: 60px 0; color: var(--secondary); font-size: 16px; }}
</style>
</head>
<body>
  <h1>Downloads</h1>
  <div class="downloads-list">
    {}
  </div>
</body>
</html>
"#,
            list_html
        )
    }
}
