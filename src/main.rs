use std::env;
use std::sync::{Arc, Mutex};
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
};
use valorex::{
    browser::{events::BrowserEvent, BrowserWindow},
    config::BrowserConfig,
    shield::ShieldEngine,
    storage::StorageEngine,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    println!("======================================================");
    println!("  Valorex Browser - Ultra-fast & Privacy-First (Rust) ");
    println!("======================================================");

    let event_loop = EventLoopBuilder::<BrowserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    let mut config = BrowserConfig::default();
    if let Some(target_url) = env::args().nth(1) {
        config.homepage = target_url;
    }

    let shield = Arc::new(Mutex::new(ShieldEngine::new()));
    let storage = StorageEngine::new()?;

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
            _ => (),
        }
    });
}
