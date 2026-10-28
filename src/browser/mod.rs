use std::sync::{Arc, Mutex};
use tao::{
    dpi::LogicalSize,
    event_loop::EventLoopWindowTarget,
    window::{Window, WindowBuilder},
};
use wry::{WebView, WebViewBuilder};
use crate::config::BrowserConfig;
use crate::shield::ShieldEngine;
use log::info;

pub struct BrowserWindow {
    pub window: Window,
    pub webview: WebView,
    pub shield: Arc<Mutex<ShieldEngine>>,
    pub config: BrowserConfig,
}

impl BrowserWindow {
    pub fn new<T>(
        event_loop: &EventLoopWindowTarget<T>,
        config: BrowserConfig,
        shield: Arc<Mutex<ShieldEngine>>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let window = WindowBuilder::new()
            .with_title("Valorex")
            .with_inner_size(LogicalSize::new(1280.0, 800.0))
            .with_min_inner_size(LogicalSize::new(400.0, 300.0))
            .build(event_loop)?;

        let initial_url = config.homepage.clone();
        info!("Launching Valorex webview pointing to: {}", initial_url);

        let webview = WebViewBuilder::new()
            .with_url(&initial_url)
            .with_devtools(true)
            .build(&window)?;

        Ok(Self {
            window,
            webview,
            shield,
            config,
        })
    }

    pub fn navigate(&self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let valid_url = if !url.starts_with("http://") && !url.starts_with("https://") && !url.starts_with("file://") {
            format!("https://duckduckgo.com/?q={}", url)
        } else {
            url.to_string()
        };
        self.webview.load_url(&valid_url)?;
        Ok(())
    }
}
