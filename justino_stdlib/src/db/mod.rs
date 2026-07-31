//! Database Module: `justino::db`.

pub mod sqlite;
pub use sqlite::*;

use crate::error::StdlibError;
use crate::trait_module::NativeModule;
use justino_core::vm::VM;

pub struct DbModule;

impl NativeModule for DbModule {
    fn module_name(&self) -> &'static str {
        "db"
    }

    fn register_exports(&self, _vm: &mut VM) -> Result<(), StdlibError> {
        Ok(())
    }
}
