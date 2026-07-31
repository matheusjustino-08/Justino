//! Bidirectional Event Bridge between UI Events and Justino VM.

use crate::error::StdlibError;
use std::collections::HashMap;

pub type EventCallback = Box<dyn FnMut(&str) -> Result<(), StdlibError>>;

pub struct EventBridge {
    pub listeners: HashMap<String, EventCallback>,
}

impl EventBridge {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    pub fn bind_event<F>(&mut self, event_name: impl Into<String>, callback: F)
    where
        F: FnMut(&str) -> Result<(), StdlibError> + 'static,
    {
        self.listeners.insert(event_name.into(), Box::new(callback));
    }

    pub fn trigger_event(&mut self, event_name: &str, payload: &str) -> Result<bool, StdlibError> {
        if let Some(cb) = self.listeners.get_mut(event_name) {
            cb(payload)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
