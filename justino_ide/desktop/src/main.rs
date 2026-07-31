#![windows_subsystem = "windows"] // Hides the CLI console on Windows!

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::webview::WebViewBuilder;

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    
    // Create the Native OS Window
    let window = WindowBuilder::new()
        .with_title("Justino Studio")
        .with_inner_size(tao::dpi::LogicalSize::new(1440.0, 900.0))
        .build(&event_loop)
        .unwrap();

    // Determine the absolute path to our UI folder (in a real production app this would be embedded)
    let current_dir = std::env::current_dir().unwrap();
    
    // As we are inside justino_ide/desktop, the UI is in justino_ide/ui
    let ui_dir = current_dir.parent().unwrap().join("ui");
    let index_html_path = ui_dir.join("index.html");
    
    let url = format!("file:///{}", index_html_path.display()).replace("\\", "/");

    // Initialize the WebView pointing to our HTML
    let _webview = WebViewBuilder::new(window)?
        .with_url(&url)?
        .build()?;

    // Native App Event Loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
