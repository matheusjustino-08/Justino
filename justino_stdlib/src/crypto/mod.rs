//! Cryptography Module: `justino::crypto`.

pub mod hash;
pub mod jwt;

pub use hash::*;
pub use jwt::*;

use crate::error::StdlibError;
use crate::trait_module::NativeModule;
use justino_core::vm::VM;

pub struct CryptoModule;

impl NativeModule for CryptoModule {
    fn module_name(&self) -> &'static str {
        "crypto"
    }

    fn register_exports(&self, _vm: &mut VM) -> Result<(), StdlibError> {
        Ok(())
    }
}
