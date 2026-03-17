use crate::Error;
use crate::template::Assets;
use std::io::Read;
use tera::{Context, Tera};

/// Collection of templates used for codegen.
pub struct TemplateEngine {
    assets: Assets,
    tera: Tera,
}

impl TemplateEngine {
    /// Returns the built-in template collection.
    pub fn from_bytes(bytes: impl Read) -> Result<Self, Error> {
        let assets = Assets::from_reader(bytes).expect("Assets must exist");
        let mut tera = Tera::default();

        // This should just work
        tera.add_raw_templates(assets.list().map(|x| (x, assets.get_string(x).unwrap())))?;

        Ok(Self { assets, tera })
    }

    /// Loads the given template.
    pub fn get(&self, path: impl AsRef<str>) -> Result<String, Error> {
        let x = self.assets.get_string(path)?;
        Ok(x)
    }

    /// Renders the named template with the given Tera context.
    pub fn render(&self, path: impl AsRef<str>, context: &Context) -> Result<String, Error> {
        let rendered = self.tera.render(path.as_ref(), context)?;
        Ok(rendered)
    }
}
