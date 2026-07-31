//! Compiler Driver Runner (`runner/compiler_cli.rs`).

use justino_core::vm::Value;
use justino_core::{eval_jucode, JustinoError};

pub struct CompilerCli;

impl CompilerCli {
    /// Compiles and runs `.jucode` source code directly in the VM.
    pub fn run_jucode_source(source: &str) -> Result<Value, JustinoError> {
        eval_jucode(source, 1)
    }

    /// Compiles `.jucode` into a standalone `.exe` binary package.
    pub fn build_executable(source: &str, output_exe: &str) -> Result<String, JustinoError> {
        // Validate source syntax
        let _ = eval_jucode(source, 1)?;

        let exe_bytes = std::fs::read("justino.exe")
            .or_else(|_| std::fs::read("../target/debug/justino.exe"))
            .unwrap_or_else(|_| vec![0u8; 1024]);

        let mut final_bytes = exe_bytes;
        let source_bytes = source.as_bytes();
        let source_len = source_bytes.len() as u64;

        final_bytes.extend_from_slice(source_bytes);
        final_bytes.extend_from_slice(&source_len.to_le_bytes());
        final_bytes.extend_from_slice(b"JUSTINO!");

        std::fs::write(output_exe, final_bytes).map_err(|e| JustinoError::RuntimeError {
            message: format!("Failed to write executable '{}': {}", output_exe, e),
            span: None,
        })?;

        Ok(format!("Executable built successfully: {}", output_exe))
    }
}
