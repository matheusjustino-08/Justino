//! HTTP Module: `justino::http`.

pub mod client;
pub mod request_response;
pub mod server;

pub use client::*;
pub use request_response::*;
pub use server::*;

use crate::error::StdlibError;
use crate::trait_module::NativeModule;
use justino_core::vm::VM;

pub struct HttpModule;

impl NativeModule for HttpModule {
    fn module_name(&self) -> &'static str {
        "http"
    }

    fn register_exports(&self, _vm: &mut VM) -> Result<(), StdlibError> {
        Ok(())
    }
}
