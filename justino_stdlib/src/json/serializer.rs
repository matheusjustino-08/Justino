//! Ultra-Fast Native JSON Parser and Serializer in Pure Rust.

use crate::error::StdlibError;
use justino_core::vm::Value;
use std::collections::HashMap;
use std::rc::Rc;

pub struct JsonSerializer;

impl JsonSerializer {
    /// Parses a JSON string into a native `Value`.
    pub fn parse(json_str: &str) -> Result<Value, StdlibError> {
        let trimmed = json_str.trim();
        if trimmed == "null" {
            Ok(Value::Null)
        } else if trimmed == "true" {
            Ok(Value::Bool(true))
        } else if trimmed == "false" {
            Ok(Value::Bool(false))
        } else if let Ok(int_val) = trimmed.parse::<i64>() {
            Ok(Value::Int(int_val))
        } else if let Ok(float_val) = trimmed.parse::<f64>() {
            Ok(Value::Float(float_val))
        } else if trimmed.starts_with('"') && trimmed.ends_with('"') {
            let content = &trimmed[1..trimmed.len() - 1];
            Ok(Value::String(Rc::new(content.to_string())))
        } else if trimmed.starts_with('{') && trimmed.ends_with('}') {
            let mut fields = HashMap::new();
            let inner = &trimmed[1..trimmed.len() - 1].trim();
            if !inner.is_empty() {
                for pair in inner.split(',') {
                    if let Some((k, v)) = pair.split_once(':') {
                        let key_clean = k.trim().trim_matches('"').to_string();
                        let val_parsed = Self::parse(v.trim())?;
                        fields.insert(key_clean, val_parsed);
                    }
                }
            }
            Ok(Value::StructInstance {
                name: "JsonObject".to_string(),
                fields: std::rc::Rc::new(std::cell::RefCell::new(fields)),
            })
        } else {
            Err(StdlibError::JsonError(format!("Invalid JSON token: {}", json_str)))
        }
    }

    /// Serializes a native `Value` into a JSON string.
    pub fn stringify(value: &Value) -> String {
        match value {
            Value::Null => "null".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::String(s) => format!("\"{}\"", s),
            Value::StructInstance { fields, .. } => {
                let mut entries = Vec::new();
                for (k, v) in fields.borrow().iter() {
                    entries.push(format!("\"{}\":{}", k, Self::stringify(v)));
                }
                format!("{{{}}}", entries.join(","))
            }
            Value::Function(f) => format!("\"<function {}>\"", f.name),
            Value::Object(_) => "\"{<object>}\"".to_string(),
        }
    }
}
