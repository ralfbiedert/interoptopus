//! ...

use crate::stage::ProcessError;
use interoptopus::inventory::Inventory;

#[derive(Default)]
pub struct Config {}

pub struct Stage {
    interop_dll_name: String,
    interop_hash: String,
}

impl Stage {
    pub fn new(_: Config) -> Self {
        Self { interop_dll_name: String::new(), interop_hash: String::new() }
    }

    pub fn process(&mut self, _: &Inventory) -> ProcessError {
        // TODO
        Ok(())
    }

    pub fn interop_dll_name(&self) -> &str {
        &self.interop_dll_name
    }

    pub fn interop_hash(&self) -> &str {
        &self.interop_hash
    }

    pub fn interoptopus_crate(&self) -> &str {
        env!("CARGO_PKG_NAME")
    }

    pub fn interoptopus_version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }
}
