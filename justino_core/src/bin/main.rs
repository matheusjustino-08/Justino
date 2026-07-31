use justino_core::eval_jucode;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let source = if args.len() > 1 {
        let file_path = &args[1];
        if !file_path.ends_with(".jucode") {
            eprintln!("Warning: File '{}' does not use official extension .jucode", file_path);
        }
        match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Error reading file '{}': {}", file_path, err);
                process::exit(1);
            }
        }
    } else {
        println!("Executing Justino demo program (.jucode)...\n");
        r#"
            struct Rectangle {
                width: int,
                height: int
            }

            fn calculate_area(r: Rectangle) -> int {
                return r.width * r.height;
            }

            fn main() -> int {
                let mut rect = Rectangle { width: 12, height: 5 };
                let area = calculate_area(rect);
                let mut i = 1;
                let mut sum = 0;
                while i <= 3 {
                    sum = sum + area;
                    i = i + 1;
                }
                let msg = "Total accumulated area: ${sum}";
                return sum;
            }

            return main();
        "#.to_string()
    };

    println!("--- Justino Source Code (.jucode) ---");
    println!("{}\n", source);
    println!("--- Executing in Register VM ---");

    match eval_jucode(&source, 1) {
        Ok(result) => {
            println!("Success! Final VM result: {}", result);
        }
        Err(err) => {
            eprintln!("Execution Error: {}", err);
            process::exit(1);
        }
    }
}
