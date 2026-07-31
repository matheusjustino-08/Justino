//! File System Module: `justino::fs`.

pub mod async_file;
pub mod env_reader;

pub use async_file::*;
pub use env_reader::*;

use crate::error::StdlibError;
use crate::trait_module::NativeModule;
use justino_core::vm::VM;

pub struct FsModule;

impl NativeModule for FsModule {
    fn module_name(&self) -> &'static str {
        "fs"
    }

    fn register_exports(&self, _vm: &mut VM) -> Result<(), StdlibError> {
        Ok(())
    }
}
