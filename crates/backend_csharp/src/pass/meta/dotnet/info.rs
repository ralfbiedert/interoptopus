//! Computes and stores API hash metadata for the dotnet (plugin) pipeline.

use crate::pass::{ModelResult, Outcome, PassInfo};
use interoptopus::inventory::PluginInventory;
use interoptopus::pattern::api_guard::ApiHash;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    api_hash: u64,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, api_hash: 0 }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta, inventory: &PluginInventory) -> ModelResult {
        self.api_hash = ApiHash::from_plugin(inventory).hash();
        Ok(Outcome::Unchanged)
    }

    /// Returns the API guard hash as a `u64`.
    #[must_use]
    pub fn api_hash(&self) -> u64 {
        self.api_hash
    }

    /// Returns the API guard hash formatted as a C# hex literal (e.g., `0x00123ABC...`).
    #[must_use]
    pub fn api_hash_hex_literal(&self) -> String {
        format!("0x{:016X}", self.api_hash)
    }
}
