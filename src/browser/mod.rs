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

use std::path::PathBuf;

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
}

impl BrowserWindow {
    pub fn new(
        event_loop: &EventLoopWindowTarget<BrowserEvent>,
        proxy: EventLoopProxy<BrowserEvent>,
        config: BrowserConfig,
        shield: Arc<Mutex<ShieldEngine>>,
        storage: StorageEngine,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let window = WindowBuilder::new()
            .with_title("Valorex")
            .with_inner_size(LogicalSize::new(1280.0, 800.0))
            .with_min_inner_size(LogicalSize::new(450.0, 350.0))
            .with_decorations(false)
            .build(event_loop)?;

        let initial_url = config.homepage.clone();
        info!("Launching Valorex webview with initial homepage: {}", initial_url);

        // --- Toolbar Webview Builder ---
        let proxy_ipc = proxy.clone();
        let toolbar_builder = WebViewBuilder::new()
            .with_html(CHROME_HTML)
            .with_ipc_handler(move |req: wry::http::Request<String>| {
                #[derive(serde::Deserialize)]
                struct IpcPayload {
                    action: String,
                    url: Option<String>,
                    id: Option<usize>,
                    from_id: Option<usize>,
                    to_id: Option<usize>,
                    index: Option<usize>,
                    enabled: Option<bool>,
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
                            if let Some(id) = payload.id {
                                let _ = proxy_ipc.send_event(BrowserEvent::OpenTabContextMenu(id));
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
                        _ => {}
                    }
                }
            });

        // --- Content Webview Builder ---
        let proxy_load = proxy.clone();
        let on_page_load = move |event: PageLoadEvent, url: String| {
            if let PageLoadEvent::Finished = event {
                let _ = proxy_load.send_event(BrowserEvent::PageLoaded(url));
            }
        };

        let proxy_title = proxy.clone();
        let on_title = move |title: String| {
            let _ = proxy_title.send_event(BrowserEvent::TitleChanged(title));
        };

        let shield_for_nav = shield.clone();
        let on_navigate = move |url: String| -> bool {
            let mut s = shield_for_nav.lock().unwrap();
            if s.should_block(&url, &url, "document") {
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
            let mut download_dir = dirs::download_dir().unwrap_or_else(|| PathBuf::from("/home/tarek/Downloads"));
            download_dir.push(filename);
            *path = download_dir.clone();
            let _ = proxy_dl_start.send_event(BrowserEvent::DownloadStarted(url, path.clone()));
            true
        };

        let proxy_dl_comp = proxy.clone();
        let on_dl_comp = move |url: String, path: Option<PathBuf>, success: bool| {
            let _ = proxy_dl_comp.send_event(BrowserEvent::DownloadCompleted(url, path, success));
        };

        let blocked_counter = shield.lock().unwrap().blocked_counter();
        let storage_for_protocol = storage.clone();
        let proxy_content_ipc = proxy.clone();
        let shortcuts_script = r#"
            window.addEventListener('keydown', function(e) {
                if (e.key === 'F11') {
                    e.preventDefault();
                    if (window.ipc) window.ipc.postMessage(JSON.stringify({ action: 'toggle_fullscreen' }));
                } else if (e.ctrlKey && e.key === 'Tab') {
                    e.preventDefault();
                    if (window.ipc) {
                        if (e.shiftKey) {
                            window.ipc.postMessage(JSON.stringify({ action: 'prev_tab' }));
                        } else {
                            window.ipc.postMessage(JSON.stringify({ action: 'next_tab' }));
                        }
                    }
                } else if (e.ctrlKey && !e.shiftKey && !e.altKey && e.key >= '1' && e.key <= '9') {
                    e.preventDefault();
                    if (window.ipc) window.ipc.postMessage(JSON.stringify({ action: 'switch_tab_index', index: parseInt(e.key) - 1 }));
                } else if (e.ctrlKey && !e.shiftKey && !e.altKey) {
                    if (e.key === 't' || e.key === 'T') {
                        e.preventDefault();
                        if (window.ipc) window.ipc.postMessage(JSON.stringify({ action: 'new_tab' }));
                    } else if (e.key === 'r' || e.key === 'R') {
                        e.preventDefault();
                        if (window.ipc) window.ipc.postMessage(JSON.stringify({ action: 'reload' }));
                    } else if (e.key === 'h' || e.key === 'H') {
                        e.preventDefault();
                        if (window.ipc) window.ipc.postMessage(JSON.stringify({ action: 'navigate', url: 'valorex://history' }));
                    } else if (e.key === 'b' || e.key === 'B') {
                        e.preventDefault();
                        if (window.ipc) window.ipc.postMessage(JSON.stringify({ action: 'navigate', url: 'valorex://bookmarks' }));
                    } else if (e.key === 'j' || e.key === 'J') {
                        e.preventDefault();
                        if (window.ipc) window.ipc.postMessage(JSON.stringify({ action: 'navigate', url: 'valorex://downloads' }));
                    } else if (e.key === '=' || e.key === '+') {
                        e.preventDefault();
                        if (window.ipc) window.ipc.postMessage(JSON.stringify({ action: 'zoom_in' }));
                    } else if (e.key === '-' || e.key === '_') {
                        e.preventDefault();
                        if (window.ipc) window.ipc.postMessage(JSON.stringify({ action: 'zoom_out' }));
                    } else if (e.key === '0') {
                        e.preventDefault();
                        if (window.ipc) window.ipc.postMessage(JSON.stringify({ action: 'zoom_reset' }));
                    }
                }
            }, true);
        "#;

        let content_builder = WebViewBuilder::new()
            .with_url(&initial_url)
            .with_devtools(true)
            .with_initialization_script(shortcuts_script)
            .with_ipc_handler(move |req: wry::http::Request<String>| {
                #[derive(serde::Deserialize)]
                struct ContentIpc {
                    action: String,
                    url: Option<String>,
                    id: Option<i64>,
                    index: Option<usize>,
                }
                let msg = req.body();
                if let Ok(payload) = serde_json::from_str::<ContentIpc>(msg) {
                    match payload.action.as_str() {
                        "navigate" => {
                            if let Some(url) = payload.url {
                                let _ = proxy_content_ipc.send_event(BrowserEvent::Navigate(url));
                            }
                        }
                        "clear_history" => {
                            let _ = proxy_content_ipc.send_event(BrowserEvent::ClearHistory);
                        }
                        "delete_bookmark" => {
                            if let Some(id) = payload.id {
                                let _ = proxy_content_ipc.send_event(BrowserEvent::DeleteBookmark(id));
                            }
                        }
                        "new_tab" => {
                            let _ = proxy_content_ipc.send_event(BrowserEvent::NewTab("valorex://newtab".into()));
                        }
                        "reload" => {
                            let _ = proxy_content_ipc.send_event(BrowserEvent::Reload);
                        }
                        "zoom_in" => {
                            let _ = proxy_content_ipc.send_event(BrowserEvent::ZoomIn);
                        }
                        "zoom_out" => {
                            let _ = proxy_content_ipc.send_event(BrowserEvent::ZoomOut);
                        }
                        "zoom_reset" => {
                            let _ = proxy_content_ipc.send_event(BrowserEvent::ZoomReset);
                        }
                        "toggle_fullscreen" => {
                            let _ = proxy_content_ipc.send_event(BrowserEvent::ToggleFullscreen);
                        }
                        "next_tab" => {
                            let _ = proxy_content_ipc.send_event(BrowserEvent::NextTab);
                        }
                        "prev_tab" => {
                            let _ = proxy_content_ipc.send_event(BrowserEvent::PrevTab);
                        }
                        "switch_tab_index" => {
                            if let Some(idx) = payload.index {
                                let _ = proxy_content_ipc.send_event(BrowserEvent::SwitchTabIndex(idx));
                            }
                        }
                        _ => {}
                    }
                }
            })
            .with_on_page_load_handler(on_page_load)
            .with_document_title_changed_handler(on_title)
            .with_navigation_handler(on_navigate)
            .with_download_started_handler(on_dl_start)
            .with_download_completed_handler(on_dl_comp)
            .with_custom_protocol("valorex".into(), move |_id, req| {
                use std::sync::atomic::Ordering;
                let uri = req.uri().to_string();
                let body = if uri.contains("newtab") {
                    InternalPages::render_newtab(blocked_counter.load(Ordering::Relaxed))
                } else if uri.contains("history") {
                    InternalPages::render_history(&storage_for_protocol)
                } else if uri.contains("bookmarks") {
                    InternalPages::render_bookmarks(&storage_for_protocol)
                } else if uri.contains("downloads") {
                    InternalPages::render_downloads(&storage_for_protocol)
                } else if uri.contains("settings") {
                    InternalPages::render_settings()
                } else {
                    InternalPages::render_newtab(0)
                };

                http::Response::builder()
                    .header("content-type", "text/html; charset=utf-8")
                    .status(200)
                    .body(body.as_bytes().to_vec())
                    .unwrap()
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
            config,
            zoom_level: 1.0,
            is_fullscreen: false,
            decorations_enabled: false,
        };

        instance.sync_toolbar_state();

        Ok(instance)
    }

    pub fn handle_event(&mut self, event: BrowserEvent) {
        match event {
            BrowserEvent::Navigate(raw_url) => {
                let resolved = Self::resolve_url(&raw_url);
                info!("Navigating to: {}", resolved);
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
                if let Some(tab) = self.tab_manager.active_tab_mut() {
                    tab.url = url.clone();
                    tab.can_go_back = self.content_webview.can_go_back().unwrap_or(false);
                    tab.can_go_forward = self.content_webview.can_go_forward().unwrap_or(false);
                }

                // Inject cosmetic ad blocking CSS if available
                let cosmetic_script = {
                    let s = self.shield.lock().unwrap();
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
                    let _ = self.storage.record_history(&url, &title);
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
                let _ = self.content_webview.load_url(&resolved);
                self.sync_toolbar_state();
            }
            BrowserEvent::CloseTab(id) => {
                if let Some(new_active_id) = self.tab_manager.close_tab(id) {
                    if let Some(tab) = self.tab_manager.tabs().iter().find(|t| t.id == new_active_id) {
                        let _ = self.content_webview.load_url(&tab.url);
                    }
                }
                self.sync_toolbar_state();
            }
            BrowserEvent::SwitchTab(id) => {
                if self.tab_manager.switch_tab(id) {
                    if let Some(tab) = self.tab_manager.active_tab() {
                        let _ = self.content_webview.load_url(&tab.url);
                    }
                }
                self.sync_toolbar_state();
            }
            BrowserEvent::ToggleShield(enabled_opt) => {
                let mut s = self.shield.lock().unwrap();
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
                            // Find and delete
                            if let Ok(bookmarks) = self.storage.get_bookmarks() {
                                if let Some(found) = bookmarks.iter().find(|b| b.url == url) {
                                    let _ = self.storage.delete_bookmark(found.id);
                                }
                            }
                        } else {
                            let _ = self.storage.add_bookmark(&title, &url, None);
                        }
                    }
                }
                self.sync_toolbar_state();
            }
            BrowserEvent::ClearHistory => {
                let _ = self.storage.clear_history();
                let _ = self.content_webview.clear_all_browsing_data();
                info!("Cleared browsing data and history.");
            }
            BrowserEvent::DeleteBookmark(id) => {
                let _ = self.storage.delete_bookmark(id);
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
                if success {
                    if let Some(path) = path_opt {
                        let filename = path
                            .file_name()
                            .and_then(|f| f.to_str())
                            .unwrap_or("file");
                        let path_str = path.to_string_lossy().to_string();
                        let _ = self.storage.record_download(filename, &url, &path_str);
                        info!("Download completed successfully: {}", path_str);
                    }
                } else {
                    warn!("Download failed for URL: {}", url);
                }
            }
            BrowserEvent::RulesUpdated => {
                if let Ok(updater) = crate::shield::updater::FilterUpdater::new() {
                    if let Ok(local_rules) = updater.load_local_rules() {
                        if !local_rules.is_empty() {
                            self.shield.lock().unwrap().load_rules(local_rules.lines());
                            info!("Shield engine updated with fresh EasyList/EasyPrivacy rules.");
                        }
                    }
                }
            }
            BrowserEvent::ToggleDecorations => {
                self.decorations_enabled = !self.decorations_enabled;
                self.window.set_decorations(self.decorations_enabled);
                info!("Window decorations toggled to: {}", self.decorations_enabled);
                self.sync_toolbar_state();
            }
            BrowserEvent::MinimizeWindow => {
                self.window.set_minimized(true);
            }
            BrowserEvent::MaximizeWindow => {
                self.window.set_maximized(!self.window.is_maximized());
            }
            BrowserEvent::CloseWindow => {
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
                    let menu = gtk::Menu::new();

                    let item_newtab = gtk::MenuItem::with_label("New Tab (Ctrl+T)");
                    let proxy_c = self.proxy.clone();
                    item_newtab.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::NewTab("valorex://newtab".into()));
                    });
                    menu.append(&item_newtab);

