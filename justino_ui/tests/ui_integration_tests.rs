use justino_ui::css::CssParser;
use justino_ui::widget::{ButtonWidget, ContainerWidget, InputWidget, TextWidget, Window};
use justino_ui::UiError;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[test]
fn test_ui_e2e_window_layout_and_bidi_switch() -> Result<(), UiError> {
    let css_content = r#"
        .app-window {
            background-color: #f8f9fa;
            width: 800px;
            height: 600px;
            padding: 20px;
        }

        .header-title {
            font-size: 24px;
            color: #333333;
            margin-bottom: 15px;
        }

        .form-container {
            display: flex;
            flex-direction: row;
            gap: 10px;
        }

        .btn-submit {
            background-color: #007bff;
            color: #ffffff;
            padding: 10px;
            border-radius: 5px;
        }
    "#;

    let mut parser = CssParser::new(css_content);
    let stylesheet = parser.parse()?;

    // Create Window
    let mut win = Window::new("Justino UI Demo", 800, 600, "pt-BR");
    win.set_stylesheet(stylesheet);

    // Construct UI Widget Tree
    let mut root = ContainerWidget::new(1, vec!["app-window".to_string()]);
    let title_widget = TextWidget::new(2, "Bem-vindo ao Justino UI");

    let mut form_box = ContainerWidget::new(3, vec!["form-container".to_string()]);
    let input_widget = InputWidget::new(4, "Digite seu nome...", vec!["user-input".to_string()]);
    let btn_widget = ButtonWidget::new(5, "Enviar", vec!["btn-submit".to_string()]);

    form_box.add_child(input_widget.node);
    form_box.add_child(btn_widget.node);

    root.add_child(title_widget.node);
    root.add_child(form_box.node);

    win.set_root(root.node);

    // Register click event handler
    let clicked = Arc::new(AtomicBool::new(false));
    let clicked_clone = clicked.clone();
    win.on_click(5, move |_node_id| {
        clicked_clone.store(true, Ordering::SeqCst);
        Ok(())
    });

    // Render LTR frame
    let ctx_ltr = win.render_frame();
    assert!(!ctx_ltr.commands.is_empty());

    // Switch dynamically to Arabic (RTL) locale
    win.set_locale("ar-SA");
    assert!(win.locale.is_rtl());

    // Render RTL frame
    let ctx_rtl = win.render_frame();
    assert!(!ctx_rtl.commands.is_empty());

    Ok(())
}
