use justino_core::eval_jucode;
use justino_ui::css::CssParser;
use justino_ui::widget::{ButtonWidget, ContainerWidget, InputWidget, TextWidget, Window};
use std::fs;

const DEFAULT_JUCODE: &str = include_str!("../../../app_demo.jucode");
const DEFAULT_CSS: &str = include_str!("../../../styles.css");

fn main() {
    println!("============================================================");
    println!("  JUSTINO LANGUAGE GRAPHICAL UI WINDOW (.jucode + .css)");
    println!("============================================================\n");

    let jucode_code = fs::read_to_string("app_demo.jucode")
        .or_else(|_| fs::read_to_string("../app_demo.jucode"))
        .unwrap_or_else(|_| DEFAULT_JUCODE.to_string());

    let css_code = fs::read_to_string("styles.css")
        .or_else(|_| fs::read_to_string("../styles.css"))
        .unwrap_or_else(|_| DEFAULT_CSS.to_string());

    println!("1. Compiling and Executing app_demo.jucode in VM...");
    match eval_jucode(&jucode_code, 1) {
        Ok(val) => println!("   -> VM Result: {}\n", val),
        Err(err) => println!("   -> Error: {}\n", err),
    }

    println!("2. Loading CSS3 Stylesheet (styles.css)...");
    let mut parser = CssParser::new(&css_code);
    match parser.parse() {
        Ok(stylesheet) => {
            println!("3. Launching Native OS Desktop GUI Window on Screen...");
            let mut win = Window::new("Justino UI Window (.jucode + .css)", 800, 600, "en-US");
            win.set_stylesheet(stylesheet);

            let mut root = ContainerWidget::new(1, vec!["window-root".to_string()]);
            let mut header = ContainerWidget::new(2, vec!["header-container".to_string()]);
            header.add_child(TextWidget::new(3, "Welcome to Justino Language!").node);
            header.add_child(TextWidget::new(4, "GPU-Accelerated Declarative UI Engine").node);

            let mut form = ContainerWidget::new(5, vec!["form-box".to_string()]);
            form.add_child(InputWidget::new(6, "Justino Developer", vec!["input-text".to_string()]).node);
            form.add_child(ButtonWidget::new(7, "Confirm", vec!["btn-action".to_string()]).node);
            form.add_child(ButtonWidget::new(8, "Toggle Language (RTL)", vec!["btn-toggle-lang".to_string()]).node);

            root.add_child(header.node);
            root.add_child(form.node);

            win.set_root(root.node);

            if let Err(e) = win.run_native_window() {
                eprintln!("Win32 Window Error: {}", e);
            }
        }
        Err(err) => {
            eprintln!("CSS Parser Error: {}", err);
        }
    }
}
