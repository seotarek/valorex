use crate::i18n::Language;
use crate::storage::StorageEngine;

pub fn escape_html(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(c),
        }
    }
    escaped
}

pub fn sanitize_href(url: &str) -> String {
    let lower = url.trim().to_lowercase();
    if lower.starts_with("http://") || lower.starts_with("https://") || lower.starts_with("valorex://") {
        escape_html(url)
    } else {
        "#".to_string()
    }
}

pub struct InternalPages;

impl InternalPages {
    pub fn render_newtab(blocked_total: u64, lang: Language, token: &str) -> String {
        let bw_saved = ((blocked_total as f64) * 0.08).round() as u64;
        let time_saved = ((blocked_total as f64) * 0.04).round() as u64;

        let is_ar = lang.is_rtl();
        let dir = lang.direction();
        let lang_code = lang.code();

        let (greetings_title, search_placeholder, ads_label, bw_label, time_label, migration_title, migration_desc, import_chrome_btn, import_brave_btn, _shortcuts_heading) = if is_ar {
            (
                "أهلاً بك في Valorex",
                "ابحث باستخدام Google أو اسأل Gemini الذكي...",
                "عنصر ضار محجوب",
                "ميجابايت تم توفيرها",
                "ثانية تم توفيرها",
                "استيراد الإشارات وسجل التصفح بنقرة واحدة",
                "انقل بياناتك ومفضلاتك من متصفحك السابق مباشرة إلى Valorex.",
                "استيراد Chrome",
                "استيراد Brave",
                "المواقع الأكثر زيارة"
            )
        } else {
            (
                "Welcome to Valorex",
                "Search with Google or ask Gemini AI...",
                "Trackers & Ads Blocked",
                "Bandwidth Saved",
                "Estimated Time Saved",
                "One-Click Browser Data Migration",
                "Import your bookmarks and history from your previous browser into Valorex.",
                "Import Chrome",
                "Import Brave",
                "Frequently Visited"
            )
        };

        format!(
            r###"<!DOCTYPE html>
<html lang="{lang_code}" dir="{dir}">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Valorex 2027</title>
<style>
  :root {{
    --bg-deep: #0a0b0e;
    --card-surface: rgba(22, 24, 30, 0.78);
    --card-border: rgba(255, 255, 255, 0.08);
    --card-glow: rgba(0, 210, 255, 0.15);
    --accent-cyan: #00d2ff;
    --accent-orange: #ff5500;
    --accent-gemini: #a855f7;
    --text-primary: #f8fafc;
    --text-secondary: #94a3b8;
    --font: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Inter", "Cairo", "Tajawal", sans-serif;
  }}

  * {{ box-sizing: border-box; margin: 0; padding: 0; }}

  body {{
    background-color: var(--bg-deep);
    background-image: 
      radial-gradient(at 0% 0%, rgba(0, 210, 255, 0.09) 0px, transparent 50%),
      radial-gradient(at 100% 100%, rgba(255, 85, 0, 0.07) 0px, transparent 50%),
      radial-gradient(at 50% 50%, rgba(15, 17, 23, 0.95) 0px, var(--bg-deep) 100%);
    color: var(--text-primary);
    font-family: var(--font);
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start;
    padding: 30px 20px 40px;
    overflow-x: hidden;
  }}

  /* Top Hero Clock & Header */
  .hero-header {{
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-top: 15px;
    margin-bottom: 25px;
    text-align: center;
  }}

  .clock {{
    font-size: 54px;
    font-weight: 700;
    letter-spacing: -1.5px;
    color: #ffffff;
    text-shadow: 0 4px 20px rgba(0, 0, 0, 0.6);
  }}

  .date-string {{
    font-size: 13px;
    color: var(--text-secondary);
    font-weight: 500;
    letter-spacing: 0.5px;
    margin-top: 2px;
  }}

  .greeting {{
    font-size: 15px;
    color: var(--accent-cyan);
    font-weight: 600;
    margin-top: 6px;
    display: flex;
    align-items: center;
    gap: 8px;
  }}

  /* Shields Live Telemetry Strip */
  .telemetry-strip {{
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 14px;
    width: 100%;
    max-width: 660px;
    margin-bottom: 28px;
  }}

  .stat-card {{
    background: var(--card-surface);
    backdrop-filter: blur(16px);
    -webkit-backdrop-filter: blur(16px);
    border: 1px solid var(--card-border);
    border-radius: 16px;
    padding: 14px 18px;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
    transition: transform 0.2s ease, border-color 0.2s ease;
  }}

  .stat-card:hover {{
    transform: translateY(-2px);
    border-color: rgba(0, 210, 255, 0.3);
  }}

  .stat-header {{
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 11px;
    color: var(--text-secondary);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.6px;
  }}

  .stat-value {{
    font-size: 24px;
    font-weight: 700;
    margin-top: 6px;
    color: #ffffff;
  }}

  .stat-value.cyan {{ color: var(--accent-cyan); text-shadow: 0 0 16px rgba(0, 210, 255, 0.4); }}
  .stat-value.orange {{ color: var(--accent-orange); text-shadow: 0 0 16px rgba(255, 85, 0, 0.4); }}
  .stat-value.purple {{ color: var(--accent-gemini); text-shadow: 0 0 16px rgba(168, 85, 247, 0.4); }}

  /* Universal Search Box */
  .search-container {{
    width: 100%;
    max-width: 660px;
    margin-bottom: 26px;
    position: relative;
  }}

  .search-bar {{
    width: 100%;
    background: rgba(24, 26, 34, 0.85);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    border: 1px solid var(--card-border);
    border-radius: 28px;
    padding: 6px 14px 6px 20px;
    display: flex;
    align-items: center;
    gap: 12px;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.5);
    transition: border-color 0.25s, box-shadow 0.25s;
  }}

  .search-bar:focus-within {{
    border-color: var(--accent-cyan);
    box-shadow: 0 0 0 3px rgba(0, 210, 255, 0.2), 0 12px 35px rgba(0, 0, 0, 0.6);
  }}

  .search-icon {{
    color: var(--text-secondary);
    display: flex;
    align-items: center;
    flex-shrink: 0;
  }}

  .search-input {{
    flex: 1;
    background: transparent;
    border: none;
    outline: none;
    color: var(--text-primary);
    font-size: 15px;
    font-family: inherit;
  }}

  .search-input::placeholder {{
    color: #64748b;
  }}

  .ai-mode-toggle {{
    background: rgba(168, 85, 247, 0.15);
    border: 1px solid rgba(168, 85, 247, 0.35);
    color: #d8b4fe;
    font-size: 12px;
    font-weight: 600;
    padding: 6px 14px;
    border-radius: 20px;
    display: flex;
    align-items: center;
    gap: 6px;
    cursor: pointer;
    transition: background 0.2s, transform 0.15s;
    user-select: none;
  }}

  .ai-mode-toggle:hover {{
    background: rgba(168, 85, 247, 0.28);
    transform: scale(1.03);
  }}

  .ai-mode-toggle.active {{
    background: linear-gradient(135deg, #a855f7, #6366f1);
    color: #ffffff;
    border-color: transparent;
    box-shadow: 0 0 14px rgba(168, 85, 247, 0.4);
  }}

  /* One-Click Browser Data Migration Card */
  .migration-banner {{
    width: 100%;
    max-width: 660px;
    background: linear-gradient(135deg, rgba(28, 30, 38, 0.85), rgba(18, 20, 26, 0.95));
    border: 1px solid rgba(0, 210, 255, 0.2);
    border-radius: 16px;
    padding: 16px 20px;
    margin-bottom: 26px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.35);
  }}

  .migration-info {{
    display: flex;
    flex-direction: column;
    gap: 3px;
  }}

  .migration-title {{
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
    display: flex;
    align-items: center;
    gap: 8px;
  }}

  .migration-desc {{
    font-size: 11px;
    color: var(--text-secondary);
  }}

  .migration-actions {{
    display: flex;
    gap: 8px;
    flex-shrink: 0;
  }}

  .migrate-btn {{
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.14);
    color: var(--text-primary);
    padding: 6px 12px;
    border-radius: 8px;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.15s, border-color 0.15s, transform 0.1s;
    display: flex;
    align-items: center;
    gap: 6px;
  }}

