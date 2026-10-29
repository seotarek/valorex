use log::info;
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

    let mut config = BrowserConfig::load();
    let is_incognito = args.iter().any(|a| a == "-i" || a == "--incognito");
    if is_incognito {
        config.incognito = true;
        config.shields_enabled = true;
    }

    if args.iter().any(|a| a == "--disable-shields") {
        config.shields_enabled = false;
    }

    let initial_target_url = args.iter().skip(1).find(|a| !a.starts_with('-')).map(|u| BrowserWindow::resolve_url(u));

    let shield = Arc::new(Mutex::new(ShieldEngine::new()));
    shield.lock().unwrap_or_else(|e| e.into_inner()).set_enabled(config.shields_enabled);

    // Immediately load offline cached filter rules at startup
    if let Ok(updater) = FilterUpdater::new() {
        if let Ok(local_rules) = updater.load_local_rules() {
            if !local_rules.is_empty() {
                shield.lock().unwrap_or_else(|e| e.into_inner()).load_rules(local_rules.lines());
                info!("Loaded offline cached filter rules into Shield engine at startup.");
            }
        }
    }

    let storage = if is_incognito {
        info!("Incognito mode active: using ephemeral in-memory storage.");
        StorageEngine::new_in_memory()?
    } else {
        StorageEngine::new()?
    };

    // Spawn background worker to sync latest EasyList/EasyPrivacy filter rules in normal mode
    if !is_incognito {
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
    }

    let mut browser_window = BrowserWindow::new(
        &event_loop,
        proxy.clone(),
        config,
        shield,
        storage,
        initial_target_url,
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
                if !browser_window.config.incognito {
                    let _ = browser_window.config.save();
                }
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
