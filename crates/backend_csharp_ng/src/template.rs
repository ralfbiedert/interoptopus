use crate::error::Error;
use interoptopus_backends::template::Assets;

// Include the tar file that was created by build.rs
const ASSET_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/templates.tar"));

/// Collection of templates used for codegen.
pub struct Templates {
    assets: Assets,
}

impl Templates {
    /// Returns the built-in template collection.
    pub fn builtins() -> Self {
        let assets = Assets::from_reader(ASSET_BYTES).expect("Assets must exist");
        Self { assets }
    }

    /// Loads the given template.
    pub fn load_string(&self, path: impl AsRef<str>) -> Result<String, Error> {
        let x = self.assets.load_string(path)?;
        Ok(x)
    }
}
