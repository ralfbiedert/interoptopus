//! ...

use crate::pass::{ModelResult, Outcome, PassInfo};

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    interop_dll_name: String,
    interop_hash: String,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self {
            info: PassInfo { name: "meta_info" },
            interop_dll_name: String::new(),
            interop_hash: String::new(),
        }
    }

    pub fn process(&mut self, _pass_meta: &mut super::PassMeta) -> ModelResult {
        // TODO
        Ok(Outcome::Unchanged)
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
