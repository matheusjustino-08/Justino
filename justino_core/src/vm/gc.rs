//! Lightweight Safe GC and Memory Management (Reference Counting + Arena).

use crate::vm::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Arena for tracking runtime allocations safely without unsafe code.
#[derive(Debug, Default)]
pub struct GcArena {
    struct_allocations: Vec<Rc<RefCell<HashMap<String, Value>>>>,
    string_allocations: Vec<Rc<String>>,
}

impl GcArena {
    pub fn new() -> Self {
        Self {
            struct_allocations: Vec::new(),
            string_allocations: Vec::new(),
        }
    }

    /// Allocates a new heap string managed by reference counting and tracked by the arena.
    pub fn alloc_string(&mut self, text: impl Into<String>) -> Value {
        let rc_str = Rc::new(text.into());
        self.string_allocations.push(rc_str.clone());
        Value::String(rc_str)
    }

    /// Allocates a new struct instance with fields managed safely via Rc<RefCell<HashMap>>.
    pub fn alloc_struct(&mut self, name: impl Into<String>, fields: HashMap<String, Value>) -> Value {
        let rc_map = Rc::new(RefCell::new(fields));
        self.struct_allocations.push(rc_map.clone());
        Value::StructInstance {
            name: name.into(),
            fields: rc_map,
        }
    }

    /// Runs garbage collection pass to prune unreferenced arena tracked objects (Rc count == 1).
    pub fn collect_garbage(&mut self) {
        self.struct_allocations.retain(|rc| Rc::strong_count(rc) > 1);
        self.string_allocations.retain(|rc| Rc::strong_count(rc) > 1);
    }

    /// Total active allocations tracked by arena.
    pub fn live_objects_count(&self) -> usize {
        self.struct_allocations.len() + self.string_allocations.len()
    }
}
