use crate::template::Assets;
use crate::Error;
use std::io::Read;

/// Collection of templates used for codegen.
pub struct TemplateEngine {
    assets: Assets,
}

impl TemplateEngine {
    /// Returns the built-in template collection.
    pub fn from_bytes(bytes: impl Read) -> Result<Self, Error> {
        let assets = Assets::from_reader(bytes).expect("Assets must exist");
        Ok(Self { assets })
    }

    /// Loads the given template.
    pub fn get(&self, path: impl AsRef<str>) -> Result<String, Error> {
        let x = self.assets.get_string(path)?;
        Ok(x)
    }
}
