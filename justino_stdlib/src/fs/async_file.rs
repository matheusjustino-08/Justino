//! UTF-8 File I/O Operations with Path Traversal Protection.

use crate::error::StdlibError;
use std::fs;
use std::path::PathBuf;

pub struct AsyncFile;

impl AsyncFile {
    /// Reads a UTF-8 text file with path traversal protection.
    pub fn read_file(path_str: &str) -> Result<String, StdlibError> {
        let sanitized = sanitize_path(path_str)?;
        fs::read_to_string(&sanitized)
            .map_err(|e| StdlibError::FsError(format!("Failed to read file '{}': {}", path_str, e)))
    }

    /// Writes UTF-8 content to a text file with path traversal protection.
    pub fn write_file(path_str: &str, content: &str) -> Result<bool, StdlibError> {
        let sanitized = sanitize_path(path_str)?;
        if let Some(parent) = sanitized.parent() {
            let _ = fs::create_dir_all(parent);
        }
        fs::write(&sanitized, content)
            .map_err(|e| StdlibError::FsError(format!("Failed to write file '{}': {}", path_str, e)))?;
        Ok(true)
    }
}

fn sanitize_path(path_str: &str) -> Result<PathBuf, StdlibError> {
    if path_str.contains("..") {
        return Err(StdlibError::FsError(format!(
            "Path Traversal Violation: Relative parent navigation prohibited ('{}')",
            path_str
        )));
    }
    Ok(PathBuf::from(path_str))
}
