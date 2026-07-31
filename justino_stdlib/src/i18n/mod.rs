//! Internationalization Module: `justino::i18n`.

pub mod cldr_format;
pub mod locale_manager;

pub use cldr_format::*;
pub use locale_manager::*;

use crate::error::StdlibError;
use crate::trait_module::NativeModule;
use justino_core::vm::VM;

pub struct I18nModule;

impl NativeModule for I18nModule {
    fn module_name(&self) -> &'static str {
        "i18n"
    }

    fn register_exports(&self, _vm: &mut VM) -> Result<(), StdlibError> {
        Ok(())
    }
}
