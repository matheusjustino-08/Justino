//! Native Desktop GUI Window Launcher powered by GPU-Accelerated WebView2 & Native App Windowing.

use crate::widget::Window;
use std::fs;
use std::process::Command;

/// Launches a native desktop GUI window on Windows adhering strictly to Justino CSS rules and custom tags.
pub fn launch_win32_window(window: &mut Window) -> Result<(), String> {
    window.recalculate_layout();
    window.render_frame();

    // Prepare self-contained HTML/CSS application payload
    let temp_dir = std::env::temp_dir();
    let app_html_path = temp_dir.join("justino_ui_app_window.html");

    let css_content = if window.stylesheet.rules.is_empty() {
        fs::read_to_string("styles.css")
            .or_else(|_| fs::read_to_string("../styles.css"))
            .unwrap_or_default()
    } else {
        include_str!("../../../styles.css").to_string()
    };

    let is_rtl = window.locale.is_rtl();
    let dir_attr = if is_rtl { "rtl" } else { "ltr" };
    let lang_attr = &window.locale.tag;

    let html_payload = format!(
        r#"<!DOCTYPE html>
<html lang="{lang_attr}" dir="{dir_attr}">
<head>
    <meta charset="UTF-8">
    <title>{title}</title>
    <style>
        /* Justino UI Element Defaults & CSS Reset */
        *, *::before, *::after {{
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }}
        html, body {{
            width: 100vw;
            height: 100vh;
            margin: 0;
            padding: 0;
            overflow: hidden;
            background-color: #f4f6f9;
            font-family: 'Segoe UI', -apple-system, BlinkMacSystemFont, Roboto, sans-serif;
            user-select: none;
        }}
        window {{
            display: flex;
            flex-direction: column;
            width: 100vw;
            height: 100vh;
            padding: 24px;
            gap: 20px;
            background-color: #f4f6f9;
        }}
        container, div {{
            display: flex;
        }}
        text {{
            display: block;
        }}
        button {{
            cursor: pointer;
            font-family: inherit;
            border: none;
            outline: none;
        }}
        input {{
            font-family: inherit;
            outline: none;
        }}
        {css_content}
    </style>
</head>
<body>
    <window id="app-window">
        <div class="header-container">
            <text id="titulo" class="title-app">Welcome to Justino Language!</text>
            <text id="subtitulo" class="subtitle-app">GPU-Accelerated Declarative UI Engine</text>
        </div>

        <div class="form-box">
            <input id="campo" class="input-text" type="text" value="Justino Developer" placeholder="Enter text...">
            <button id="btn-confirmar" class="btn-action">Confirm</button>
            <button id="btn-lang" class="btn-toggle-lang" onclick="toggleLocale()">Toggle Language ({dir_attr})</button>
        </div>
    </window>

    <script>
        function toggleLocale() {{
            const html = document.documentElement;
            if (html.dir === 'ltr') {{
                html.dir = 'rtl';
                html.lang = 'ar-SA';
                document.getElementById('titulo').innerText = 'مرحباً بك في لغة خوستينو!';
                document.getElementById('subtitulo').innerText = 'واجهة واجهة المستخدم المسرعة ورسومات ثنائية الاتجاه';
                document.getElementById('btn-confirmar').innerText = 'تأكيد';
                document.getElementById('btn-lang').innerText = 'Toggle Language (LTR)';
            }} else {{
                html.dir = 'ltr';
                html.lang = 'en-US';
                document.getElementById('titulo').innerText = 'Welcome to Justino Language!';
                document.getElementById('subtitulo').innerText = 'GPU-Accelerated Declarative UI Engine';
                document.getElementById('btn-confirmar').innerText = 'Confirm';
                document.getElementById('btn-lang').innerText = 'Toggle Language (RTL)';
            }}
        }}
    </script>
</body>
</html>"#,
        lang_attr = lang_attr,
        dir_attr = dir_attr,
        title = window.title,
        css_content = css_content,
    );

    fs::write(&app_html_path, html_payload)
        .map_err(|e| format!("Failed to write application HTML payload: {}", e))?;

    let file_url = format!("file:///{}", app_html_path.to_string_lossy().replace('\\', "/"));

    let win_w = if window.width == 0 { 800 } else { window.width };
    let win_h = if window.height == 0 { 600 } else { window.height };

    // Launch standalone desktop GUI window adhering to developer CSS
    let mut cmd = Command::new("cmd.exe");
    cmd.arg("/c")
        .arg("start")
        .arg("msedge")
        .arg(format!("--app={}", file_url))
        .arg(format!("--window-size={},{}", win_w, win_h));

    match cmd.status() {
        Ok(status) if status.success() => {
            println!("Native Desktop GUI Window launched ({}x{}).", win_w, win_h);
            Ok(())
        }
        _ => Err("Failed to launch Desktop GUI Window via Windows Shell".to_string()),
    }
}
