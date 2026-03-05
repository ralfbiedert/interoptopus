//! ...

use crate::pass::{ModelResult, Outcome, PassInfo};

#[derive(Default)]
pub struct Config {
    pub dll_name: String,
}

pub struct Pass {
    info: PassInfo,
    dll_name: String,
    api_hash: String,
}

impl Pass {
    pub fn new(config: Config) -> Self {
        Self { info: PassInfo { name: "meta_info" }, dll_name: config.dll_name, api_hash: String::new() }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta) -> ModelResult {
        // TODO
        Ok(Outcome::Unchanged)
    }

    pub fn dll_name(&self) -> &str {
        &self.dll_name
    }

    pub fn api_hash(&self) -> &str {
        &self.api_hash
    }

    pub fn interoptopus_crate(&self) -> &str {
        env!("CARGO_PKG_NAME")
    }

    pub fn interoptopus_version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }
}