                    let item_history = gtk::MenuItem::with_label("History (Ctrl+H)");
                    let proxy_c = self.proxy.clone();
                    item_history.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::Navigate("valorex://history".into()));
                    });
                    menu.append(&item_history);

                    let item_bookmarks = gtk::MenuItem::with_label("Bookmarks (Ctrl+B)");
                    let proxy_c = self.proxy.clone();
                    item_bookmarks.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::Navigate("valorex://bookmarks".into()));
                    });
                    menu.append(&item_bookmarks);

                    let item_downloads = gtk::MenuItem::with_label("Downloads (Ctrl+J)");
                    let proxy_c = self.proxy.clone();
                    item_downloads.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::Navigate("valorex://downloads".into()));
                    });
                    menu.append(&item_downloads);

                    menu.append(&gtk::SeparatorMenuItem::new());

                    let item_settings = gtk::MenuItem::with_label("Settings & Shields (إعدادات المتصفح)");
                    let proxy_c = self.proxy.clone();
                    item_settings.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::Navigate("valorex://settings".into()));
                    });
                    menu.append(&item_settings);

                    menu.append(&gtk::SeparatorMenuItem::new());

                    let item_decor = gtk::CheckMenuItem::with_label("Use System Title Bar (شريط عنوان النظام)");
                    item_decor.set_active(self.decorations_enabled);
                    let proxy_c = self.proxy.clone();
                    item_decor.connect_toggled(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::ToggleDecorations);
                    });
                    menu.append(&item_decor);

                    menu.append(&gtk::SeparatorMenuItem::new());

                    let item_exit = gtk::MenuItem::with_label("Exit Valorex");
                    item_exit.connect_activate(|_| {
                        std::process::exit(0);
                    });
                    menu.append(&item_exit);

                    menu.show_all();
                    menu.popup_easy(3, gtk::current_event_time());
                }
            }
            BrowserEvent::OpenTabContextMenu(tab_id) => {
                #[cfg(target_os = "linux")]
                {
                    let menu = gtk::Menu::new();

                    let item_newtab = gtk::MenuItem::with_label("New Tab (Ctrl+T)");
                    let proxy_c = self.proxy.clone();
                    item_newtab.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::NewTab("valorex://newtab".into()));
                    });
                    menu.append(&item_newtab);

                    let item_reload = gtk::MenuItem::with_label("Reload Tab (Ctrl+R)");
                    let proxy_c = self.proxy.clone();
                    item_reload.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::Reload);
                    });
                    menu.append(&item_reload);

                    let item_dup = gtk::MenuItem::with_label("Duplicate Tab");
                    let proxy_c = self.proxy.clone();
                    item_dup.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::DuplicateTab(tab_id));
                    });
                    menu.append(&item_dup);

                    let is_pinned = self.tab_manager.tabs().iter().find(|t| t.id == tab_id).map(|t| t.is_pinned).unwrap_or(false);
                    let pin_label = if is_pinned { "Unpin Tab" } else { "Pin Tab" };
                    let item_pin = gtk::MenuItem::with_label(pin_label);
                    let proxy_c = self.proxy.clone();
                    item_pin.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::PinTab(tab_id));
                    });
                    menu.append(&item_pin);

                    menu.append(&gtk::SeparatorMenuItem::new());

                    let item_close = gtk::MenuItem::with_label("Close Tab (Ctrl+W)");
                    let proxy_c = self.proxy.clone();
                    item_close.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::CloseTab(tab_id));
                    });
                    menu.append(&item_close);

                    let item_close_others = gtk::MenuItem::with_label("Close Other Tabs");
                    let proxy_c = self.proxy.clone();
                    item_close_others.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::CloseOtherTabs(tab_id));
                    });
                    menu.append(&item_close_others);

                    menu.append(&gtk::SeparatorMenuItem::new());

                    let item_decor = gtk::CheckMenuItem::with_label("Use System Title Bar (شريط عنوان النظام)");
                    item_decor.set_active(self.decorations_enabled);
                    let proxy_c = self.proxy.clone();
                    item_decor.connect_toggled(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::ToggleDecorations);
                    });
                    menu.append(&item_decor);

                    menu.show_all();
                    menu.popup_easy(3, gtk::current_event_time());
                }
            }
            BrowserEvent::OpenShieldMenu => {
                #[cfg(target_os = "linux")]
                {
                    let menu = gtk::Menu::new();
                    let (enabled, count) = {
                        let s = self.shield.lock().unwrap();
                        (s.is_enabled(), s.get_blocked_count())
                    };

                    let item_status = gtk::CheckMenuItem::with_label("Shield Protection (حماية الدرع)");
                    item_status.set_active(enabled);
                    let proxy_c = self.proxy.clone();
                    item_status.connect_toggled(move |item| {
                        let _ = proxy_c.send_event(BrowserEvent::ToggleShield(Some(item.is_active())));
                    });
                    menu.append(&item_status);

                    let item_count = gtk::MenuItem::with_label(&format!("Trackers & Ads Blocked: {}", count));
                    item_count.set_sensitive(false);
                    menu.append(&item_count);

                    menu.append(&gtk::SeparatorMenuItem::new());

                    let item_settings = gtk::MenuItem::with_label("Open Shield Settings & Rules (الإعدادات)");
                    let proxy_c = self.proxy.clone();
                    item_settings.connect_activate(move |_| {
                        let _ = proxy_c.send_event(BrowserEvent::Navigate("valorex://settings".into()));
                    });
                    menu.append(&item_settings);

                    menu.show_all();
                    menu.popup_easy(3, gtk::current_event_time());
                }
            }
            BrowserEvent::DuplicateTab(id) => {
                if let Some(new_id) = self.tab_manager.duplicate_tab(id) {
                    if let Some(tab) = self.tab_manager.tabs().iter().find(|t| t.id == new_id) {
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
                self.tab_manager.close_other_tabs(id);
                if let Some(tab) = self.tab_manager.active_tab() {
                    let _ = self.content_webview.load_url(&tab.url);
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
                    let _ = self.content_webview.load_url(&tab.url);
                }
                self.sync_toolbar_state();
            }
            BrowserEvent::PrevTab => {
                let new_active_id = self.tab_manager.prev_tab();
                if let Some(tab) = self.tab_manager.tabs().iter().find(|t| t.id == new_active_id) {
                    let _ = self.content_webview.load_url(&tab.url);
                }
                self.sync_toolbar_state();
            }
            BrowserEvent::SwitchTabIndex(index) => {
                if let Some(new_active_id) = self.tab_manager.switch_to_index(index) {
                    if let Some(tab) = self.tab_manager.tabs().iter().find(|t| t.id == new_active_id) {
                        let _ = self.content_webview.load_url(&tab.url);
                    }
                    self.sync_toolbar_state();
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
            let s = self.shield.lock().unwrap();
            (s.is_enabled(), s.get_blocked_count())
        };

        let tabs_json = serde_json::Value::Array(self.tab_manager.get_tab_json_list());

        let payload = serde_json::json!({
            "url": current_url,
            "can_go_back": can_back,
            "can_go_forward": can_forward,
            "blocked_count": blocked_count,
            "shields_enabled": shield_enabled,
            "is_bookmarked": is_bookmarked,
            "decorations_enabled": self.decorations_enabled,
            "tabs": tabs_json,
        });

        let script = format!("if (window.valorexUpdate) {{ window.valorexUpdate({}); }}", payload);
        let _ = self.toolbar_webview.evaluate_script(&script);
    }

    pub fn resolve_url(raw: &str) -> String {
        let trimmed = raw.trim();
        if trimmed.starts_with("valorex://") {
            trimmed.to_string()
        } else if trimmed.starts_with("http://") || trimmed.starts_with("https://") || trimmed.starts_with("file://") {
            trimmed.to_string()
        } else if trimmed.contains('.') && !trimmed.contains(' ') {
            format!("https://{}", trimmed)
        } else {
            let query = trimmed.replace(' ', "+");
            format!("https://www.google.com/search?q={}", query)
        }
    }
}
