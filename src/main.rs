use std::env;
use std::sync::{Arc, Mutex};
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
};
use valorex::{
    browser::{events::BrowserEvent, BrowserWindow},
    config::BrowserConfig,
    shield::{updater::FilterUpdater, ShieldEngine},
    storage::StorageEngine,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args: Vec<String> = env::args().collect();
    if args.iter().any(|a| a == "-v" || a == "--version") {
        println!("Valorex Browser v0.1.0");
        println!("Engine: Rust 1.85+, Wry (WebKitGTK 4.1), Brave Adblock 0.13");
        return Ok(());
    }

    if args.iter().any(|a| a == "-h" || a == "--help") {
        println!("Valorex - Ultra-fast, privacy-first native web browser in Rust");
        println!();
        println!("USAGE:");
        println!("    valorex [OPTIONS] [URL]");
        println!();
        println!("OPTIONS:");
        println!("    -i, --incognito         Open in private/incognito mode");
        println!("        --disable-shields   Disable built-in ad & tracker blocking");
        println!("    -v, --version           Print version information");
        println!("    -h, --help              Print help information");
        return Ok(());
    }

    println!("======================================================");
    println!("  Valorex Browser - Ultra-fast & Privacy-First (Rust) ");
    println!("======================================================");

    let event_loop = EventLoopBuilder::<BrowserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    let mut config = BrowserConfig::default();
    if args.iter().any(|a| a == "-i" || a == "--incognito") {
        config.shields_enabled = true;
        config.homepage = "valorex://newtab".to_string();
    }

    if args.iter().any(|a| a == "--disable-shields") {
        config.shields_enabled = false;
    }

    for arg in args.iter().skip(1) {
        if !arg.starts_with('-') {
            config.homepage = arg.clone();
            break;
        }
    }

    let shield = Arc::new(Mutex::new(ShieldEngine::new()));
    let storage = StorageEngine::new()?;

    // Spawn background worker to sync latest EasyList/EasyPrivacy filter rules
    let proxy_updater = proxy.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build();
        if let Ok(rt) = rt {
            rt.block_on(async move {
                if let Ok(updater) = FilterUpdater::new() {
                    if let Ok(count) = updater.update_filters().await {
                        if count > 0 {
                            let _ = proxy_updater.send_event(BrowserEvent::RulesUpdated);
                        }
                    }
                }
            });
        }
    });

    let mut browser_window = BrowserWindow::new(
        &event_loop,
        proxy.clone(),
        config,
        shield,
        storage,
    )?;

    let window_id = browser_window.window.id();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(browser_event) => {
                browser_window.handle_event(browser_event);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id: id,
                ..
            } if id == window_id => {
                println!("Valorex shutting down gracefully.");
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                window_id: id,
                ..
            } if id == window_id => {
                browser_window.sync_toolbar_state();
            }
            _ => (),
        }
    });
}
