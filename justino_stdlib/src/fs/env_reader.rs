//! Native `.env` File Parser and Environment Reader.

use crate::error::StdlibError;
use std::collections::HashMap;
use std::fs;

pub struct EnvReader;

impl EnvReader {
    /// Parses a `.env` file and returns key-value mappings.
    pub fn parse_env_file(path_str: &str) -> Result<HashMap<String, String>, StdlibError> {
        let content = fs::read_to_string(path_str)
            .map_err(|e| StdlibError::FsError(format!("Failed to read env file '{}': {}", path_str, e)))?;

        let mut env_map = HashMap::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            if let Some((k, v)) = trimmed.split_once('=') {
                let key = k.trim().to_string();
                let val = v.trim().trim_matches('"').trim_matches('\'').to_string();
                env_map.insert(key.clone(), val.clone());
                std::env::set_var(&key, &val);
            }
        }

        Ok(env_map)
    }
}