  .migrate-btn:hover {{
    background: rgba(0, 210, 255, 0.18);
    border-color: var(--accent-cyan);
    transform: translateY(-1px);
  }}

  .migrate-btn svg {{
    width: 13px;
    height: 13px;
    fill: currentColor;
  }}

  /* Curated 2027 Bookmarks & Shortcuts Grid */
  .shortcuts-grid {{
    width: 100%;
    max-width: 660px;
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 14px;
  }}

  .shortcut-card {{
    background: var(--card-surface);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    border: 1px solid var(--card-border);
    border-radius: 14px;
    padding: 16px 12px;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-decoration: none;
    color: var(--text-primary);
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
  }}

  .shortcut-card:hover {{
    transform: translateY(-3px);
    background: rgba(30, 34, 44, 0.9);
    border-color: rgba(255, 255, 255, 0.2);
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.4);
  }}

  .shortcut-icon-wrap {{
    width: 44px;
    height: 44px;
    border-radius: 12px;
    background: rgba(255, 255, 255, 0.05);
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 10px;
    transition: transform 0.2s;
  }}

  .shortcut-card:hover .shortcut-icon-wrap {{
    transform: scale(1.08);
  }}

  .shortcut-title {{
    font-size: 12px;
    font-weight: 500;
    color: var(--text-primary);
  }}

  /* Status Notification Toast */
  .status-toast {{
    position: fixed;
    bottom: 24px;
    left: 50%;
    transform: translateX(-50%);
    background: rgba(20, 22, 28, 0.95);
    border: 1px solid var(--accent-cyan);
    color: #ffffff;
    padding: 10px 22px;
    border-radius: 20px;
    font-size: 13px;
    font-weight: 500;
    display: none;
    align-items: center;
    gap: 10px;
    box-shadow: 0 12px 30px rgba(0, 0, 0, 0.6);
    z-index: 1000;
  }}
