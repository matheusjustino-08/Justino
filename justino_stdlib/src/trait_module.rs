//! Trait for registering native modules into the Justino Virtual Machine.

use crate::error::StdlibError;
use justino_core::vm::VM;

pub trait NativeModule {
    /// Returns the module's unique canonical name (e.g. "window", "http", "json", "fs", "crypto", "db", "i18n").
    fn module_name(&self) -> &'static str;

    /// Registers the native module's builtin functions and constants into the VM.
    fn register_exports(&self, vm: &mut VM) -> Result<(), StdlibError>;
}
