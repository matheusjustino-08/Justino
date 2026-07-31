//! Standalone AOT Compiler & Executable Packaging CLI for the Justino Language (.jucode -> .exe).

use justino_core::{eval_jucode, Compiler, Parser, Scanner};
use std::env;
use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom, Write};
use std::process;

const MAGIC_TAG: &[u8; 8] = b"JUSTINO!";

fn main() {
    // 1. Check if this binary itself has an embedded .jucode payload footer
    if let Ok(Some(embedded_source)) = check_and_extract_embedded_payload() {
        println!("============================================================");
        println!("  Justino Language (.jucode) - Native Executable (.exe)");
        println!("============================================================\n");

        match eval_jucode(&embedded_source, 1) {
            Ok(result) => {
                println!("Final Result: {}\n", result);
            }
            Err(err) => {
                eprintln!("Execution Error: {}\n", err);
            }
        }

        println!("------------------------------------------------------------");
        println!("Press Enter to close this window...");
        let mut pause_buf = [0u8; 1];
        let _ = std::io::stdin().read(&mut pause_buf);
        process::exit(0);
    }

    // 2. Normal CLI operation
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "build" | "compile" => {
            if args.len() < 3 {
                eprintln!("Error: Specify the .jucode file to compile.");
                eprintln!("Usage: justino build <file.jucode> [-o output.exe]");
                process::exit(1);
            }

            let input_path = &args[2];
            let mut output_path = String::from("my_program.exe");

            let mut i = 3;
            while i < args.len() {
                if args[i] == "-o" && i + 1 < args.len() {
                    output_path = args[i + 1].clone();
                    i += 2;
                } else {
                    i += 1;
                }
            }

            if !output_path.ends_with(".exe") {
                output_path.push_str(".exe");
            }

            println!("[Justino AOT Compiler] Compiling '{}'...", input_path);

            let source = match fs::read_to_string(input_path) {
                Ok(content) => content,
                Err(err) => {
                    eprintln!("Error reading file '{}': {}", input_path, err);
                    process::exit(1);
                }
            };

            // Validate source syntax & compilation before packaging
            let mut scanner = Scanner::new(&source, 1);
            let tokens = match scanner.scan() {
                Ok(toks) => toks,
                Err(err) => {
                    eprintln!("Lexical Error: {}", err);
                    process::exit(1);
                }
            };

            let mut parser = Parser::new(tokens, 1);
            let program = match parser.parse_program() {
                Ok(prog) => prog,
                Err(err) => {
                    eprintln!("Syntax Error: {}", err);
                    process::exit(1);
                }
            };

            let compiler = Compiler::new(1);
            let compiled_func = match compiler.compile_program(&program) {
                Ok(func) => func,
                Err(err) => {
                    eprintln!("Compilation Error: {}", err);
                    process::exit(1);
                }
            };

            println!("   -> AST validated. {} bytecode instructions generated.", compiled_func.instructions.len());

            match build_standalone_executable(&source, &output_path) {
                Ok(_) => {
                    println!("------------------------------------------------------------");
                    println!("Native executable generated successfully: '{}'", output_path);
                    println!("------------------------------------------------------------");
                }
                Err(err) => {
                    eprintln!("Error packaging executable: {}", err);
                    process::exit(1);
                }
            }
        }
        "run" => {
            if args.len() < 3 {
                eprintln!("Usage: justino run <file.jucode>");
                process::exit(1);
            }
            let input_path = &args[2];
            let source = match fs::read_to_string(input_path) {
                Ok(content) => content,
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                    process::exit(1);
                }
            };

            match eval_jucode(&source, 1) {
                Ok(val) => println!("Result: {}", val),
                Err(err) => {
                    eprintln!("Execution Error: {}", err);
                    process::exit(1);
                }
            }
        }
        _ => print_usage(),
    }
}

fn print_usage() {
    println!("Justino Programming Language (.jucode) - AOT Compiler Driver");
    println!("Usage:");
    println!("  justino build <file.jucode> [-o output.exe]   Compiles into a standalone native executable (.exe)");
    println!("  justino run <file.jucode>                    Executes directly in the VM");
}

fn build_standalone_executable(source: &str, output_path: &str) -> Result<(), String> {
    let current_exe = env::current_exe().map_err(|e| format!("Failed to get current binary path: {}", e))?;

    // Copy base runner executable bytes to output path
    fs::copy(&current_exe, output_path).map_err(|e| format!("Failed to create executable '{}': {}", output_path, e))?;

    let mut file = File::options()
        .append(true)
        .open(output_path)
        .map_err(|e| format!("Failed to open '{}' for writing: {}", output_path, e))?;

    let source_bytes = source.as_bytes();
    let source_len = source_bytes.len() as u64;

    file.write_all(source_bytes)
        .map_err(|e| format!("Failed to write payload: {}", e))?;

    file.write_all(&source_len.to_le_bytes())
        .map_err(|e| format!("Failed to write payload size: {}", e))?;

    file.write_all(MAGIC_TAG)
        .map_err(|e| format!("Failed to write magic tag: {}", e))?;

    file.flush().map_err(|e| format!("Failed to save file: {}", e))?;

    Ok(())
}

fn check_and_extract_embedded_payload() -> Result<Option<String>, String> {
    let current_exe = match env::current_exe() {
        Ok(path) => path,
        Err(_) => return Ok(None),
    };

    let mut file = match File::open(&current_exe) {
        Ok(f) => f,
        Err(_) => return Ok(None),
    };

    let file_len = match file.metadata() {
        Ok(meta) => meta.len(),
        Err(_) => return Ok(None),
    };

    if file_len < 16 {
        return Ok(None);
    }

    // Read footer magic tag (last 8 bytes)
    if file.seek(SeekFrom::End(-8)).is_err() {
        return Ok(None);
    }

    let mut tag_buf = [0u8; 8];
    if file.read_exact(&mut tag_buf).is_err() {
        return Ok(None);
    }

    if &tag_buf != MAGIC_TAG {
        return Ok(None);
    }

    // Read source payload length (8 bytes preceding magic tag)
    if file.seek(SeekFrom::End(-16)).is_err() {
        return Ok(None);
    }

    let mut len_buf = [0u8; 8];
    if file.read_exact(&mut len_buf).is_err() {
        return Ok(None);
    }

    let payload_len = u64::from_le_bytes(len_buf);
    let total_footer_size = 16 + payload_len;

    if file_len < total_footer_size {
        return Ok(None);
    }

    let payload_start = file_len - total_footer_size;
    if file.seek(SeekFrom::Start(payload_start)).is_err() {
        return Ok(None);
    }

    let mut payload_bytes = vec![0u8; payload_len as usize];
    if file.read_exact(&mut payload_bytes).is_err() {
        return Ok(None);
    }

    let source_str = String::from_utf8(payload_bytes).map_err(|e| format!("Invalid UTF-8 payload: {}", e))?;
    Ok(Some(source_str))
}
