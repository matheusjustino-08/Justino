//! Runtime values used by the Justino Virtual Machine.

use crate::compiler::opcode::CompiledFunction;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

/// Represents any runtime value in the Justino language.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(Rc<String>),
    Null,
    Object(Rc<RefCell<HashMap<String, Value>>>),
    Function(Rc<CompiledFunction>),
    StructInstance {
        name: String,
        fields: Rc<RefCell<HashMap<String, Value>>>,
    },
}

impl Value {
    /// Determines truthiness of a value according to Justino language rules.
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Null => false,
            Value::Int(n) => *n != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Object(_) | Value::Function(_) | Value::StructInstance { .. } => true,
        }
    }

    /// Returns a human-readable type name string.
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::Bool(_) => "bool",
            Value::String(_) => "string",
            Value::Null => "null",
            Value::Object(_) => "object",
            Value::Function(_) => "function",
            Value::StructInstance { .. } => "struct",
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Bool(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::Null => write!(f, "null"),
            Value::Object(map) => {
                write!(f, "{{ ")?;
                let map_ref = map.borrow();
                for (i, (k, v)) in map_ref.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, " }}")
            }
            Value::Function(func) => write!(f, "<fn {}>", func.name),
            Value::StructInstance { name, fields } => {
                write!(f, "{} {{ ", name)?;
                let map_ref = fields.borrow();
                for (i, (k, v)) in map_ref.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, " }}")
            }
        }
    }
}
