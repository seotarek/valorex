use std::env;
use std::sync::{Arc, Mutex};
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use valorex::{
    browser::BrowserWindow,
    config::BrowserConfig,
    shield::ShieldEngine,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    println!("======================================================");
    println!("  Valorex Browser - Ultra-fast & Privacy-First (Rust) ");
    println!("======================================================");

    let event_loop = EventLoop::new();

    let mut config = BrowserConfig::default();
    if let Some(target_url) = env::args().nth(1) {
        config.homepage = target_url;
    }

    let shield = Arc::new(Mutex::new(ShieldEngine::new()));
    let browser_window = BrowserWindow::new(&event_loop, config, shield)?;

    let window_id = browser_window.window.id();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
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
