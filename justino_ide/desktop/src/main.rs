#![windows_subsystem = "windows"]

use serde::Deserialize;
use std::process::Command;
use std::fs;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::webview::WebViewBuilder;

#[derive(Deserialize)]
struct IpcMessage {
    action: String,
    payload: String,
}

fn main() {
    // We use a custom event type (String) for the EventLoop so we can send IPC messages to it
    let event_loop = EventLoop::<String>::with_user_event();
    let proxy = event_loop.create_proxy();

    let window = WindowBuilder::new()
        .with_title("Justino Studio")
        .with_inner_size(tao::dpi::LogicalSize::new(1440.0, 900.0))
        .build(&event_loop)
        .unwrap();

    let current_dir = std::env::current_dir().unwrap();
    let ui_dir = current_dir.parent().unwrap().join("ui");
    let index_html_path = ui_dir.join("index.html");
    let url = format!("file:///{}", index_html_path.display()).replace("\\", "/");

    let webview = WebViewBuilder::new(window).unwrap()
        .with_url(&url).unwrap()
        .with_ipc_handler(move |_window, req_str| {
            // Forward the IPC string from JS to the main event loop
            let _ = proxy.send_event(req_str);
        })
        .build().unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(req_str) => {
                if let Ok(msg) = serde_json::from_str::<IpcMessage>(&req_str) {
                    if msg.action == "run_code" {
                        // 1. Save the code to a temporary file
                        let temp_file = "temp_run.jucode";
                        let _ = fs::write(temp_file, &msg.payload);

                        // 2. Execute the Justino Compiler
                        // Assuming justino_core is parallel to justino_ide
                        let compiler_path = "../../justino_core/Cargo.toml";
                        
                        let output = Command::new("cargo")
                            .args(["run", "--manifest-path", compiler_path, "--", temp_file])
                            .output();

                        match output {
                            Ok(out) => {
                                let stdout = String::from_utf8_lossy(&out.stdout).replace("`", "\\`").replace("\n", "\\n");
                                let stderr = String::from_utf8_lossy(&out.stderr).replace("`", "\\`").replace("\n", "\\n");
                                
                                let result_str = if out.status.success() {
                                    format!("Success:\\n{}", stdout)
                                } else {
                                    format!("Error:\\n{}", stderr)
                                };

                                // 3. Send the result back to JS
                                let js = format!("window.receiveIpcResponse('run_code', `{}`);", result_str);
                                let _ = webview.evaluate_script(&js);
                            },
                            Err(e) => {
                                let err_str = format!("Failed to execute compiler: {}", e).replace("`", "\\`");
                                let js = format!("window.receiveIpcResponse('run_code', `{}`);", err_str);
                                let _ = webview.evaluate_script(&js);
                            }
                        }
                    }
                }
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
