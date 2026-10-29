pub mod events;
pub mod tab;

use crate::browser::events::BrowserEvent;
use crate::browser::tab::TabManager;
use crate::config::BrowserConfig;
use crate::protocol::InternalPages;
use crate::shield::ShieldEngine;
use crate::storage::StorageEngine;
use crate::ui::CHROME_HTML;
use log::{info, warn};
use std::sync::{Arc, Mutex};
use tao::{
    dpi::LogicalSize,
    event_loop::{EventLoopProxy, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};
use wry::{http, PageLoadEvent, WebView, WebViewBuilder};

#[cfg(target_os = "linux")]
use gtk::prelude::*;
#[cfg(target_os = "linux")]
use tao::platform::unix::WindowExtUnix;
#[cfg(target_os = "linux")]
use wry::WebViewBuilderExtUnix;

use std::path::{Path, PathBuf};

pub struct BrowserWindow {
    pub window: Window,
    pub proxy: EventLoopProxy<BrowserEvent>,
    pub toolbar_webview: WebView,
    pub content_webview: WebView,
    pub tab_manager: TabManager,
    pub shield: Arc<Mutex<ShieldEngine>>,
    pub storage: StorageEngine,
    pub config: BrowserConfig,
    pub zoom_level: f64,
    pub is_fullscreen: bool,
    pub decorations_enabled: bool,
    pub show_window_controls: bool,
    pub show_tab_close_buttons: bool,
    pub current_language: crate::i18n::Language,
    pub shared_lang: Arc<Mutex<crate::i18n::Language>>,
    pub current_active_url: Arc<Mutex<String>>,
    pub _web_context: wry::WebContext,
}

impl BrowserWindow {
    pub fn new(
        event_loop: &EventLoopWindowTarget<BrowserEvent>,
        proxy: EventLoopProxy<BrowserEvent>,
        config: BrowserConfig,
        shield: Arc<Mutex<ShieldEngine>>,
        storage: StorageEngine,
        initial_target_url: Option<String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let window = WindowBuilder::new()
            .with_title("Valorex")
            .with_inner_size(LogicalSize::new(1280.0, 800.0))
            .with_min_inner_size(LogicalSize::new(450.0, 350.0))
            .with_decorations(config.decorations_visible)
            .build(event_loop)?;

        let initial_url = initial_target_url.unwrap_or_else(|| config.homepage.clone());
        info!("Launching Valorex webview with initial homepage: {}", initial_url);
        let current_language = crate::i18n::Language::from_code(&config.language);
        let shared_lang = Arc::new(Mutex::new(current_language));
        let current_active_url = Arc::new(Mutex::new(initial_url.clone()));

        // --- Toolbar Webview Builder ---
        let proxy_ipc = proxy.clone();
        let toolbar_builder = WebViewBuilder::new()
            .with_html(CHROME_HTML)
            .with_ipc_handler(move |req: wry::http::Request<String>| {
                #[derive(serde::Deserialize)]
                #[allow(dead_code)]
                struct IpcPayload {
                    action: String,
                    url: Option<String>,
                    target: Option<String>,
                    prompt: Option<String>,
                    id: Option<usize>,
                    from_id: Option<usize>,
                    to_id: Option<usize>,
                    index: Option<usize>,
                    enabled: Option<bool>,
                    x: Option<i32>,
                    y: Option<i32>,
                }

                let msg = req.body();
                if let Ok(payload) = serde_json::from_str::<IpcPayload>(msg) {
                    match payload.action.as_str() {
                        "navigate" => {
                            if let Some(url) = payload.url {
                                let _ = proxy_ipc.send_event(BrowserEvent::Navigate(url));
                            }
                        }
                        "go_back" => {
                            let _ = proxy_ipc.send_event(BrowserEvent::GoBack);
                        }
                        "go_forward" => {
                            let _ = proxy_ipc.send_event(BrowserEvent::GoForward);
                        }
                        "reload" => {
                            let _ = proxy_ipc.send_event(BrowserEvent::Reload);
                        }
                        "new_tab" => {
                            let url = payload.url.unwrap_or_else(|| "valorex://newtab".to_string());
                            let _ = proxy_ipc.send_event(BrowserEvent::NewTab(url));
                        }
                        "close_tab" => {
                            if let Some(id) = payload.id {
                                let _ = proxy_ipc.send_event(BrowserEvent::CloseTab(id));
                            }
                        }
                        "switch_tab" => {
                            if let Some(id) = payload.id {
                                let _ = proxy_ipc.send_event(BrowserEvent::SwitchTab(id));
                            }
                        }
                        "toggle_shield" => {
                            let _ = proxy_ipc.send_event(BrowserEvent::ToggleShield(payload.enabled));
                        }
                        "toggle_bookmark" => {
                            let _ = proxy_ipc.send_event(BrowserEvent::ToggleBookmark);
                        }
                        "clear_history" => {
                            let _ = proxy_ipc.send_event(BrowserEvent::ClearHistory);
                        }
                        "delete_bookmark" => {
                            if let Some(id) = payload.id {
                                let _ = proxy_ipc.send_event(BrowserEvent::DeleteBookmark(id as i64));
                            }
                        }
                        "duplicate_tab" => {
                            if let Some(id) = payload.id {
                                let _ = proxy_ipc.send_event(BrowserEvent::DuplicateTab(id));
                            }
                        }
                        "pin_tab" => {
                            if let Some(id) = payload.id {
                                let _ = proxy_ipc.send_event(BrowserEvent::PinTab(id));
                            }
                        }
                        "close_other_tabs" => {
                            if let Some(id) = payload.id {
                                let _ = proxy_ipc.send_event(BrowserEvent::CloseOtherTabs(id));
                            }
                        }
                        "reorder_tabs" => {
                            if let (Some(f), Some(t)) = (payload.from_id, payload.to_id) {
                                let _ = proxy_ipc.send_event(BrowserEvent::ReorderTabs(f, t));
                            }
                        }
                        "next_tab" => {
                            let _ = proxy_ipc.send_event(BrowserEvent::NextTab);
                        }
                        "prev_tab" => {
                            let _ = proxy_ipc.send_event(BrowserEvent::PrevTab);
                        }
                        "switch_tab_index" => {
                            if let Some(idx) = payload.index {
                                let _ = proxy_ipc.send_event(BrowserEvent::SwitchTabIndex(idx));
                            }
                        }
                        "open_menu" => {
                            let _ = proxy_ipc.send_event(BrowserEvent::OpenMenu);
                        }
                        "open_shield_menu" => {
                            let _ = proxy_ipc.send_event(BrowserEvent::OpenShieldMenu);
                        }
                        "open_tab_context" => {
                            let coords = payload.x.and_then(|x| payload.y.map(|y| (x, y)));
                            if let Some(id) = payload.id {
                                let _ = proxy_ipc.send_event(BrowserEvent::OpenTabContextMenu(id, coords));
                            }
                        }
                        "minimize_window" => {
                            let _ = proxy_ipc.send_event(BrowserEvent::MinimizeWindow);
                        }
                        "maximize_window" => {
                            let _ = proxy_ipc.send_event(BrowserEvent::MaximizeWindow);
                        }
                        "close_window" => {
                            let _ = proxy_ipc.send_event(BrowserEvent::CloseWindow);
                        }
                        "drag_window" => {
                            let _ = proxy_ipc.send_event(BrowserEvent::DragWindow);
                        }
                        "toggle_fullscreen" => {
                            let _ = proxy_ipc.send_event(BrowserEvent::ToggleFullscreen);
                        }
                        "import_data" => {
                            let target = payload.target.unwrap_or_else(|| "google-chrome".to_string());
                            let _ = proxy_ipc.send_event(BrowserEvent::ImportData(target));
                        }
                        "open_gemini" => {
                            let _ = proxy_ipc.send_event(BrowserEvent::OpenGemini);
                        }
                        "summarize_page" => {
                            let _ = proxy_ipc.send_event(BrowserEvent::SummarizePage);
                        }
                        "google_login" => {
                            let _ = proxy_ipc.send_event(BrowserEvent::OpenGoogleLogin);
                        }
                        "toggle_window_controls" => {
                            let _ = proxy_ipc.send_event(BrowserEvent::ToggleWindowControls);
                        }
                        "toggle_tab_close" => {
                            let _ = proxy_ipc.send_event(BrowserEvent::ToggleTabCloseButtons);
                        }
                        "close_tabs_to_right" => {
                            if let Some(id) = payload.id {
                                let _ = proxy_ipc.send_event(BrowserEvent::CloseTabsToRight(id));
                            }
                        }
                        "reopen_closed_tab" => {
                            let _ = proxy_ipc.send_event(BrowserEvent::ReopenClosedTab);
                        }
                        "open_tab_bar_context" => {
                            let coords = payload.x.and_then(|x| payload.y.map(|y| (x, y)));
                            let _ = proxy_ipc.send_event(BrowserEvent::OpenTabBarContextMenu(coords));
                        }
                        "set_language" => {
                            if let Some(target) = payload.target {
                                let _ = proxy_ipc.send_event(BrowserEvent::SetLanguage(target));
                            }
                        }
                        _ => {}
                    }
                }
            });

        // --- Content Webview Builder ---
        let proxy_load = proxy.clone();
        let active_url_load = current_active_url.clone();
        let on_page_load = move |event: PageLoadEvent, url: String| {
            if let PageLoadEvent::Finished = event {
                *active_url_load.lock().unwrap_or_else(|e| e.into_inner()) = url.clone();
                let _ = proxy_load.send_event(BrowserEvent::PageLoaded(url));
            }
        };

        let proxy_title = proxy.clone();
        let on_title = move |title: String| {
            let _ = proxy_title.send_event(BrowserEvent::TitleChanged(title));
        };

        let secret_token = {
            use std::io::Read;
            let mut buf = [0u8; 16];
            let mut filled = false;
            if let Ok(mut f) = std::fs::File::open("/dev/urandom") {
                if f.read_exact(&mut buf).is_ok() {
                    filled = true;
                }
            }
            if !filled {
                use std::time::SystemTime;
                let nanos = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos();
                let pid = std::process::id() as u128;
                let addr = (&buf as *const _ as usize) as u128;
                let seed = nanos ^ (pid << 32) ^ (addr << 64);
                let seed_bytes = seed.to_le_bytes();
                buf.copy_from_slice(&seed_bytes[..16]);
            }
            buf.iter().map(|b| format!("{:02x}", b)).collect::<String>()
        };
        let secret_token = Arc::new(secret_token);
        let token_nav = secret_token.clone();

        let shield_for_nav = shield.clone();
        let proxy_nav = proxy.clone();
        let active_url_nav = current_active_url.clone();
        let on_navigate = move |url: String| -> bool {
            let trimmed = url.trim();
            let trimmed_lower = trimmed.to_lowercase();
            let current_url = active_url_nav.lock().unwrap_or_else(|e| e.into_inner()).clone();
            let is_internal = current_url.starts_with("valorex://");

            if trimmed.starts_with("valorex-action://") {
                if !is_internal {
                    warn!("Security: blocked unauthorized valorex-action:// navigation from external page: {}", current_url);
                    return false;
                }
                let action_raw = &trimmed["valorex-action://".len()..];
                if let Ok(decoded) = percent_encoding::percent_decode_str(action_raw).decode_utf8() {
                    Self::dispatch_content_ipc_str(&decoded, &proxy_nav, is_internal, &token_nav);
                }
                return false;
            }

            // Block external websites from navigating to local file://, internal valorex://, javascript: or data: URLs
            if !is_internal && (trimmed_lower.starts_with("file://")
                || trimmed_lower.starts_with("valorex://")
                || trimmed_lower.starts_with("valorex-action://")
                || trimmed_lower.starts_with("javascript:")
                || trimmed_lower.starts_with("data:")) {
                warn!("Security: blocked unauthorized navigation to {} from external page {}", url, current_url);
                return false;
            }

            *active_url_nav.lock().unwrap_or_else(|e| e.into_inner()) = trimmed.to_string();

            let source_url = if current_url.is_empty() { trimmed } else { &current_url };
            let mut s = shield_for_nav.lock().unwrap_or_else(|e| e.into_inner());
            if s.should_block(trimmed, source_url, "document") {
                warn!("Valorex Shield blocked navigation to: {}", url);
                false
            } else {
                true
            }
        };

        let proxy_dl_start = proxy.clone();
        let on_dl_start = move |url: String, path: &mut PathBuf| -> bool {
            let filename = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("downloaded_file");
            let download_dir = dirs::download_dir().unwrap_or_else(|| {
                dirs::home_dir().map(|h| h.join("Downloads")).unwrap_or_else(std::env::temp_dir)
            });
            let _ = std::fs::create_dir_all(&download_dir);

            let mut target_path = download_dir.join(filename);
            if target_path.exists() {
                let p = Path::new(filename);
                let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("download");
                let ext = p.extension().and_then(|e| e.to_str()).map(|e| format!(".{}", e)).unwrap_or_default();
                let mut counter = 1;
                while target_path.exists() {
                    target_path = download_dir.join(format!("{} ({}){}", stem, counter, ext));
                    counter += 1;
                }
            }
            *path = target_path.clone();
            let _ = proxy_dl_start.send_event(BrowserEvent::DownloadStarted(url, path.clone()));
            true
        };

        let proxy_dl_comp = proxy.clone();
        let on_dl_comp = move |url: String, path: Option<PathBuf>, success: bool| {
            let _ = proxy_dl_comp.send_event(BrowserEvent::DownloadCompleted(url, path, success));
        };

        let blocked_counter = shield.lock().unwrap_or_else(|e| e.into_inner()).blocked_counter();
        let storage_for_protocol = storage.clone();
        let proxy_content_ipc = proxy.clone();

        let shortcuts_script = format!(
            r#"
            (function() {{
                const __valorex_token = "{}";
                const nativeIpcPost = (window.ipc && typeof window.ipc.postMessage === 'function')
                    ? window.ipc.postMessage.bind(window.ipc)
                    : (window.webkit && window.webkit.messageHandlers && window.webkit.messageHandlers.ipc && typeof window.webkit.messageHandlers.ipc.postMessage === 'function')
                        ? window.webkit.messageHandlers.ipc.postMessage.bind(window.webkit.messageHandlers.ipc)
                        : null;

                function sendContentIpc(payload) {{
                    if (payload && typeof payload === 'object') {{
                        payload.token = __valorex_token;
                    }}
                    const jsonStr = typeof payload === 'string' ? payload : JSON.stringify(payload);
                    if (nativeIpcPost) {{
                        nativeIpcPost(jsonStr);
                        return;
                    }}
                }}

                window.addEventListener('keydown', function(e) {{
                    if (!e.isTrusted) return;
                    if (e.key === 'F11') {{
                        e.preventDefault();
                        sendContentIpc({{ action: 'toggle_fullscreen' }});
                    }} else if (e.ctrlKey && e.key === 'Tab') {{
                        e.preventDefault();
                        if (e.shiftKey) {{
                            sendContentIpc({{ action: 'prev_tab' }});
                        }} else {{
                            sendContentIpc({{ action: 'next_tab' }});
                        }}
                    }} else if (e.ctrlKey && !e.shiftKey && !e.altKey && e.key >= '1' && e.key <= '9') {{
                        e.preventDefault();
                        sendContentIpc({{ action: 'switch_tab_index', index: parseInt(e.key) - 1 }});
                    }} else if (e.ctrlKey && e.shiftKey && !e.altKey) {{
                        if (e.key === 't' || e.key === 'T') {{
                            e.preventDefault();
                            sendContentIpc({{ action: 'reopen_closed_tab' }});
                        }} else if (e.key === 'g' || e.key === 'G') {{
                            e.preventDefault();
                            sendContentIpc({{ action: 'open_gemini' }});
                        }}
                    }} else if (e.ctrlKey && !e.shiftKey && !e.altKey) {{
                        if (e.key === 't' || e.key === 'T') {{
                            e.preventDefault();
                            sendContentIpc({{ action: 'new_tab' }});
                        }} else if (e.key === 'w' || e.key === 'W') {{
                            e.preventDefault();
                            sendContentIpc({{ action: 'close_tab' }});
                        }} else if (e.key === 'r' || e.key === 'R') {{
                            e.preventDefault();
                            sendContentIpc({{ action: 'reload' }});
                        }} else if (e.key === 'h' || e.key === 'H') {{
                            e.preventDefault();
                            sendContentIpc({{ action: 'open_history' }});
                        }} else if (e.key === 'b' || e.key === 'B') {{
                            e.preventDefault();
                            sendContentIpc({{ action: 'open_bookmarks' }});
                        }} else if (e.key === 'j' || e.key === 'J') {{
                            e.preventDefault();
                            sendContentIpc({{ action: 'open_downloads' }});
                        }} else if (e.key === '=' || e.key === '+') {{
                            e.preventDefault();
                            sendContentIpc({{ action: 'zoom_in' }});
                        }} else if (e.key === '-' || e.key === '_') {{
                            e.preventDefault();
                            sendContentIpc({{ action: 'zoom_out' }});
                        }} else if (e.key === '0') {{
                            e.preventDefault();
                            sendContentIpc({{ action: 'zoom_reset' }});
                        }}
                    }}
                }}, true);

                // Floating Smart Action Bar on text selection
                let selectionBar = null;
                document.addEventListener('mouseup', function(e) {{
                    if (!e.isTrusted) return;
                    const sel = window.getSelection();
                    const text = sel ? sel.toString().trim() : '';
                    if (text && text.length > 0) {{
                        if (!selectionBar) {{
                            selectionBar = document.createElement('div');
                            selectionBar.id = 'valorex-selection-bar';
                            selectionBar.style.cssText = 'position:fixed; z-index:2147483647; background:#18191f; border:1px solid #00d2ff; border-radius:8px; padding:4px 8px; display:flex; gap:6px; box-shadow:0 8px 24px rgba(0,0,0,0.6); font-family:-apple-system,sans-serif; font-size:12px;';
                            document.body.appendChild(selectionBar);
                        }}
                        selectionBar.innerHTML = `
                            <button id="valorex-sel-search" style="background:#242731; border:none; color:#f0f2f5; padding:4px 8px; border-radius:5px; cursor:pointer; display:flex; align-items:center; gap:4px;"><svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>Google</button>
                            <button id="valorex-sel-gemini" style="background:rgba(168,85,247,0.25); border:1px solid #a855f7; color:#d8b4fe; padding:4px 8px; border-radius:5px; cursor:pointer; display:flex; align-items:center; gap:4px;"><svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2l2.4 7.6L22 12l-7.6 2.4L12 22l-2.4-7.6L2 12l7.6-2.4z"/></svg>Ask Gemini</button>
                            <button id="valorex-sel-copy" style="background:#242731; border:none; color:#f0f2f5; padding:4px 8px; border-radius:5px; cursor:pointer; display:flex; align-items:center; gap:4px;"><svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>Copy</button>
                        `;
                        const rect = sel.getRangeAt(0).getBoundingClientRect();
                        selectionBar.style.top = Math.max(10, rect.top - 40) + 'px';
                        selectionBar.style.left = Math.max(10, Math.min(window.innerWidth - 220, rect.left)) + 'px';
                        selectionBar.style.display = 'flex';

                        document.getElementById('valorex-sel-search').onclick = () => {{
                            sendContentIpc({{ action: 'navigate', url: 'https://www.google.com/search?q=' + encodeURIComponent(text) }});
                            selectionBar.style.display = 'none';
                        }};
                        document.getElementById('valorex-sel-gemini').onclick = () => {{
                            sendContentIpc({{ action: 'navigate', url: 'https://gemini.google.com/app?q=' + encodeURIComponent(text) }});
                            selectionBar.style.display = 'none';
                        }};
                        document.getElementById('valorex-sel-copy').onclick = () => {{
                            navigator.clipboard.writeText(text);
                            selectionBar.style.display = 'none';
                        }};
                    }} else if (selectionBar) {{
                        selectionBar.style.display = 'none';
                    }}
                }});
            }})();
            "#,
            secret_token
        );

        let mut web_context = if config.incognito {
            wry::WebContext::new(None)
        } else {
            let mut session_dir = dirs::config_dir().unwrap_or_else(|| {
                dirs::home_dir().map(|h| h.join(".config")).unwrap_or_else(|| std::env::temp_dir().join("valorex_config"))
            });
            session_dir.push("valorex");
            session_dir.push("session");
            let _ = std::fs::create_dir_all(&session_dir);
            wry::WebContext::new(Some(session_dir))
        };

        let shared_lang_for_proto = shared_lang.clone();
        let active_url_ipc = current_active_url.clone();
        let token_ipc = secret_token.clone();
        let content_builder = WebViewBuilder::new_with_web_context(&mut web_context)
            .with_user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
            .with_url(&initial_url)
            .with_devtools(!config.incognito)
            .with_initialization_script(&shortcuts_script)
            .with_ipc_handler(move |req: wry::http::Request<String>| {
                let req_uri = req.uri().to_string();
                let current_url = active_url_ipc.lock().unwrap_or_else(|e| e.into_inner()).clone();
                // A request is internal ONLY if WebKit's request origin URI confirms it is valorex://
                // or if req_uri is empty (initial state) AND current_url is confirmed valorex://
                let is_internal = req_uri.starts_with("valorex://") || (req_uri.is_empty() && current_url.starts_with("valorex://"));
                Self::dispatch_content_ipc_str(req.body(), &proxy_content_ipc, is_internal, &token_ipc);
            })
            .with_on_page_load_handler(on_page_load)
            .with_document_title_changed_handler(on_title)
            .with_navigation_handler(on_navigate)
            .with_download_started_handler(on_dl_start)
            .with_download_completed_handler(on_dl_comp)
            .with_custom_protocol("valorex".into(), move |_id, req| {
                use std::sync::atomic::Ordering;
                let uri = req.uri().to_string();
                let current_lang = *shared_lang_for_proto.lock().unwrap_or_else(|e| e.into_inner());
                let stripped = uri.trim_start_matches("valorex://");
                let page = stripped.split(['/', '?', '#']).next().unwrap_or("newtab");
                let body = match page {
                    "history" => InternalPages::render_history(&storage_for_protocol, current_lang),
                    "bookmarks" => InternalPages::render_bookmarks(&storage_for_protocol, current_lang),
                    "downloads" => InternalPages::render_downloads(&storage_for_protocol, current_lang),
                    "settings" => InternalPages::render_settings(current_lang),
                    _ => InternalPages::render_newtab(blocked_counter.load(Ordering::Relaxed), current_lang),
                };

                http::Response::builder()
                    .header("content-type", "text/html; charset=utf-8")
                    .status(200)
                    .body(body.as_bytes().to_vec())
                    .unwrap_or_else(|_| http::Response::new(Vec::new()))
                    .map(Into::into)
            });

        // --- GTK Dual-Layer Packing on Linux ---
        #[cfg(target_os = "linux")]
        let (toolbar_webview, content_webview) = {
            let vbox = window
                .default_vbox()
                .ok_or_else(|| "Failed to obtain GTK default vbox from Tao window".to_string())?;

            let toolbar_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
            toolbar_box.set_size_request(-1, 82);

            let content_box = gtk::Box::new(gtk::Orientation::Vertical, 0);

            vbox.pack_start(&toolbar_box, false, false, 0);
            vbox.pack_start(&content_box, true, true, 0);

            toolbar_box.show_all();
            content_box.show_all();

            let tb = toolbar_builder.build_gtk(&toolbar_box)?;
            let cb = content_builder.build_gtk(&content_box)?;
            (tb, cb)
        };

        #[cfg(not(target_os = "linux"))]
        let (toolbar_webview, content_webview) = {
            let tb = toolbar_builder.build(&window)?;
            let cb = content_builder.build(&window)?;
            (tb, cb)
        };

        let tab_manager = TabManager::new(&initial_url);

        let instance = Self {
            window,
            proxy,
            toolbar_webview,
            content_webview,
            tab_manager,
            shield,
            storage,
            decorations_enabled: config.decorations_visible,
            show_window_controls: config.window_controls_visible,
            show_tab_close_buttons: config.tab_close_visible,
            config,
            zoom_level: 1.0,
            is_fullscreen: false,
            current_language,
            shared_lang,
            current_active_url,
            _web_context: web_context,
        };

        instance.sync_toolbar_state();

        Ok(instance)
    }

    pub fn authorize_and_parse_content_ipc(
        msg: &str,
        is_internal: bool,
        secret_token: &str,
    ) -> Option<BrowserEvent> {
        #[derive(serde::Deserialize)]
        struct ContentIpc {
            action: String,
            url: Option<String>,
            target: Option<String>,
            id: Option<i64>,
            index: Option<usize>,
            token: Option<String>,
        }
        let payload = serde_json::from_str::<ContentIpc>(msg).ok()?;
        let is_authorized = is_internal || payload.token.as_deref() == Some(secret_token);

        match payload.action.as_str() {
            "navigate" => {
                if !is_authorized {
                    warn!("Security: blocked unauthorized navigate IPC from external page");
                    return None;
                }
                let url = payload.url?;
                let trimmed_lower = url.trim().to_lowercase();
                if !is_internal && (trimmed_lower.starts_with("file://")
                    || trimmed_lower.starts_with("valorex://")
                    || trimmed_lower.starts_with("valorex-action://")
                    || trimmed_lower.starts_with("javascript:")
                    || trimmed_lower.starts_with("data:")) {
                    warn!("Security: blocked privileged navigation from non-internal origin: {}", url);
                    return None;
                }
                Some(BrowserEvent::Navigate(url))
            }
            "open_history" => {
                if !is_authorized {
                    warn!("Security: rejected unauthorized open_history");
                    return None;
                }
                Some(BrowserEvent::Navigate("valorex://history".into()))
            }
            "open_bookmarks" => {
                if !is_authorized {
                    warn!("Security: rejected unauthorized open_bookmarks");
                    return None;
                }
                Some(BrowserEvent::Navigate("valorex://bookmarks".into()))
            }
            "open_downloads" => {
                if !is_authorized {
                    warn!("Security: rejected unauthorized open_downloads");
                    return None;
                }
                Some(BrowserEvent::Navigate("valorex://downloads".into()))
            }
            "clear_history" => {
                if !is_internal {
                    warn!("Security: rejected unauthorized clear_history from external web content");
                    return None;
                }
                Some(BrowserEvent::ClearHistory)
            }
            "delete_bookmark" => {
                if !is_internal {
                    warn!("Security: rejected unauthorized delete_bookmark from external web content");
                    return None;
                }
                payload.id.map(BrowserEvent::DeleteBookmark)
            }
            "new_tab" => {
                if !is_authorized {
                    warn!("Security: rejected unauthorized new_tab from external web content");
                    return None;
                }
                Some(BrowserEvent::NewTab("valorex://newtab".into()))
            }
            "close_tab" => {
                if !is_authorized {
                    warn!("Security: rejected unauthorized close_tab from external web content");
                    return None;
                }
                Some(BrowserEvent::CloseTab(payload.id.unwrap_or(0) as usize))
            }
            "reload" => {
                if !is_authorized {
                    warn!("Security: rejected unauthorized reload from external web content");
                    return None;
                }
                Some(BrowserEvent::Reload)
            }
            "zoom_in" => {
                if !is_authorized { return None; }
                Some(BrowserEvent::ZoomIn)
            }
            "zoom_out" => {
                if !is_authorized { return None; }
                Some(BrowserEvent::ZoomOut)
            }
            "zoom_reset" => {
                if !is_authorized { return None; }
                Some(BrowserEvent::ZoomReset)
            }
            "toggle_fullscreen" => {
                if !is_authorized { return None; }
                Some(BrowserEvent::ToggleFullscreen)
            }
            "next_tab" => {
                if !is_authorized { return None; }
                Some(BrowserEvent::NextTab)
            }
            "prev_tab" => {
                if !is_authorized { return None; }
                Some(BrowserEvent::PrevTab)
            }
            "switch_tab_index" => {
                if !is_authorized { return None; }
                payload.index.map(BrowserEvent::SwitchTabIndex)
            }
            "import_data" => {
                if !is_internal {
                    warn!("Security: rejected unauthorized import_data from external web content");
                    return None;
                }
                let target = payload.target.unwrap_or_else(|| "google-chrome".to_string());
                Some(BrowserEvent::ImportData(target))
            }
            "open_gemini" => {
                if !is_authorized { return None; }
                Some(BrowserEvent::OpenGemini)
            }
            "google_login" => {
                if !is_internal {
                    warn!("Security: rejected unauthorized google_login from external web content");
                    return None;
                }
                Some(BrowserEvent::OpenGoogleLogin)
            }
            "toggle_window_controls" => {
                if !is_internal {
                    warn!("Security: rejected toggle_window_controls from external web content");
                    return None;
                }
                Some(BrowserEvent::ToggleWindowControls)
            }
            "toggle_tab_close" => {
                if !is_internal {
                    warn!("Security: rejected toggle_tab_close from external web content");
                    return None;
                }
                Some(BrowserEvent::ToggleTabCloseButtons)
            }
            "toggle_decorations" => {
                if !is_internal {
                    warn!("Security: rejected toggle_decorations from external web content");
                    return None;
                }
                Some(BrowserEvent::ToggleDecorations)
            }
            "reopen_closed_tab" => {
                if !is_authorized { return None; }
                Some(BrowserEvent::ReopenClosedTab)
            }
            "set_language" => {
                if !is_internal {
                    warn!("Security: rejected set_language from external web content");
                    return None;
                }
                payload.target.map(BrowserEvent::SetLanguage)
            }
            "minimize_window" => {
                if !is_internal { return None; }
                Some(BrowserEvent::MinimizeWindow)
            }
            "maximize_window" => {
                if !is_internal { return None; }
                Some(BrowserEvent::MaximizeWindow)
            }
            "close_window" => {
                if !is_internal { return None; }
                Some(BrowserEvent::CloseWindow)
            }
            _ => None,
        }
    }

    pub fn dispatch_content_ipc_str(
        msg: &str,
        proxy: &EventLoopProxy<BrowserEvent>,
        is_internal: bool,
        secret_token: &str,
    ) {
        if let Some(event) = Self::authorize_and_parse_content_ipc(msg, is_internal, secret_token) {
            let _ = proxy.send_event(event);
        }
    }

    pub fn handle_event(&mut self, event: BrowserEvent) {
        match event {
            BrowserEvent::Navigate(raw_url) => {
                let resolved = Self::resolve_url(&raw_url);
                info!("Navigating to: {}", resolved);
                *self.current_active_url.lock().unwrap_or_else(|e| e.into_inner()) = resolved.clone();
                if let Err(e) = self.content_webview.load_url(&resolved) {
                    warn!("Failed to load URL {}: {}", resolved, e);
                }
                if let Some(tab) = self.tab_manager.active_tab_mut() {
                    tab.url = resolved;
                }
                self.sync_toolbar_state();
            }
            BrowserEvent::PageLoaded(url) => {
                info!("Page loaded: {}", url);
                *self.current_active_url.lock().unwrap_or_else(|e| e.into_inner()) = url.clone();
                if let Some(tab) = self.tab_manager.active_tab_mut() {
                    tab.url = url.clone();
                    tab.can_go_back = self.content_webview.can_go_back().unwrap_or(false);
                    tab.can_go_forward = self.content_webview.can_go_forward().unwrap_or(false);
                    // Record in history once when page load finishes
                    if let Err(e) = self.storage.record_history(&url, &tab.title) {
                        warn!("Failed to record history for {}: {}", url, e);
                    }
                }

                // Inject cosmetic ad blocking CSS if available
                let cosmetic_script = {
                    let s = self.shield.lock().unwrap_or_else(|e| e.into_inner());
                    s.generate_cosmetic_script(&url)
                };
                if let Some(script) = cosmetic_script {
                    let _ = self.content_webview.evaluate_script(&script);
                }

                self.sync_toolbar_state();
            }
            BrowserEvent::TitleChanged(title) => {
                if let Some(tab) = self.tab_manager.active_tab_mut() {
                    tab.title = title.clone();
                    let url = tab.url.clone();
                    if let Err(e) = self.storage.update_history_title(&url, &title) {
                        warn!("Failed to update history title for {}: {}", url, e);
                    }
                }
                let _ = self.window.set_title(&format!("{} - Valorex", title));
                self.sync_toolbar_state();
            }
            BrowserEvent::GoBack => {
                let _ = self.content_webview.go_back();
            }
            BrowserEvent::GoForward => {
                let _ = self.content_webview.go_forward();
            }
            BrowserEvent::Reload => {
                let _ = self.content_webview.reload();
            }
            BrowserEvent::NewTab(url) => {
                let resolved = Self::resolve_url(&url);
                let _id = self.tab_manager.new_tab(&resolved);
                *self.current_active_url.lock().unwrap_or_else(|e| e.into_inner()) = resolved.clone();
                let _ = self.content_webview.load_url(&resolved);
                self.sync_toolbar_state();
            }
            BrowserEvent::CloseTab(id) => {
                let target_id = if id == 0 {
                    self.tab_manager.active_id()
                } else {
                    id
                };
                let prev_active_id = self.tab_manager.active_id();
                let was_active = target_id == prev_active_id;
                if let Some(new_active_id) = self.tab_manager.close_tab(target_id) {
                    if was_active {
                        if let Some(tab) = self.tab_manager.tabs().iter().find(|t| t.id == new_active_id) {
                            *self.current_active_url.lock().unwrap_or_else(|e| e.into_inner()) = tab.url.clone();
                            let _ = self.content_webview.load_url(&tab.url);
                        }
                    }
                }
                self.sync_toolbar_state();
            }
            BrowserEvent::SwitchTab(id) => {
                if id == self.tab_manager.active_id() {
                    return;
                }
                if self.tab_manager.switch_tab(id) {
                    if let Some(tab) = self.tab_manager.active_tab() {
                        *self.current_active_url.lock().unwrap_or_else(|e| e.into_inner()) = tab.url.clone();
                        let _ = self.content_webview.load_url(&tab.url);
                    }
                }
                self.sync_toolbar_state();
            }
            BrowserEvent::ToggleShield(enabled_opt) => {
                let mut s = self.shield.lock().unwrap_or_else(|e| e.into_inner());
                let new_state = enabled_opt.unwrap_or(!s.is_enabled());
                s.set_enabled(new_state);
                info!("Valorex Shields toggled to: {}", new_state);
                drop(s);
                self.sync_toolbar_state();
            }
            BrowserEvent::ToggleBookmark => {
                if let Some(tab) = self.tab_manager.active_tab() {
                    let url = tab.url.clone();
                    let title = tab.title.clone();
                    if let Ok(is_bookmarked) = self.storage.is_bookmarked(&url) {
                        if is_bookmarked {
                            if let Err(e) = self.storage.delete_bookmark_by_url(&url) {
                                warn!("Failed to delete bookmark by url {}: {}", url, e);
                            }
                        } else {
                            if let Err(e) = self.storage.add_bookmark(&title, &url, None) {
                                warn!("Failed to add bookmark {}: {}", url, e);
                            }
                        }
                    }
                }
                self.sync_toolbar_state();
            }
            BrowserEvent::ClearHistory => {
                if let Err(e) = self.storage.clear_history() {
                    warn!("Failed to clear history database: {}", e);
                }
                let _ = self.content_webview.clear_all_browsing_data();
                self.tab_manager.clear_closed_tabs();
                info!("Cleared browsing data, history, and closed tabs cache.");
                self.sync_toolbar_state();
            }
            BrowserEvent::DeleteBookmark(id) => {
                if let Err(e) = self.storage.delete_bookmark(id) {
                    warn!("Failed to delete bookmark id {}: {}", id, e);
                }
                self.sync_toolbar_state();
            }
            BrowserEvent::ZoomIn => {
                self.zoom_level = (self.zoom_level + 0.1).min(3.0);
                let _ = self.content_webview.zoom(self.zoom_level);
                info!("Zoom level changed to: {:.1}x", self.zoom_level);
            }
            BrowserEvent::ZoomOut => {
                self.zoom_level = (self.zoom_level - 0.1).max(0.3);
                let _ = self.content_webview.zoom(self.zoom_level);
                info!("Zoom level changed to: {:.1}x", self.zoom_level);
            }
            BrowserEvent::ZoomReset => {
                self.zoom_level = 1.0;
                let _ = self.content_webview.zoom(1.0);
                info!("Zoom level reset to 1.0x");
            }
            BrowserEvent::DownloadStarted(url, path) => {
                info!("Download started for: {} -> {:?}", url, path);
            }
            BrowserEvent::DownloadCompleted(url, path_opt, success) => {
                let filename = path_opt.as_ref()
                    .and_then(|f| f.file_name())
                    .and_then(|f| f.to_str())
                    .unwrap_or("file");
                let path_str = path_opt.as_ref()
                    .map(|f| f.to_string_lossy().to_string())
                    .unwrap_or_default();
                let status = if success { "completed" } else { "failed" };
                if let Err(e) = self.storage.record_download_with_status(filename, &url, &path_str, status) {
                    warn!("Failed to record download in storage: {}", e);
                }
                if success {
                    info!("Download completed successfully: {}", path_str);
                } else {
                    warn!("Download failed for URL: {}", url);
                }
            }
            BrowserEvent::RulesUpdated => {
                if let Ok(updater) = crate::shield::updater::FilterUpdater::new() {
                    if let Ok(local_rules) = updater.load_local_rules() {
                        if !local_rules.is_empty() {
                            self.shield.lock().unwrap_or_else(|e| e.into_inner()).load_rules(local_rules.lines());
                            info!("Shield engine updated with fresh EasyList/EasyPrivacy rules.");
                        }
                    }
                }
            }
            BrowserEvent::ToggleDecorations => {
                self.decorations_enabled = !self.decorations_enabled;
                self.config.decorations_visible = self.decorations_enabled;
                if !self.config.incognito {
                    let _ = self.config.save();
                }
                self.window.set_decorations(self.decorations_enabled);
                info!("Window decorations toggled to: {}", self.decorations_enabled);
                self.sync_toolbar_state();
            }
            BrowserEvent::MinimizeWindow => {
                self.window.set_minimized(true);
            }
            BrowserEvent::MaximizeWindow => {
                let is_max = !self.window.is_maximized();
                self.window.set_maximized(is_max);
                self.sync_toolbar_state();
            }
            BrowserEvent::CloseWindow => {
                if !self.config.incognito {
                    let _ = self.config.save();
                }
                std::process::exit(0);
            }
            BrowserEvent::DragWindow => {
                let _ = self.window.drag_window();
            }
            BrowserEvent::ToggleFullscreen => {
                self.is_fullscreen = !self.is_fullscreen;
                let mode = if self.is_fullscreen {
                    Some(tao::window::Fullscreen::Borderless(None))
                } else {
                    None
                };
                self.window.set_fullscreen(mode);
                info!("Fullscreen toggled to: {}", self.is_fullscreen);
            }
            BrowserEvent::OpenMenu => {
                #[cfg(target_os = "linux")]
                {
                    let lang = self.current_language;
                    let menu = gtk::Menu::new();

                    let item_newtab = gtk::MenuItem::with_label(lang.new_tab());
                    let proxy_c = self.proxy.clone();
                    item_newtab.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::NewTab("valorex://newtab".into()));
                    });
                    menu.append(&item_newtab);

                    let item_reopen = gtk::MenuItem::with_label(lang.reopen_closed_tab());
                    let proxy_c = self.proxy.clone();
                    item_reopen.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::ReopenClosedTab);
                    });
                    menu.append(&item_reopen);

                    let item_history = gtk::MenuItem::with_label(lang.history());
                    let proxy_c = self.proxy.clone();
                    item_history.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::Navigate("valorex://history".into()));
                    });
                    menu.append(&item_history);

                    let item_bookmarks = gtk::MenuItem::with_label(lang.bookmarks());
                    let proxy_c = self.proxy.clone();
                    item_bookmarks.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::Navigate("valorex://bookmarks".into()));
                    });
                    menu.append(&item_bookmarks);

                    let item_downloads = gtk::MenuItem::with_label(lang.downloads());
                    let proxy_c = self.proxy.clone();
                    item_downloads.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::Navigate("valorex://downloads".into()));
                    });
                    menu.append(&item_downloads);

                    menu.append(&gtk::SeparatorMenuItem::new());

                    let item_import = gtk::MenuItem::with_label(lang.import_data());
                    let proxy_c = self.proxy.clone();
                    item_import.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::Navigate("valorex://settings".into()));
                    });
                    menu.append(&item_import);

                    let item_google = gtk::MenuItem::with_label(lang.google_login());
                    let proxy_c = self.proxy.clone();
                    item_google.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::OpenGoogleLogin);
                    });
                    menu.append(&item_google);

                    let item_gemini = gtk::MenuItem::with_label(lang.gemini_ai());
                    let proxy_c = self.proxy.clone();
                    item_gemini.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::OpenGemini);
                    });
                    menu.append(&item_gemini);

                    menu.append(&gtk::SeparatorMenuItem::new());

                    let item_win_controls = gtk::CheckMenuItem::with_label(lang.toggle_window_controls());
                    item_win_controls.set_active(self.show_window_controls);
                    let proxy_c = self.proxy.clone();
                    item_win_controls.connect_toggled(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::ToggleWindowControls);
                    });
                    menu.append(&item_win_controls);

                    let item_tab_close = gtk::CheckMenuItem::with_label(lang.toggle_tab_close_btn());
                    item_tab_close.set_active(self.show_tab_close_buttons);
                    let proxy_c = self.proxy.clone();
                    item_tab_close.connect_toggled(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::ToggleTabCloseButtons);
                    });
                    menu.append(&item_tab_close);

                    let item_decor = gtk::CheckMenuItem::with_label(lang.toggle_system_titlebar());
                    item_decor.set_active(self.decorations_enabled);
                    let proxy_c = self.proxy.clone();
                    item_decor.connect_toggled(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::ToggleDecorations);
                    });
                    menu.append(&item_decor);

                    menu.append(&gtk::SeparatorMenuItem::new());

                    let item_lang = gtk::MenuItem::with_label(lang.switch_language());
                    let proxy_c = self.proxy.clone();
                    let target_code = if lang.is_rtl() { "en" } else { "ar" };
                    item_lang.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::SetLanguage(target_code.into()));
                    });
                    menu.append(&item_lang);

                    let item_settings = gtk::MenuItem::with_label(lang.settings());
                    let proxy_c = self.proxy.clone();
                    item_settings.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::Navigate("valorex://settings".into()));
                    });
                    menu.append(&item_settings);

                    menu.append(&gtk::SeparatorMenuItem::new());

                    let item_exit = gtk::MenuItem::with_label(lang.exit_browser());
                    let proxy_c = self.proxy.clone();
                    item_exit.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::CloseWindow);
                    });
                    menu.append(&item_exit);

                    let gtk_win = self.window.gtk_window();
                    menu.show_all();
                    menu.popup_at_widget(gtk_win.upcast_ref::<gtk::Widget>(), gtk::gdk::Gravity::NorthEast, gtk::gdk::Gravity::NorthEast, None);
                }
            }
            BrowserEvent::OpenTabContextMenu(tab_id, _coords) => {
                #[cfg(target_os = "linux")]
                {
                    let lang = self.current_language;
                    let menu = gtk::Menu::new();

                    let item_newtab = gtk::MenuItem::with_label(lang.new_tab());
                    let proxy_c = self.proxy.clone();
                    item_newtab.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::NewTab("valorex://newtab".into()));
                    });
                    menu.append(&item_newtab);

                    let item_reload = gtk::MenuItem::with_label(lang.reload());
                    let proxy_c = self.proxy.clone();
                    item_reload.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::Reload);
                    });
                    menu.append(&item_reload);

                    let item_dup = gtk::MenuItem::with_label(lang.duplicate_tab());
                    let proxy_c = self.proxy.clone();
                    item_dup.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::DuplicateTab(tab_id));
                    });
                    menu.append(&item_dup);

                    let is_pinned = self.tab_manager.tabs().iter().find(|t| t.id == tab_id).map(|t| t.is_pinned).unwrap_or(false);
                    let pin_label = if is_pinned { lang.unpin_tab() } else { lang.pin_tab() };
                    let item_pin = gtk::MenuItem::with_label(pin_label);
                    let proxy_c = self.proxy.clone();
                    item_pin.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::PinTab(tab_id));
                    });
                    menu.append(&item_pin);

                    let item_reopen = gtk::MenuItem::with_label(lang.reopen_closed_tab());
                    let proxy_c = self.proxy.clone();
                    item_reopen.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::ReopenClosedTab);
                    });
                    menu.append(&item_reopen);

                    menu.append(&gtk::SeparatorMenuItem::new());

                    let item_close = gtk::MenuItem::with_label(lang.close_tab());
                    let proxy_c = self.proxy.clone();
                    item_close.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::CloseTab(tab_id));
                    });
                    menu.append(&item_close);

                    let item_close_others = gtk::MenuItem::with_label(lang.close_other_tabs());
                    let proxy_c = self.proxy.clone();
                    item_close_others.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::CloseOtherTabs(tab_id));
                    });
                    menu.append(&item_close_others);

                    let item_close_right = gtk::MenuItem::with_label(lang.close_tabs_to_right());
                    let proxy_c = self.proxy.clone();
                    item_close_right.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::CloseTabsToRight(tab_id));
                    });
                    menu.append(&item_close_right);

                    menu.append(&gtk::SeparatorMenuItem::new());

                    let item_win_controls = gtk::CheckMenuItem::with_label(lang.toggle_window_controls());
                    item_win_controls.set_active(self.show_window_controls);
                    let proxy_c = self.proxy.clone();
                    item_win_controls.connect_toggled(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::ToggleWindowControls);
                    });
                    menu.append(&item_win_controls);

                    let item_tab_close = gtk::CheckMenuItem::with_label(lang.toggle_tab_close_btn());
                    item_tab_close.set_active(self.show_tab_close_buttons);
                    let proxy_c = self.proxy.clone();
                    item_tab_close.connect_toggled(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::ToggleTabCloseButtons);
                    });
                    menu.append(&item_tab_close);

                    let item_decor = gtk::CheckMenuItem::with_label(lang.toggle_system_titlebar());
                    item_decor.set_active(self.decorations_enabled);
                    let proxy_c = self.proxy.clone();
                    item_decor.connect_toggled(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::ToggleDecorations);
                    });
                    menu.append(&item_decor);

                    let gtk_win = self.window.gtk_window();
                    menu.show_all();
                    menu.popup_at_widget(gtk_win.upcast_ref::<gtk::Widget>(), gtk::gdk::Gravity::NorthWest, gtk::gdk::Gravity::NorthWest, None);
                }
            }
            BrowserEvent::OpenShieldMenu => {
                #[cfg(target_os = "linux")]
                {
                    let lang = self.current_language;
                    let menu = gtk::Menu::new();
                    let (enabled, count) = {
                        let s = self.shield.lock().unwrap_or_else(|e| e.into_inner());
                        (s.is_enabled(), s.get_blocked_count())
                    };

                    let shield_title = if lang.is_rtl() { "درع الحماية والخصوصية" } else { "Shield Protection" };
                    let item_status = gtk::CheckMenuItem::with_label(shield_title);
                    item_status.set_active(enabled);
                    let proxy_c = self.proxy.clone();
                    item_status.connect_toggled(move |item| {
                        let _ = proxy_c.send_event(BrowserEvent::ToggleShield(Some(item.is_active())));
                    });
                    menu.append(&item_status);

                    let count_label = if lang.is_rtl() {
                        format!("العناصر المحجوبة: {}", count)
                    } else {
                        format!("Trackers & Ads Blocked: {}", count)
                    };
                    let item_count = gtk::MenuItem::with_label(&count_label);
                    item_count.set_sensitive(false);
                    menu.append(&item_count);

                    menu.append(&gtk::SeparatorMenuItem::new());

                    let settings_label = if lang.is_rtl() { "فتح إعدادات الدرع والقواعد" } else { "Open Shield Settings" };
                    let item_settings = gtk::MenuItem::with_label(settings_label);
                    let proxy_c = self.proxy.clone();
                    item_settings.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::Navigate("valorex://settings".into()));
                    });
                    menu.append(&item_settings);

                    let gtk_win = self.window.gtk_window();
                    menu.show_all();
                    menu.popup_at_widget(gtk_win.upcast_ref::<gtk::Widget>(), gtk::gdk::Gravity::NorthEast, gtk::gdk::Gravity::NorthEast, None);
                }
            }
            BrowserEvent::DuplicateTab(id) => {
                if let Some(new_id) = self.tab_manager.duplicate_tab(id) {
                    if let Some(tab) = self.tab_manager.tabs().iter().find(|t| t.id == new_id) {
                        *self.current_active_url.lock().unwrap_or_else(|e| e.into_inner()) = tab.url.clone();
                        let _ = self.content_webview.load_url(&tab.url);
                    }
                    self.sync_toolbar_state();
                }
            }
            BrowserEvent::PinTab(id) => {
                let _ = self.tab_manager.toggle_pin_tab(id);
                self.sync_toolbar_state();
            }
            BrowserEvent::CloseOtherTabs(id) => {
                let prev_active_id = self.tab_manager.active_id();
                self.tab_manager.close_other_tabs(id);
                if prev_active_id != id {
                    if let Some(tab) = self.tab_manager.active_tab() {
                        *self.current_active_url.lock().unwrap_or_else(|e| e.into_inner()) = tab.url.clone();
                        let _ = self.content_webview.load_url(&tab.url);
                    }
                }
                self.sync_toolbar_state();
            }
            BrowserEvent::ReorderTabs(from_id, to_id) => {
                self.tab_manager.reorder_tabs(from_id, to_id);
                self.sync_toolbar_state();
            }
            BrowserEvent::NextTab => {
                let new_active_id = self.tab_manager.next_tab();
                if let Some(tab) = self.tab_manager.tabs().iter().find(|t| t.id == new_active_id) {
                    *self.current_active_url.lock().unwrap_or_else(|e| e.into_inner()) = tab.url.clone();
                    let _ = self.content_webview.load_url(&tab.url);
                }
                self.sync_toolbar_state();
            }
            BrowserEvent::PrevTab => {
                let new_active_id = self.tab_manager.prev_tab();
                if let Some(tab) = self.tab_manager.tabs().iter().find(|t| t.id == new_active_id) {
                    *self.current_active_url.lock().unwrap_or_else(|e| e.into_inner()) = tab.url.clone();
                    let _ = self.content_webview.load_url(&tab.url);
                }
                self.sync_toolbar_state();
            }
            BrowserEvent::SwitchTabIndex(index) => {
                let prev_active_id = self.tab_manager.active_id();
                if let Some(new_active_id) = self.tab_manager.switch_to_index(index) {
                    if prev_active_id != new_active_id {
                        if let Some(tab) = self.tab_manager.tabs().iter().find(|t| t.id == new_active_id) {
                            *self.current_active_url.lock().unwrap_or_else(|e| e.into_inner()) = tab.url.clone();
                            let _ = self.content_webview.load_url(&tab.url);
                        }
                    }
                    self.sync_toolbar_state();
                }
            }
            BrowserEvent::ImportData(browser_id) => {
                info!("Starting background import from: {}", browser_id);
                let storage_clone = self.storage.clone();
                let proxy_clone = self.proxy.clone();
                let browser_id_clone = browser_id.clone();
                std::thread::spawn(move || {
                    let summary = crate::importer::BrowserImporter::import_from_browser(&browser_id_clone, &storage_clone);
                    let _ = proxy_clone.send_event(BrowserEvent::ImportFinished(summary));
                });
            }
            BrowserEvent::ImportFinished(summary) => {
                info!("Import finished: {} bookmarks, {} history items", summary.bookmarks_imported, summary.history_imported);
                let json = serde_json::to_string(&summary).unwrap_or_default();
                let script = format!("if (window.valorexOnImportFinished) {{ window.valorexOnImportFinished({}); }}", json);
                let _ = self.toolbar_webview.evaluate_script(&script);
                let _ = self.content_webview.evaluate_script(&script);
                self.sync_toolbar_state();
            }
            BrowserEvent::SetLanguage(lang_code) => {
                self.current_language = crate::i18n::Language::from_code(&lang_code);
                *self.shared_lang.lock().unwrap_or_else(|e| e.into_inner()) = self.current_language;
                self.config.language = self.current_language.code().to_string();
                if !self.config.incognito {
                    let _ = self.config.save();
                }
                info!("Language changed to: {:?}", self.current_language);
                if let Some(tab) = self.tab_manager.active_tab() {
                    if tab.url.starts_with("valorex://") {
                        let _ = self.content_webview.load_url(&tab.url);
                    }
                }
                self.sync_toolbar_state();
            }
            BrowserEvent::OpenGemini => {
                let _ = self.handle_event(BrowserEvent::NewTab("https://gemini.google.com".into()));
            }
            BrowserEvent::SummarizePage => {
                let js = r#"
                    if (window.valorexTriggerSummary) {
                        window.valorexTriggerSummary();
                    }
                "#;
                let _ = self.toolbar_webview.evaluate_script(js);
            }
            BrowserEvent::OpenGoogleLogin => {
                let _ = self.handle_event(BrowserEvent::NewTab("https://accounts.google.com/".into()));
            }
            BrowserEvent::ToggleWindowControls => {
                self.show_window_controls = !self.show_window_controls;
                self.config.window_controls_visible = self.show_window_controls;
                if !self.config.incognito {
                    let _ = self.config.save();
                }
                info!("Toggled window caption controls to: {}", self.show_window_controls);
                self.sync_toolbar_state();
            }
            BrowserEvent::ToggleTabCloseButtons => {
                self.show_tab_close_buttons = !self.show_tab_close_buttons;
                self.config.tab_close_visible = self.show_tab_close_buttons;
                if !self.config.incognito {
                    let _ = self.config.save();
                }
                info!("Toggled tab close buttons to: {}", self.show_tab_close_buttons);
                self.sync_toolbar_state();
            }
            BrowserEvent::CloseTabsToRight(id) => {
                let prev_active_id = self.tab_manager.active_id();
                self.tab_manager.close_tabs_to_right(id);
                let new_active_id = self.tab_manager.active_id();
                if prev_active_id != new_active_id {
                    if let Some(tab) = self.tab_manager.active_tab() {
                        *self.current_active_url.lock().unwrap_or_else(|e| e.into_inner()) = tab.url.clone();
                        let _ = self.content_webview.load_url(&tab.url);
                    }
                }
                self.sync_toolbar_state();
            }
            BrowserEvent::ReopenClosedTab => {
                if let Some(new_id) = self.tab_manager.reopen_closed_tab() {
                    if let Some(tab) = self.tab_manager.tabs().iter().find(|t| t.id == new_id) {
                        *self.current_active_url.lock().unwrap_or_else(|e| e.into_inner()) = tab.url.clone();
                        let _ = self.content_webview.load_url(&tab.url);
                    }
                    self.sync_toolbar_state();
                }
            }
            BrowserEvent::OpenTabBarContextMenu(_coords) => {
                #[cfg(target_os = "linux")]
                {
                    let lang = self.current_language;
                    let menu = gtk::Menu::new();

                    let item_newtab = gtk::MenuItem::with_label(lang.new_tab());
                    let proxy_c = self.proxy.clone();
                    item_newtab.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::NewTab("valorex://newtab".into()));
                    });
                    menu.append(&item_newtab);

                    let item_reopen = gtk::MenuItem::with_label(lang.reopen_closed_tab());
                    let proxy_c = self.proxy.clone();
                    item_reopen.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::ReopenClosedTab);
                    });
                    menu.append(&item_reopen);

                    menu.append(&gtk::SeparatorMenuItem::new());

                    let item_win_controls = gtk::CheckMenuItem::with_label(lang.toggle_window_controls());
                    item_win_controls.set_active(self.show_window_controls);
                    let proxy_c = self.proxy.clone();
                    item_win_controls.connect_toggled(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::ToggleWindowControls);
                    });
                    menu.append(&item_win_controls);

                    let item_tab_close = gtk::CheckMenuItem::with_label(lang.toggle_tab_close_btn());
                    item_tab_close.set_active(self.show_tab_close_buttons);
                    let proxy_c = self.proxy.clone();
                    item_tab_close.connect_toggled(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::ToggleTabCloseButtons);
                    });
                    menu.append(&item_tab_close);

                    let item_decor = gtk::CheckMenuItem::with_label(lang.toggle_system_titlebar());
                    item_decor.set_active(self.decorations_enabled);
                    let proxy_c = self.proxy.clone();
                    item_decor.connect_toggled(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::ToggleDecorations);
                    });
                    menu.append(&item_decor);

                    menu.append(&gtk::SeparatorMenuItem::new());

                    let item_settings = gtk::MenuItem::with_label(lang.settings());
                    let proxy_c = self.proxy.clone();
                    item_settings.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::Navigate("valorex://settings".into()));
                    });
                    menu.append(&item_settings);

                    let gtk_win = self.window.gtk_window();
                    menu.show_all();
                    menu.popup_at_widget(gtk_win.upcast_ref::<gtk::Widget>(), gtk::gdk::Gravity::NorthWest, gtk::gdk::Gravity::NorthWest, None);
                }
            }
        }
    }

    pub fn sync_toolbar_state(&self) {
        let (current_url, can_back, can_forward) = if let Some(tab) = self.tab_manager.active_tab() {
            (tab.url.clone(), tab.can_go_back, tab.can_go_forward)
        } else {
            ("valorex://newtab".to_string(), false, false)
        };

        let is_bookmarked = self.storage.is_bookmarked(&current_url).unwrap_or(false);
        let (shield_enabled, blocked_count) = {
            let s = self.shield.lock().unwrap_or_else(|e| e.into_inner());
            (s.is_enabled(), s.get_blocked_count())
        };

        let is_secure = current_url.starts_with("https://") || current_url.starts_with("valorex://");
        let tabs_json = serde_json::Value::Array(self.tab_manager.get_tab_json_list());

        let payload = serde_json::json!({
            "url": current_url,
            "can_go_back": can_back,
            "can_go_forward": can_forward,
            "blocked_count": blocked_count,
            "shields_enabled": shield_enabled,
            "is_bookmarked": is_bookmarked,
            "is_secure": is_secure,
            "decorations_enabled": self.decorations_enabled,
            "show_window_controls": self.show_window_controls,
            "show_tab_close_buttons": self.show_tab_close_buttons,
            "is_maximized": self.window.is_maximized(),
            "language": self.current_language.code(),
            "tabs": tabs_json,
        });

        let script = format!("if (window.valorexUpdate) {{ window.valorexUpdate({}); }}", payload);
        let _ = self.toolbar_webview.evaluate_script(&script);
    }

    pub fn resolve_url(raw: &str) -> String {
        let trimmed = raw.trim();
        let trimmed_lower = trimmed.to_lowercase();
        if trimmed_lower.starts_with("valorex://") {
            trimmed.to_string()
        } else if trimmed_lower.starts_with("http://") || trimmed_lower.starts_with("https://") || trimmed_lower.starts_with("file://") {
            trimmed.to_string()
        } else if trimmed_lower == "localhost"
            || trimmed_lower.starts_with("localhost:")
            || trimmed_lower.starts_with("localhost/")
            || trimmed_lower == "127.0.0.1"
            || trimmed_lower.starts_with("127.0.0.1:")
            || trimmed_lower.starts_with("127.0.0.1/") {
            format!("http://{}", trimmed)
        } else if trimmed.contains('.') && !trimmed.contains(' ') {
            if trimmed.starts_with('.') || trimmed.ends_with('.') {
                let encoded: String = percent_encoding::utf8_percent_encode(trimmed, percent_encoding::NON_ALPHANUMERIC).to_string();
                format!("https://www.google.com/search?q={}", encoded)
            } else {
                format!("https://{}", trimmed)
            }
        } else {
            let encoded: String = percent_encoding::utf8_percent_encode(trimmed, percent_encoding::NON_ALPHANUMERIC).to_string();
            format!("https://www.google.com/search?q={}", encoded)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_url_schemes() {
        assert_eq!(BrowserWindow::resolve_url("valorex://newtab"), "valorex://newtab");
        assert_eq!(BrowserWindow::resolve_url("https://example.com"), "https://example.com");
        assert_eq!(BrowserWindow::resolve_url("http://example.com"), "http://example.com");
        assert_eq!(BrowserWindow::resolve_url("file:///tmp/test.html"), "file:///tmp/test.html");
    }

    #[test]
    fn test_resolve_url_domain() {
        assert_eq!(BrowserWindow::resolve_url("github.com"), "https://github.com");
        assert_eq!(BrowserWindow::resolve_url("crates.io"), "https://crates.io");
    }

    #[test]
    fn test_resolve_url_localhost_and_casing() {
        assert_eq!(BrowserWindow::resolve_url("localhost"), "http://localhost");
        assert_eq!(BrowserWindow::resolve_url("localhost:3000"), "http://localhost:3000");
        assert_eq!(BrowserWindow::resolve_url("localhost/dashboard"), "http://localhost/dashboard");
        assert_eq!(BrowserWindow::resolve_url("127.0.0.1:8080"), "http://127.0.0.1:8080");
        assert_eq!(BrowserWindow::resolve_url("HTTPS://EXAMPLE.COM"), "HTTPS://EXAMPLE.COM");
        assert_eq!(BrowserWindow::resolve_url(".."), "https://www.google.com/search?q=%2E%2E");
    }

    #[test]
    fn test_resolve_url_search_queries() {
        assert_eq!(
            BrowserWindow::resolve_url("rust programming"),
            "https://www.google.com/search?q=rust%20programming"
        );
        let arabic_res = BrowserWindow::resolve_url("متصفح فالوركس");
        assert!(arabic_res.starts_with("https://www.google.com/search?q="));
        assert!(arabic_res.contains("%D9%85%D8%AA%D8%B5%D9%81%D8%AD"));

        let symbol_res = BrowserWindow::resolve_url("c++ & rust #code");
        // Verify # and & are percent-encoded
        assert!(!symbol_res["https://www.google.com/search?q=".len()..].contains('&'));
        assert!(!symbol_res["https://www.google.com/search?q=".len()..].contains('#'));
    }

    #[test]
    fn test_ipc_privilege_separation_external_rejected() {
        let token = "test_secret_token_123456";

        // Even with a valid token, external callers (is_internal = false) MUST be rejected for privileged admin actions
        let clear_req = format!(r#"{{"action":"clear_history","token":"{}"}}"#, token);
        assert!(BrowserWindow::authorize_and_parse_content_ipc(&clear_req, false, token).is_none());

        let del_bm_req = format!(r#"{{"action":"delete_bookmark","id":42,"token":"{}"}}"#, token);
        assert!(BrowserWindow::authorize_and_parse_content_ipc(&del_bm_req, false, token).is_none());

        let import_req = format!(r#"{{"action":"import_data","target":"brave","token":"{}"}}"#, token);
        assert!(BrowserWindow::authorize_and_parse_content_ipc(&import_req, false, token).is_none());

        let login_req = format!(r#"{{"action":"google_login","token":"{}"}}"#, token);
        assert!(BrowserWindow::authorize_and_parse_content_ipc(&login_req, false, token).is_none());

        let lang_req = format!(r#"{{"action":"set_language","target":"en","token":"{}"}}"#, token);
        assert!(BrowserWindow::authorize_and_parse_content_ipc(&lang_req, false, token).is_none());
    }

    #[test]
    fn test_ipc_privileged_internal_accepted() {
        let token = "test_secret_token_123456";

        // Internal caller (is_internal = true) is authorized for admin actions
        let clear_req = r#"{"action":"clear_history"}"#;
        match BrowserWindow::authorize_and_parse_content_ipc(clear_req, true, token) {
            Some(BrowserEvent::ClearHistory) => {}
            other => panic!("Expected ClearHistory event, got: {:?}", other),
        }

        let del_bm_req = r#"{"action":"delete_bookmark","id":42}"#;
        match BrowserWindow::authorize_and_parse_content_ipc(del_bm_req, true, token) {
            Some(BrowserEvent::DeleteBookmark(42)) => {}
            other => panic!("Expected DeleteBookmark(42), got: {:?}", other),
        }

        let import_req = r#"{"action":"import_data","target":"firefox"}"#;
        match BrowserWindow::authorize_and_parse_content_ipc(import_req, true, token) {
            Some(BrowserEvent::ImportData(t)) => assert_eq!(t, "firefox"),
            other => panic!("Expected ImportData(firefox), got: {:?}", other),
        }
    }

    #[test]
    fn test_ipc_navigate_scheme_blocking() {
        let token = "test_secret_token_123456";

        // Dangerous / privileged schemes blocked for external callers
        let dangerous_schemes = [
            "file:///etc/passwd",
            "FILE:///home/user/.ssh/id_rsa",
            "valorex://history",
            "VALOREX://bookmarks",
            "valorex-action://%7B%22action%22%3A%22clear_history%22%7D",
            "javascript:alert(document.cookie)",
            "data:text/html,<h1>XSS</h1>",
        ];

        for url in dangerous_schemes {
            let msg = format!(r#"{{"action":"navigate","url":"{}","token":"{}"}}"#, url, token);
            assert!(
                BrowserWindow::authorize_and_parse_content_ipc(&msg, false, token).is_none(),
                "Expected navigation to '{}' from external origin to be blocked",
                url
            );
        }

        // Safe web navigation allowed with valid token
        let safe_msg = format!(r#"{{"action":"navigate","url":"https://rust-lang.org","token":"{}"}}"#, token);
        match BrowserWindow::authorize_and_parse_content_ipc(&safe_msg, false, token) {
            Some(BrowserEvent::Navigate(u)) => assert_eq!(u, "https://rust-lang.org"),
            other => panic!("Expected Navigate to https://rust-lang.org, got: {:?}", other),
        }

        // Internal caller navigating to valorex:// allowed
        let internal_valorex_msg = r#"{"action":"navigate","url":"valorex://history"}"#;
        match BrowserWindow::authorize_and_parse_content_ipc(internal_valorex_msg, true, token) {
            Some(BrowserEvent::Navigate(u)) => assert_eq!(u, "valorex://history"),
            other => panic!("Expected Navigate to valorex://history, got: {:?}", other),
        }
    }

    #[test]
    fn test_ipc_token_validation() {
        let token = "secret_valid_token_xyz";
        let invalid_token = "attacker_fake_token_abc";

        let new_tab_valid = format!(r#"{{"action":"new_tab","token":"{}"}}"#, token);
        assert!(matches!(
            BrowserWindow::authorize_and_parse_content_ipc(&new_tab_valid, false, token),
            Some(BrowserEvent::NewTab(_))
        ));

        let new_tab_invalid = format!(r#"{{"action":"new_tab","token":"{}"}}"#, invalid_token);
        assert!(BrowserWindow::authorize_and_parse_content_ipc(&new_tab_invalid, false, token).is_none());

        let new_tab_missing = r#"{"action":"new_tab"}"#;
        assert!(BrowserWindow::authorize_and_parse_content_ipc(new_tab_missing, false, token).is_none());
    }
}

