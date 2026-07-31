//! Native GPU Window Module: `justino::window`.

pub mod bridge;
pub mod native_view;

pub use bridge::*;
pub use native_view::*;

use crate::error::StdlibError;
use crate::trait_module::NativeModule;
use justino_core::vm::VM;

pub struct WindowModule;

impl NativeModule for WindowModule {
    fn module_name(&self) -> &'static str {
        "window"
    }

    fn register_exports(&self, _vm: &mut VM) -> Result<(), StdlibError> {
        Ok(())
    }
}
