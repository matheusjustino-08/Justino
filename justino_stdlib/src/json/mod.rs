//! JSON Module: `justino::json`.

pub mod serializer;
pub use serializer::*;

use crate::error::StdlibError;
use crate::trait_module::NativeModule;
use justino_core::vm::VM;

pub struct JsonModule;

impl NativeModule for JsonModule {
    fn module_name(&self) -> &'static str {
        "json"
    }

    fn register_exports(&self, _vm: &mut VM) -> Result<(), StdlibError> {
        Ok(())
    }
}
