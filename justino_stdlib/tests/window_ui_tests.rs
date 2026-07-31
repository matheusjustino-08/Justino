use justino_stdlib::error::StdlibError;
use justino_stdlib::window::{EventBridge, NativeView};

#[test]
fn test_native_view_creation_and_stylesheet() -> Result<(), StdlibError> {
    let mut view = NativeView::new("Justino GPU Window Test", 800, 600, "en-US");
    let css = r#"
        window { background-color: #ffffff; width: 800px; height: 600px; }
        .btn-action { background-color: #3182ce; color: #ffffff; }
    "#;
    view.set_stylesheet(css)?;
    assert_eq!(view.window.width, 800);
    assert_eq!(view.window.height, 600);
    Ok(())
}

#[test]
fn test_event_bridge_binding_and_trigger() -> Result<(), StdlibError> {
    let mut bridge = EventBridge::new();
    let triggered = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let trig_clone = triggered.clone();

    bridge.bind_event("btn_click", move |payload| {
        if payload == "confirm" {
            trig_clone.store(true, std::sync::atomic::Ordering::SeqCst);
        }
        Ok(())
    });

    let success = bridge.trigger_event("btn_click", "confirm")?;
    assert!(success);
    assert!(triggered.load(std::sync::atomic::Ordering::SeqCst));
    Ok(())
}