</style>
</head>
<body>

  <!-- Top Hero Clock & Header -->
  <div class="hero-header">
    <div class="clock" id="live-clock">--:--</div>
    <div class="date-string" id="live-date">...</div>
    <div class="greeting">
      <span>{greetings_title}</span>
    </div>
  </div>

  <!-- Shields Live Telemetry Strip -->
  <div class="telemetry-strip">
    <div class="stat-card">
      <div class="stat-header">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor" color="#00d2ff"><path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm-2 16l-4-4 1.41-1.41L10 14.17l6.59-6.59L18 9l-8 8z"/></svg>
        <span>{ads_label}</span>
      </div>
      <div class="stat-value cyan">{blocked_total}</div>
    </div>
    <div class="stat-card">
      <div class="stat-header">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor" color="#ff5500"><path d="M19.35 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.6 5.64 5.35 8.04 2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96z"/></svg>
        <span>{bw_label}</span>
      </div>
      <div class="stat-value orange">{bw_saved} MB</div>
    </div>
    <div class="stat-card">
      <div class="stat-header">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor" color="#a855f7"><path d="M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm.5-13H11v6l5.25 3.15.75-1.23-4.5-2.67z"/></svg>
        <span>{time_label}</span>
      </div>
      <div class="stat-value purple">{time_saved}s</div>
    </div>
  </div>

  <!-- Universal Omnibox Search -->
  <div class="search-container">
    <form id="search-form" action="https://www.google.com/search" method="GET">
      <div class="search-bar">
        <div class="search-icon">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
        </div>
        <input type="text" name="q" class="search-input" id="search-input" placeholder="{search_placeholder}" autocomplete="off" spellcheck="false" autofocus>
        <div class="ai-mode-toggle" id="gemini-toggle" title="Toggle Gemini AI Copilot">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2l2.4 7.6L22 12l-7.6 2.4L12 22l-2.4-7.6L2 12l7.6-2.4z"/></svg>
          <span>Gemini AI</span>
        </div>
      </div>
    </form>
  </div>

  <!-- Browser Data Migration Banner -->
  <div class="migration-banner">
    <div class="migration-info">
      <div class="migration-title">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
        <span>{migration_title}</span>
      </div>
      <div class="migration-desc">{migration_desc}</div>
    </div>
    <div class="migration-actions">
      <button class="migrate-btn" onclick="triggerImport('google-chrome')">
        <span>{import_chrome_btn}</span>
      </button>
      <button class="migrate-btn" onclick="triggerImport('brave')">
        <span>{import_brave_btn}</span>
      </button>
    </div>
  </div>

  <!-- Curated Shortcuts Grid -->
  <div class="shortcuts-grid">
    <a href="https://google.com" class="shortcut-card">
      <div class="shortcut-icon-wrap">
        <svg width="22" height="22" viewBox="0 0 24 24" fill="currentColor" color="#4285F4"><path d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"/><path d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"/><path d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.06H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.94l2.85-2.22.81-.63z"/><path d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.06l3.66 2.84c.87-2.6 3.3-4.52 6.16-4.52z"/></svg>
      </div>
      <div class="shortcut-title">Google</div>
    </a>
    <a href="https://gemini.google.com" class="shortcut-card">
      <div class="shortcut-icon-wrap">
        <svg width="22" height="22" viewBox="0 0 24 24" fill="currentColor" color="#A855F7"><path d="M12 2l2.4 7.6L22 12l-7.6 2.4L12 22l-2.4-7.6L2 12l7.6-2.4z"/></svg>
      </div>
      <div class="shortcut-title">Gemini AI</div>
    </a>
    <a href="https://youtube.com" class="shortcut-card">
      <div class="shortcut-icon-wrap">
        <svg width="22" height="22" viewBox="0 0 24 24" fill="currentColor" color="#FF0000"><path d="M23.498 6.186a3.016 3.016 0 0 0-2.122-2.136C19.505 3.545 12 3.545 12 3.545s-7.505 0-9.377.505A3.017 3.017 0 0 0 .502 6.186C0 8.07 0 12 0 12s0 3.93.502 5.814a3.016 3.016 0 0 0 2.122 2.136c1.871.505 9.376.505 9.376.505s7.505 0 9.377-.505a3.015 3.015 0 0 0 2.122-2.136C24 15.93 24 12 24 12s0-3.93-.502-5.814zM9.545 15.568V8.432L15.818 12l-6.273 3.568z"/></svg>
      </div>
      <div class="shortcut-title">YouTube</div>
    </a>
    <a href="https://x.com" class="shortcut-card">
      <div class="shortcut-icon-wrap">
        <svg width="22" height="22" viewBox="0 0 24 24" fill="currentColor" color="#F0F2F5"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/></svg>
      </div>
      <div class="shortcut-title">X (Twitter)</div>
    </a>
    <a href="https://github.com" class="shortcut-card">
      <div class="shortcut-icon-wrap">
        <svg width="22" height="22" viewBox="0 0 24 24" fill="currentColor" color="#F0F2F5"><path d="M12 2C6.48 2 2 6.48 2 12c0 4.42 2.87 8.17 6.84 9.5.5.08.66-.23.66-.5v-1.69c-2.77.6-3.36-1.34-3.36-1.34-.46-1.16-1.11-1.47-1.11-1.47-.91-.62.07-.6.07-.6 1 .07 1.53 1.03 1.53 1.03.87 1.52 2.34 1.07 2.91.83.1-.65.35-1.09.63-1.34-2.22-.25-4.55-1.11-4.55-4.92 0-1.11.38-2 1.03-2.71-.1-.25-.45-1.29.1-2.64 0 0 .84-.27 2.75 1.02.79-.22 1.65-.33 2.5-.33.85 0 1.71.11 2.5.33 1.91-1.29 2.75-1.02 2.75-1.02.55 1.35.2 2.39.1 2.64.65.71 1.03 1.6 1.03 2.71 0 3.82-2.34 4.66-4.57 4.91.36.31.69.92.69 1.85V21c0 .27.16.59.67.5C19.14 20.16 22 16.42 22 12A10 10 0 0 0 12 2z"/></svg>
      </div>
      <div class="shortcut-title">GitHub</div>
    </a>
    <a href="https://reddit.com" class="shortcut-card">
      <div class="shortcut-icon-wrap">
        <svg width="22" height="22" viewBox="0 0 24 24" fill="currentColor" color="#FF4500"><path d="M12 2a10 10 0 1 0 10 10A10 10 0 0 0 12 2zm5.01 10.74a1.76 1.76 0 0 1 .1.59c0 2.21-2.28 4-5.11 4s-5.11-1.79-5.11-4c0-.2.04-.4.1-.59a1.69 1.69 0 0 1-.22-.84c0-.95.77-1.72 1.72-1.72.56 0 1.05.27 1.36.68 1.03-.68 2.38-1.11 3.86-1.16l.66-3.09 2.15.46a1.23 1.23 0 1 1 2.4.45c0 .68-.55 1.23-1.23 1.23-.55 0-1.01-.36-1.17-.86l-1.89-.4-.56 2.64c1.45.06 2.79.48 3.81 1.15.31-.41.8-.68 1.36-.68.95 0 1.72.77 1.72 1.72 0 .31-.08.6-.22.84z"/></svg>
      </div>
      <div class="shortcut-title">Reddit</div>
    </a>
    <a href="https://wikipedia.org" class="shortcut-card">
      <div class="shortcut-icon-wrap">
        <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/><path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/></svg>
      </div>
      <div class="shortcut-title">Wikipedia</div>
    </a>
    <a href="https://news.ycombinator.com" class="shortcut-card">
      <div class="shortcut-icon-wrap">
        <svg width="22" height="22" viewBox="0 0 24 24" fill="currentColor" color="#FF6600"><polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"/></svg>
      </div>
      <div class="shortcut-title">Hacker News</div>
    </a>
  </div>

  <!-- Status Notification Toast -->
  <div class="status-toast" id="toast">
    <span id="toast-text">Importing data...</span>
  </div>

  <script>
    // Bulletproof Multi-Layer IPC Bridge
    function sendIpc(action, data) {{
      const payload = JSON.stringify({{ action: action, token: "{token}", ...(data || {{}}) }});
      if (window.ipc && typeof window.ipc.postMessage === 'function') {{
        window.ipc.postMessage(payload);
        return;
      }}
      if (window.webkit && window.webkit.messageHandlers && window.webkit.messageHandlers.ipc) {{
        window.webkit.messageHandlers.ipc.postMessage(payload);
        return;
      }}
      // Fallback via location cancellation
      window.location.href = 'valorex-action://' + encodeURIComponent(payload);
    }}

    // Live Clock & Greeting
    function updateClock() {{
      const now = new Date();
      const hours = String(now.getHours()).padStart(2, '0');
      const mins = String(now.getMinutes()).padStart(2, '0');
      document.getElementById('live-clock').textContent = hours + ':' + mins;

      const locale = '{lang_code}' === 'ar' ? 'ar-EG' : 'en-US';
      const opts = {{ weekday: 'long', month: 'short', day: 'numeric' }};
      document.getElementById('live-date').textContent = now.toLocaleDateString(locale, opts);
    }}
    setInterval(updateClock, 1000);
    updateClock();

    let geminiMode = false;
    const toggle = document.getElementById('gemini-toggle');
    const input = document.getElementById('search-input');
    const form = document.getElementById('search-form');

    toggle.onclick = () => {{
      geminiMode = !geminiMode;
      toggle.classList.toggle('active', geminiMode);
      if (geminiMode) {{
        input.placeholder = '{lang_code}' === 'ar' ? 'اسأل Gemini AI عن أي شيء...' : 'Ask Gemini AI anything...';
      }} else {{
        input.placeholder = '{search_placeholder}';
      }}
      input.focus();
    }};

    form.onsubmit = (e) => {{
      const query = input.value.trim();
      if (!query) return false;
      if (geminiMode) {{
        e.preventDefault();
        const geminiUrl = 'https://gemini.google.com/app?q=' + encodeURIComponent(query);
        window.location.href = geminiUrl;
        return false;
      }}
      return true;
    }};

    // Browser Migration Trigger
    const browserNames = {{
      'google-chrome': 'Google Chrome',
      'brave': 'Brave',
      'firefox': 'Firefox',
      'microsoft-edge': 'Microsoft Edge',
      'chromium': 'Chromium'
    }};

    function triggerImport(browser) {{
      const displayName = browserNames[browser] || browser;
      const msg = '{lang_code}' === 'ar' ? 'جاري استيراد البيانات من ' + displayName + '...' : 'Importing data from ' + displayName + '...';
      showToast(msg);
      sendIpc('import_data', {{ target: browser }});
    }}

    window.valorexOnImportFinished = function(summary) {{
      const displayName = browserNames[summary.browser_name] || summary.browser_name;
      if (summary.success) {{
        const msg = '{lang_code}' === 'ar' 
          ? 'تم بنجاح استيراد ' + summary.bookmarks_imported + ' مفضلة و ' + summary.history_imported + ' رابط من ' + displayName
          : 'Successfully imported ' + summary.bookmarks_imported + ' bookmarks and ' + summary.history_imported + ' history entries from ' + displayName;
        showToast(msg);
      }} else {{
        const msg = '{lang_code}' === 'ar'
          ? 'تعذر استيراد البيانات من ' + displayName + ': ' + (summary.message || 'لم يتم العثور على بيانات')
          : 'Failed to import from ' + displayName + ': ' + (summary.message || 'Profile not found');
        showToast(msg);
      }}
    }};

    function showToast(text) {{
      const toast = document.getElementById('toast');
      const toastText = document.getElementById('toast-text');
      toastText.textContent = text;
      toast.style.display = 'flex';
      setTimeout(() => {{
        toast.style.display = 'none';
      }}, 5000);
    }}
  </script>
