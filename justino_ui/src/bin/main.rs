use justino_core::eval_jucode;
use justino_ui::css::CssParser;
use justino_ui::widget::{ButtonWidget, ContainerWidget, InputWidget, TextWidget, Window};
use std::fs;

fn main() {
    println!("=== Phase 2 Execution Test: Justino UI & i18n (.jucode + .css) ===\n");

    let jucode_path = "../app_demo.jucode";
    let css_path = "../styles.css";

    let jucode_code = fs::read_to_string(jucode_path).unwrap_or_else(|_| {
        r#"
            fn main() -> int {
                return 42;
            }
            return main();
        "#.to_string()
    });

    let css_code = fs::read_to_string(css_path).unwrap_or_else(|_| {
        r#"
            window { background-color: #ffffff; width: 800px; height: 600px; padding: 20px; }
            .btn-action { background-color: #3182ce; color: #ffffff; padding: 10px; }
        "#.to_string()
    });

    println!("1. Executing logic code in Justino VM...");
    match eval_jucode(&jucode_code, 1) {
        Ok(res) => println!("   -> VM returned: {}\n", res),
        Err(err) => println!("   -> VM error: {}\n", err),
    }

    println!("2. Processing CSS3 stylesheet (styles.css)...");
    let mut parser = CssParser::new(&css_code);
    let stylesheet = match parser.parse() {
        Ok(sheet) => {
            println!("   -> {} CSS3 rules successfully loaded with specificity support!", sheet.rules.len());
            sheet
        }
        Err(err) => {
            eprintln!("   -> Error parsing CSS: {}", err);
            return;
        }
    };

    println!("\n3. Creating UI Window and Component Tree...");
    let mut win = Window::new("Justino UI Demo Application", 800, 600, "en-US");
    win.set_stylesheet(stylesheet);

    let mut root = ContainerWidget::new(1, vec!["window-root".to_string()]);
    
    let mut header = ContainerWidget::new(2, vec!["header-container".to_string()]);
    let title = TextWidget::new(3, "Welcome to Justino Language!");
    let subtitle = TextWidget::new(4, "GPU-accelerated UI rendering engine with Bi-directional support");
    header.add_child(title.node);
    header.add_child(subtitle.node);

    let mut form = ContainerWidget::new(5, vec!["form-box".to_string()]);
    let input = InputWidget::new(6, "Enter your name...", vec!["input-text".to_string()]);
    let btn_action = ButtonWidget::new(7, "Confirm", vec!["btn-action".to_string()]);
    let btn_lang = ButtonWidget::new(8, "Toggle Language", vec!["btn-toggle-lang".to_string()]);

    form.add_child(input.node);
    form.add_child(btn_action.node);
    form.add_child(btn_lang.node);

    root.add_child(header.node);
    root.add_child(form.node);

    win.set_root(root.node);

    println!("\n4. Rendering Frame in English (en-US - LTR)...");
    let cmd_count_ltr = win.render_frame().commands.len();
    println!("   -> {} GPU draw commands generated in buffer.", cmd_count_ltr);
    println!("   -> Action button position (LTR): x = {:.1}px, y = {:.1}px", 
             win.root_node.children[1].children[1].dimensions.content.x,
             win.root_node.children[1].children[1].dimensions.content.y);

    println!("\n5. Dynamically switching to Arabic (ar-SA - RTL Bidi Layout)...");
    win.set_locale("ar-SA");
    let cmd_count_rtl = win.render_frame().commands.len();
    println!("   -> Active locale: {} (RTL = {})", win.locale.tag, win.locale.is_rtl());
    println!("   -> {} GPU draw commands generated in buffer.", cmd_count_rtl);
    println!("   -> Action button position (Mirrored RTL): x = {:.1}px, y = {:.1}px", 
             win.root_node.children[1].children[1].dimensions.content.x,
             win.root_node.children[1].children[1].dimensions.content.y);

    println!("\nUI execution test completed successfully!");
}
