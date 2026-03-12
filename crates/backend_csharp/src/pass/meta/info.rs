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
    #[must_use] 
    pub fn new(config: Config) -> Self {
        Self { info: PassInfo { name: file!() }, dll_name: config.dll_name, api_hash: String::new() }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta) -> ModelResult {
        // TODO
        Ok(Outcome::Unchanged)
    }

    #[must_use] 
    pub fn dll_name(&self) -> &str {
        &self.dll_name
    }

    #[must_use] 
    pub fn api_hash(&self) -> &str {
        &self.api_hash
    }

    #[must_use] 
    pub fn interoptopus_crate(&self) -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    #[must_use] 
    pub fn interoptopus_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}