</body>
</html>
"###
        )
    }

    pub fn render_history(storage: &StorageEngine, lang: Language, token: &str) -> String {
        let entries = storage.get_recent_history(500).unwrap_or_default();
        let is_ar = lang.is_rtl();
        let dir = lang.direction();
        let lang_code = lang.code();

        let (page_title, clear_btn_label, empty_label, confirm_msg) = if is_ar {
            ("سجل التصفح", "مسح سجل التصفح بالكامل", "سجل التصفح فارغ حالياً.", "هل أنت متأكد من مسح جميع بيانات التصفح؟")
        } else {
            ("Browsing History", "Clear Browsing Data", "No browsing history recorded yet.", "Are you sure you want to clear all browsing history?")
        };

        let mut list_html = String::new();
        for item in entries {
            let display_time = item.visit_time.split('T').next().unwrap_or(&item.visit_time);
            let safe_time = escape_html(display_time);
            let safe_url = escape_html(&item.url);
            let safe_href = sanitize_href(&item.url);
            let safe_title = escape_html(&item.title);
            list_html.push_str(&format!(
                r#"<div class="history-item">
                    <span class="history-time">{}</span>
                    <a href="{}" class="history-title">{}</a>
                    <span class="history-url">{}</span>
                </div>"#,
                safe_time, safe_href, safe_title, safe_url
            ));
        }

        if list_html.is_empty() {
            list_html = format!(r#"<div class="empty">{}</div>"#, empty_label);
        }

        format!(
            r###"<!DOCTYPE html>
<html lang="{lang_code}" dir="{dir}">
<head>
<meta charset="utf-8">
<title>{page_title} - Valorex</title>
<style>
  :root {{
    --bg: #121316;
    --card: #1c1d22;
    --border: #2b2d35;
    --accent: #00d2ff;
    --text: #f0f2f5;
    --secondary: #9aa0ad;
    --font: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Cairo", "Tajawal", sans-serif;
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
    border: none;
    color: white;
    padding: 8px 16px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 13px;
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
    direction: ltr;
    unicode-bidi: isolate;
    text-align: left;
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
    <h1>{page_title}</h1>
    <button class="clear-btn" onclick="clearAllHistory()">{clear_btn_label}</button>
  </div>
  <div class="history-list">
    {list_html}
  </div>
  <script>
    function sendIpc(action, data) {{
      const payload = JSON.stringify({{ action: action, token: "{token}", ...(data || {{}}) }});
      if (window.ipc && typeof window.ipc.postMessage === 'function') {{
        window.ipc.postMessage(payload);
        return;
      }}
      if (window.webkit && window.webkit.messageHandlers && window.webkit.messageHandlers.ipc) {{
        window.webkit.messageHandlers.ipc.postMessage(payload);
        return;
      }}
      window.location.href = 'valorex-action://' + encodeURIComponent(payload);
    }}

    function clearAllHistory() {{
      if (confirm('{confirm_msg}')) {{
        sendIpc('clear_history');
        setTimeout(() => location.reload(), 300);
      }}
    }}
  </script>
</body>
</html>
"###
        )
    }

    pub fn render_bookmarks(storage: &StorageEngine, lang: Language, token: &str) -> String {
        let entries = storage.get_bookmarks().unwrap_or_default();
        let is_ar = lang.is_rtl();
        let dir = lang.direction();
        let lang_code = lang.code();

        let (page_title, remove_label, empty_label) = if is_ar {
            ("الإشارات المرجعية والمفضلة", "حذف", "لا توجد مفضلات محفوظة بعد. اضغط على أيقونة النجمة في شريط العناوين لحفظ أي صفحة!")
        } else {
            ("Bookmarks", "Remove", "No bookmarks saved yet. Click the star icon in the address bar to bookmark any page!")
        };

        let mut list_html = String::new();
        for item in entries {
            let safe_url = escape_html(&item.url);
            let safe_href = sanitize_href(&item.url);
            let safe_title = escape_html(&item.title);
            list_html.push_str(&format!(
                r##"<div class="bookmark-item">
                    <span class="bookmark-icon"><svg width="14" height="14" viewBox="0 0 24 24" fill="#fbbf24"><path d="M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z"/></svg></span>
                    <a href="{}" class="bookmark-title">{}</a>
                    <span class="bookmark-url">{}</span>
                    <button class="del-btn" onclick="deleteBookmark({})">{}</button>
                </div>"##,
                safe_href, safe_title, safe_url, item.id, remove_label
            ));
        }

        if list_html.is_empty() {
            list_html = format!(r#"<div class="empty">{}</div>"#, empty_label);
        }

        format!(
            r###"<!DOCTYPE html>
<html lang="{lang_code}" dir="{dir}">
<head>
<meta charset="utf-8">
<title>{page_title} - Valorex</title>
<style>
  :root {{
    --bg: #121316;
    --card: #1c1d22;
    --border: #2b2d35;
    --accent: #fbbf24;
    --text: #f0f2f5;
    --secondary: #9aa0ad;
    --font: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Cairo", "Tajawal", sans-serif;
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
    direction: ltr;
    unicode-bidi: isolate;
    text-align: left;
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
  <h1>{page_title}</h1>
  <div class="bookmarks-list">
    {list_html}
  </div>
  <script>
    function sendIpc(action, data) {{
      const payload = JSON.stringify({{ action: action, token: "{token}", ...(data || {{}}) }});
      if (window.ipc && typeof window.ipc.postMessage === 'function') {{
        window.ipc.postMessage(payload);
        return;
      }}
      if (window.webkit && window.webkit.messageHandlers && window.webkit.messageHandlers.ipc) {{
        window.webkit.messageHandlers.ipc.postMessage(payload);
        return;
      }}
      window.location.href = 'valorex-action://' + encodeURIComponent(payload);
    }}

    function deleteBookmark(id) {{
      sendIpc('delete_bookmark', {{ id: id }});
      setTimeout(() => location.reload(), 300);
    }}
  </script>
</body>
</html>
"###
        )
    }

    pub fn render_downloads(storage: &StorageEngine, lang: Language, _token: &str) -> String {
        let entries = storage.get_downloads().unwrap_or_default();
        let is_ar = lang.is_rtl();
        let dir = lang.direction();
        let lang_code = lang.code();

        let (page_title, empty_label, completed_badge, failed_badge) = if is_ar {
            ("التنزيلات", "لا توجد ملفات تم تنزيلها حتى الآن.", "مكتمل", "فشل")
        } else {
            ("Downloads", "No downloaded files recorded yet.", "COMPLETED", "FAILED")
        };

        let mut list_html = String::new();
        for item in entries {
            let safe_filename = escape_html(&item.filename);
            let safe_path = escape_html(&item.path);
            let is_failed = item.status == "failed";
            let (badge_text, badge_class) = if is_failed {
                (failed_badge, "badge failed")
            } else {
                (completed_badge, "badge")
            };
            list_html.push_str(&format!(
                r#"<div class="download-item">
                    <span class="file-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg></span>
                    <div class="file-info">
                      <div class="filename">{}</div>
                      <div class="filepath">{}</div>
                    </div>
                    <span class="{}">{}</span>
                </div>"#,
                safe_filename, safe_path, badge_class, badge_text
            ));
        }

        if list_html.is_empty() {
            list_html = format!(r#"<div class="empty">{}</div>"#, empty_label);
        }

        format!(
            r###"<!DOCTYPE html>
<html lang="{lang_code}" dir="{dir}">
<head>
<meta charset="utf-8">
<title>{page_title} - Valorex</title>
<style>
  :root {{
    --bg: #121316;
    --card: #1c1d22;
    --border: #2b2d35;
    --accent: #00d2ff;
    --text: #f0f2f5;
    --secondary: #9aa0ad;
    --font: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Cairo", "Tajawal", sans-serif;
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
    padding: 12px 16px;
    display: flex;
    align-items: center;
    gap: 16px;
  }}
  .file-icon {{ font-size: 20px; }}
  .file-info {{ flex: 1; }}
  .filename {{ font-weight: 500; font-size: 14px; margin-bottom: 2px; direction: ltr; unicode-bidi: isolate; display: inline-block; text-align: left; }}
  .filepath {{ font-size: 11px; color: var(--secondary); direction: ltr; unicode-bidi: isolate; text-align: left; }}
  .badge {{
    background: rgba(16, 185, 129, 0.15);
    color: #10b981;
    padding: 4px 8px;
    border-radius: 4px;
    font-size: 11px;
    font-weight: 600;
  }}
  .badge.failed {{
    background: rgba(239, 68, 68, 0.15);
    color: #ef4444;
  }}
  .empty {{ text-align: center; padding: 60px 0; color: var(--secondary); font-size: 16px; }}
</style>
</head>
<body>
  <h1>{page_title}</h1>
  <div class="downloads-list">
    {list_html}
  </div>
</body>
</html>
"###
        )
    }

    pub fn render_settings(lang: Language, token: &str) -> String {
        let is_ar = lang.is_rtl();
        let dir = lang.direction();
        let lang_code = lang.code();

        if is_ar {
            format!(
                r###"<!DOCTYPE html>
<html lang="{lang_code}" dir="{dir}">
<head>
<meta charset="utf-8">
<title>الإعدادات - Valorex 2027</title>
<style>
  :root {{
    --bg: #0f1013;
    --card: #18191f;
    --border: #282a33;
    --accent: #00d2ff;
    --accent-gemini: #a855f7;
    --text: #f0f2f5;
    --secondary: #9aa0ad;
    --font: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Cairo", "Tajawal", sans-serif;
  }}
  * {{ box-sizing: border-box; margin: 0; padding: 0; }}
  body {{
    background: var(--bg);
    color: var(--text);
    font-family: var(--font);
    padding: 40px;
    max-width: 820px;
    margin: 0 auto;
  }}
  .header-wrap {{
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
    padding-bottom: 16px;
    border-bottom: 1px solid var(--border);
  }}
  h1 {{ font-size: 26px; }}
  .lang-toggle-bar {{
    display: flex;
    gap: 8px;
    background: rgba(255, 255, 255, 0.05);
    padding: 4px;
    border-radius: 8px;
    border: 1px solid var(--border);
  }}
  .lang-btn {{
    background: transparent;
    border: none;
    color: var(--secondary);
    padding: 6px 14px;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s;
  }}
  .lang-btn.active {{
    background: var(--accent);
    color: #000000;
  }}
  .section {{
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 20px;
    margin-bottom: 20px;
  }}
  .section-title {{ font-size: 16px; font-weight: 600; margin-bottom: 12px; color: #ffffff; }}
  .setting-row {{
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 14px 0;
    border-bottom: 1px solid rgba(255,255,255,0.05);
  }}
  .setting-row:last-child {{ border-bottom: none; }}
  .setting-desc {{ font-size: 12px; color: var(--secondary); margin-top: 4px; line-height: 1.5; }}
  .badge {{
    background: rgba(0, 210, 255, 0.15);
    color: var(--accent);
    padding: 5px 10px;
    border-radius: 6px;
    font-size: 11px;
    font-weight: 600;
  }}
  .badge.purple {{
    background: rgba(168, 85, 247, 0.18);
    color: var(--accent-gemini);
  }}
  .badge.green {{
    background: rgba(16, 185, 129, 0.18);
    color: #10b981;
  }}
  .btn-action {{
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid var(--border);
    color: var(--text);
    padding: 7px 16px;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.15s, border-color 0.15s;
    font-family: inherit;
  }}
  .btn-action:hover {{
    background: rgba(0, 210, 255, 0.15);
    border-color: var(--accent);
  }}
  .btn-action.purple:hover {{
    background: rgba(168, 85, 247, 0.2);
    border-color: var(--accent-gemini);
  }}
  .toast {{
    position: fixed;
    bottom: 24px;
    left: 50%;
    transform: translateX(-50%);
    background: #1e2028;
    border: 1px solid var(--accent);
    color: #fff;
    padding: 12px 24px;
    border-radius: 10px;
    display: none;
    font-size: 13px;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.5);
    z-index: 1000;
  }}
</style>
</head>
<body>
  <div class="header-wrap">
    <h1>إعدادات المتصفح</h1>
    <div class="lang-toggle-bar">
      <button class="lang-btn active">العربية</button>
      <button class="lang-btn" onclick="switchLang('en')">English</button>
    </div>
  </div>

  <!-- Data Migration Section -->
  <div class="section">
    <div class="section-title">استيراد البيانات والمفضلة</div>
    <div class="setting-row">
      <div>
        <div>استيراد من Google Chrome</div>
        <div class="setting-desc">استيراد الإشارات المرجعية وسجل التصفح من ملف Chrome الشخصي المحلي بنقرة واحدة.</div>
      </div>
      <button class="btn-action" onclick="runImport('google-chrome')">استيراد Chrome</button>
    </div>
    <div class="setting-row">
      <div>
        <div>استيراد من Brave Browser</div>
        <div class="setting-desc">استيراد الإشارات وسجل التصفح وقوائم المواقع من متصفح Brave.</div>
      </div>
      <button class="btn-action" onclick="runImport('brave')">استيراد Brave</button>
    </div>
    <div class="setting-row">
      <div>
        <div>استيراد من Mozilla Firefox</div>
        <div class="setting-desc">استيراد الإشارات وسجل التصفح من قواعد بيانات Firefox SQLite.</div>
      </div>
      <button class="btn-action" onclick="runImport('firefox')">استيراد Firefox</button>
    </div>
  </div>

  <!-- Google & Gemini AI Integration -->
  <div class="section">
    <div class="section-title">خدمات الويب ومساعد Gemini</div>
    <div class="setting-row">
      <div>
        <div>تسجيل الدخول بحساب Google</div>
        <div class="setting-desc">فتح صفحة تسجيل الدخول الرسمية لخدمات Google عبر المتصفح.</div>
      </div>
      <button class="btn-action" onclick="loginGoogle()">تسجيل الدخول</button>
    </div>
    <div class="setting-row">
      <div>
        <div>الوصول السريع إلى Google Gemini</div>
        <div class="setting-desc">فتح واجهة Gemini في لسان جديد للمساعدة الذكية في البحث وتحليل المعلومات.</div>
      </div>
      <button class="btn-action purple" onclick="openGemini()">فتح Gemini</button>
    </div>
    <div class="setting-row">
      <div>
        <div>إدارة الجلسات وملفات تعريف الارتباط</div>
        <div class="setting-desc">حفظ ملفات تعريف الارتباط محلياً في مجلد الإعدادات لضمان بقاء الجلسات النشطة.</div>
      </div>
      <span class="badge green">مفعل تلقائياً</span>
    </div>
  </div>

  <!-- Shields Section -->
  <div class="section">
    <div class="section-title">حماية الخصوصية والإعلانات</div>
    <div class="setting-row">
      <div>
        <div>حجب الإعلانات والمتتبعات العالمية</div>
        <div class="setting-desc">محرك Brave Adblock المتقدم لحجب المتتبعات والإعلانات والبرمجيات الخبيثة.</div>
      </div>
      <span class="badge">نشط ومفعل</span>
    </div>
    <div class="setting-row">
      <div>
        <div>إخفاء الفراغات الإعلانية</div>
        <div class="setting-desc">إزالة المساحات الفارغة وبانرات الإعلانات المنهارة في صفحات الويب لتجربة تصفح نظيفة.</div>
      </div>
      <span class="badge">نشط ومفعل</span>
    </div>
  </div>

  <!-- Search Engine -->
  <div class="section">
    <div class="section-title">محرك البحث الافتراضي</div>
    <div class="setting-row">
      <div>
        <div>مزود البحث الافتراضي</div>
        <div class="setting-desc">محرك Google مهيأ افتراضياً لجميع استعلامات شريط العناوين.</div>
      </div>
      <span class="badge">محرك Google الافتراضي</span>
    </div>
  </div>

  <!-- Appearance & Window Controls -->
  <div class="section">
    <div class="section-title">المظهر والتحكم بالنوافذ والتبويبات</div>
    <div class="setting-row">
      <div>
        <div>أزرار التحكم بالنافذة</div>
        <div class="setting-desc">إظهار أو إخفاء أزرار التحكم بالنافذة في شريط التبويبات العلوي لمنح التبويبات المساحة الكاملة.</div>
      </div>
      <button class="btn-action" onclick="toggleWindowControls()">تبديل الإظهار</button>
    </div>
    <div class="setting-row">
      <div>
        <div>زر إغلاق التبويب</div>
        <div class="setting-desc">إظهار أو إخفاء زر الإغلاق من التبويبات لمنع الإغلاق بالخطأ وزيادة اتساع العناوين.</div>
      </div>
      <button class="btn-action" onclick="toggleTabClose()">تبديل الإظهار</button>
    </div>
    <div class="setting-row">
      <div>
        <div>شريط عنوان النظام الأصلي</div>
        <div class="setting-desc">استخدام شريط العنوان المخصص من نظام التشغيل بدلاً من شريط التبويبات المدمج.</div>
      </div>
      <button class="btn-action" onclick="toggleSystemTitlebar()">تبديل الإظهار</button>
    </div>
  </div>

  <!-- Engine Info -->
  <div class="section">
    <div class="section-title">معلومات المحرك والنظام</div>
    <div class="setting-row">
      <div>
        <div>محرك Valorex الأساسي</div>
        <div class="setting-desc">مبني بلغة Rust مع Wry 0.57 و WebKitGTK 4.1 وتقنيات محاكاة هوية كروم الأصلية.</div>
      </div>
      <span class="badge">إصدار 2027</span>
    </div>
  </div>

  <div class="toast" id="toast"></div>

  <script>
    function sendIpc(action, data) {{
      const payload = JSON.stringify({{ action: action, token: "{token}", ...(data || {{}}) }});
      if (window.ipc && typeof window.ipc.postMessage === 'function') {{
        window.ipc.postMessage(payload);
        return;
      }}
      if (window.webkit && window.webkit.messageHandlers && window.webkit.messageHandlers.ipc) {{
        window.webkit.messageHandlers.ipc.postMessage(payload);
        return;
      }}
      window.location.href = 'valorex-action://' + encodeURIComponent(payload);
    }}

    function switchLang(code) {{
      sendIpc('set_language', {{ target: code }});
      setTimeout(() => location.reload(), 200);
    }}

    function toggleWindowControls() {{
      sendIpc('toggle_window_controls');
      showToast('تم تبديل أزرار التحكم بالنافذة');
    }}

    function toggleTabClose() {{
      sendIpc('toggle_tab_close');
      showToast('تم تبديل زر إغلاق التبويبات');
    }}

    function toggleSystemTitlebar() {{
      sendIpc('toggle_decorations');
      showToast('تم تبديل استخدام شريط عنوان النظام');
    }}

    const browserNames = {{
      'google-chrome': 'Google Chrome',
      'chromium': 'Chromium',
      'brave': 'Brave',
      'microsoft-edge': 'Microsoft Edge',
      'firefox': 'Firefox'
    }};

    function runImport(browser) {{
      const bName = browserNames[browser] || browser;
      showToast('جاري استيراد البيانات من ' + bName + '...');
      sendIpc('import_data', {{ target: browser }});
    }}

    function loginGoogle() {{
      sendIpc('google_login');
    }}

    function openGemini() {{
      sendIpc('open_gemini');
    }}

    window.valorexOnImportFinished = function(summary) {{
      const bName = browserNames[summary.browser_name] || summary.browser_name;
      showToast('تم بنجاح استيراد ' + summary.bookmarks_imported + ' مفضلة و ' + summary.history_imported + ' رابط من ' + bName);
    }};

    function showToast(msg) {{
      const t = document.getElementById('toast');
      t.textContent = msg;
      t.style.display = 'block';
      setTimeout(() => {{ t.style.display = 'none'; }}, 5000);
    }}
  </script>
</body>
</html>
"###
            )
        } else {
            // English Interface
            format!(
                r###"<!DOCTYPE html>
<html lang="{lang_code}" dir="{dir}">
<head>
<meta charset="utf-8">
<title>Settings - Valorex 2027</title>
<style>
  :root {{
    --bg: #0f1013;
    --card: #18191f;
    --border: #282a33;
    --accent: #00d2ff;
    --accent-gemini: #a855f7;
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
    max-width: 820px;
    margin: 0 auto;
  }}
  .header-wrap {{
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
    padding-bottom: 16px;
    border-bottom: 1px solid var(--border);
  }}
  h1 {{ font-size: 26px; }}
  .lang-toggle-bar {{
    display: flex;
    gap: 8px;
    background: rgba(255, 255, 255, 0.05);
    padding: 4px;
    border-radius: 8px;
    border: 1px solid var(--border);
  }}
  .lang-btn {{
    background: transparent;
    border: none;
    color: var(--secondary);
    padding: 6px 14px;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s;
  }}
  .lang-btn.active {{
    background: var(--accent);
    color: #000000;
  }}
  .section {{
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 20px;
    margin-bottom: 20px;
  }}
  .section-title {{ font-size: 16px; font-weight: 600; margin-bottom: 12px; }}
  .setting-row {{
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 14px 0;
    border-bottom: 1px solid rgba(255,255,255,0.05);
  }}
  .setting-row:last-child {{ border-bottom: none; }}
  .setting-desc {{ font-size: 12px; color: var(--secondary); margin-top: 4px; }}
  .badge {{
    background: rgba(0, 210, 255, 0.15);
    color: var(--accent);
    padding: 5px 10px;
    border-radius: 6px;
    font-size: 11px;
    font-weight: 600;
  }}
  .badge.purple {{
    background: rgba(168, 85, 247, 0.18);
    color: var(--accent-gemini);
  }}
  .badge.green {{
    background: rgba(16, 185, 129, 0.18);
    color: #10b981;
  }}
  .btn-action {{
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid var(--border);
    color: var(--text);
    padding: 6px 14px;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.15s, border-color 0.15s;
  }}
  .btn-action:hover {{
    background: rgba(0, 210, 255, 0.15);
    border-color: var(--accent);
  }}
  .btn-action.purple:hover {{
    background: rgba(168, 85, 247, 0.2);
    border-color: var(--accent-gemini);
  }}
  .toast {{
    position: fixed;
    bottom: 20px;
    right: 20px;
    background: #1e2028;
    border: 1px solid var(--accent);
    color: #fff;
    padding: 12px 20px;
    border-radius: 8px;
    display: none;
    font-size: 13px;
  }}
</style>
</head>
<body>
  <div class="header-wrap">
    <h1>Valorex Settings</h1>
    <div class="lang-toggle-bar">
      <button class="lang-btn" onclick="switchLang('ar')">العربية</button>
      <button class="lang-btn active">English</button>
    </div>
  </div>

  <!-- Data Migration Section -->
  <div class="section">
    <div class="section-title">Browser Data Migration</div>
    <div class="setting-row">
      <div>
        <div>Import from Google Chrome</div>
        <div class="setting-desc">Imports bookmarks and history from your local Google Chrome profile.</div>
      </div>
      <button class="btn-action" onclick="runImport('google-chrome')">Import Chrome</button>
    </div>
    <div class="setting-row">
      <div>
        <div>Import from Brave Browser</div>
        <div class="setting-desc">Imports bookmarks and history from your local Brave profile.</div>
      </div>
      <button class="btn-action" onclick="runImport('brave')">Import Brave</button>
    </div>
    <div class="setting-row">
      <div>
        <div>Import from Mozilla Firefox</div>
        <div class="setting-desc">Imports bookmarks and history from Firefox SQLite databases.</div>
      </div>
      <button class="btn-action" onclick="runImport('firefox')">Import Firefox</button>
    </div>
  </div>

  <!-- Google & Gemini AI Integration -->
  <div class="section">
    <div class="section-title">Web Services & Gemini Assistant</div>
    <div class="setting-row">
      <div>
        <div>Google Services Sign-in</div>
        <div class="setting-desc">Launch official Google account authentication page in a dedicated browser tab.</div>
      </div>
      <button class="btn-action" onclick="loginGoogle()">Sign in to Google</button>
    </div>
    <div class="setting-row">
      <div>
        <div>Google Gemini Quick Access</div>
        <div class="setting-desc">Launch Google Gemini in a new tab for research assistance and query analysis.</div>
      </div>
      <button class="btn-action purple" onclick="openGemini()">Launch Gemini</button>
    </div>
    <div class="setting-row">
      <div>
        <div>Cookie & Session Storage</div>
        <div class="setting-desc">Store web cookies and session state locally in the user config profile.</div>
      </div>
      <span class="badge green">ENABLED</span>
    </div>
  </div>

  <!-- Shields Section -->
  <div class="section">
    <div class="section-title">Shields & Privacy Protection</div>
    <div class="setting-row">
      <div>
        <div>Global Ad & Tracker Blocking</div>
        <div class="setting-desc">Brave Adblock engine blocking third-party telemetry, ads, and malware.</div>
      </div>
      <span class="badge">ACTIVE</span>
    </div>
    <div class="setting-row">
      <div>
        <div>Cosmetic Element Hiding</div>
        <div class="setting-desc">Removes empty visual gaps and collapsed ad banners on web pages.</div>
      </div>
      <span class="badge">ACTIVE</span>
    </div>
  </div>

  <!-- Search Engine -->
  <div class="section">
    <div class="section-title">Search Engine</div>
    <div class="setting-row">
      <div>
        <div>Default Search Provider</div>
        <div class="setting-desc">Google Search engine configured for universal omnibox queries.</div>
      </div>
      <span class="badge">Google (Default)</span>
    </div>
  </div>

  <!-- Appearance & Window Controls -->
  <div class="section">
    <div class="section-title">Appearance & Window Controls</div>
    <div class="setting-row">
      <div>
        <div>Window Caption Controls</div>
        <div class="setting-desc">Show or hide the window caption buttons (Minimize, Maximize, Close) in the tab bar.</div>
      </div>
      <button class="btn-action" onclick="toggleWindowControls()">Toggle Controls</button>
    </div>
    <div class="setting-row">
      <div>
        <div>Tab Close (X) Button</div>
        <div class="setting-desc">Show or hide the close (X) button on tabs to prevent accidental tab closing.</div>
      </div>
      <button class="btn-action" onclick="toggleTabClose()">Toggle Close (X)</button>
    </div>
    <div class="setting-row">
      <div>
        <div>System Title Bar</div>
        <div class="setting-desc">Use OS native window decoration title bar instead of integrated tab bar.</div>
      </div>
      <button class="btn-action" onclick="toggleSystemTitlebar()">Toggle System Bar</button>
    </div>
  </div>

  <!-- Engine Info -->
  <div class="section">
    <div class="section-title">Architecture & Diagnostics</div>
    <div class="setting-row">
      <div>
        <div>Valorex Core Engine</div>
        <div class="setting-desc">Rust + Wry 0.57 + WebKitGTK 4.1 + Native Chrome spoofing.</div>
      </div>
      <span class="badge">v0.2.0 (2027 Edition)</span>
    </div>
  </div>

  <div class="toast" id="toast"></div>

  <script>
    function sendIpc(action, data) {{
      const payload = JSON.stringify({{ action: action, token: "{token}", ...(data || {{}}) }});
      if (window.ipc && typeof window.ipc.postMessage === 'function') {{
        window.ipc.postMessage(payload);
        return;
      }}
      if (window.webkit && window.webkit.messageHandlers && window.webkit.messageHandlers.ipc) {{
        window.webkit.messageHandlers.ipc.postMessage(payload);
        return;
      }}
      window.location.href = 'valorex-action://' + encodeURIComponent(payload);
    }}

    function switchLang(code) {{
      sendIpc('set_language', {{ target: code }});
      setTimeout(() => location.reload(), 200);
    }}

    function toggleWindowControls() {{
      sendIpc('toggle_window_controls');
      showToast('Toggled Window Caption Controls');
    }}

    function toggleTabClose() {{
      sendIpc('toggle_tab_close');
      showToast('Toggled Tab Close Button');
    }}

    function toggleSystemTitlebar() {{
      sendIpc('toggle_decorations');
      showToast('Toggled System Title Bar');
    }}

    const browserNames = {{
      'google-chrome': 'Google Chrome',
      'chromium': 'Chromium',
      'brave': 'Brave',
      'microsoft-edge': 'Microsoft Edge',
      'firefox': 'Firefox'
    }};

    function runImport(browser) {{
      const bName = browserNames[browser] || browser;
      showToast('Importing data from ' + bName + '...');
      sendIpc('import_data', {{ target: browser }});
    }}

    function loginGoogle() {{
      sendIpc('google_login');
    }}

    function openGemini() {{
      sendIpc('open_gemini');
    }}

    window.valorexOnImportFinished = function(summary) {{
      const bName = browserNames[summary.browser_name] || summary.browser_name;
      showToast('Successfully imported ' + summary.bookmarks_imported + ' bookmarks and ' + summary.history_imported + ' history entries from ' + bName);
    }};

    function showToast(msg) {{
      const t = document.getElementById('toast');
      t.textContent = msg;
      t.style.display = 'block';
      setTimeout(() => {{ t.style.display = 'none'; }}, 5000);
    }}
  </script>
</body>
</html>
"###
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_html() {
        assert_eq!(escape_html("<script>alert('xss')</script>"), "&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;");
        assert_eq!(escape_html("Hello & \"World\""), "Hello &amp; &quot;World&quot;");
    }

    #[test]
    fn test_sanitize_href() {
        assert_eq!(sanitize_href("javascript:alert(1)"), "#");
        assert_eq!(sanitize_href("JAVASCRIPT:void(0)"), "#");
        assert_eq!(sanitize_href("data:text/html,evil"), "#");
        assert_eq!(sanitize_href("https://valorex.org"), "https://valorex.org");
        assert_eq!(sanitize_href("http://example.com"), "http://example.com");
        assert_eq!(sanitize_href("valorex://settings"), "valorex://settings");
    }

    #[test]
    fn test_render_settings_contains_accurate_descriptions() {
        let test_token = "secret_nonce_abc123";
        let ar_html = InternalPages::render_settings(Language::Arabic, test_token);
        assert!(ar_html.contains("خدمات الويب ومساعد Gemini"));
        assert!(ar_html.contains("الوصول السريع إلى Google Gemini"));
        assert!(ar_html.contains("إدارة الجلسات وملفات تعريف الارتباط"));
        assert!(ar_html.contains("dir=\"rtl\""));
        assert!(ar_html.contains(test_token));

        let en_html = InternalPages::render_settings(Language::English, test_token);
        assert!(en_html.contains("Web Services &amp; Gemini Assistant") || en_html.contains("Web Services & Gemini Assistant"));
        assert!(en_html.contains("Google Gemini Quick Access"));
        assert!(en_html.contains("Cookie &amp; Session Storage") || en_html.contains("Cookie & Session Storage"));
        assert!(en_html.contains("dir=\"ltr\""));
        assert!(en_html.contains(test_token));
    }

    #[test]
    fn test_render_newtab_bidi() {
        let test_token = "newtab_token_xyz987";
        let ar_newtab = InternalPages::render_newtab(500, Language::Arabic, test_token);
        assert!(ar_newtab.contains("dir=\"rtl\""));
        assert!(ar_newtab.contains("lang=\"ar\""));
        assert!(ar_newtab.contains("أهلاً بك في Valorex"));
        assert!(ar_newtab.contains(test_token));

        let en_newtab = InternalPages::render_newtab(500, Language::English, test_token);
        assert!(en_newtab.contains("dir=\"ltr\""));
        assert!(en_newtab.contains("lang=\"en\""));
        assert!(en_newtab.contains("Welcome to Valorex"));
        assert!(en_newtab.contains(test_token));
    }
}

