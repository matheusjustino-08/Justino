//! Native Standard Library ('Batteries Included') for the Justino Programming Language (.jucode).

pub mod crypto;
pub mod db;
pub mod error;
pub mod fs;
pub mod http;
pub mod i18n;
pub mod json;
pub mod trait_module;
pub mod window;

pub use crypto::*;
pub use db::*;
pub use error::*;
pub use fs::*;
pub use http::*;
pub use i18n::*;
pub use json::*;
pub use trait_module::*;
pub use window::*;

use justino_core::vm::VM;

/// Registers all native standard library modules into a Justino Virtual Machine instance.
pub fn register_all_stdlib(vm: &mut VM) -> Result<(), StdlibError> {
    let modules: Vec<Box<dyn NativeModule>> = vec![
        Box::new(WindowModule),
        Box::new(HttpModule),
        Box::new(JsonModule),
        Box::new(FsModule),
        Box::new(CryptoModule),
        Box::new(DbModule),
        Box::new(I18nModule),
    ];

    for module in modules {
        module.register_exports(vm)?;
    }

    Ok(())
}
