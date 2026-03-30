use crate::template::Assets;
use crate::Error;
use std::collections::HashMap;
use std::io::Read;
use tera::{Context, Tera};

const GLOBAL_PREFIX: &str = "_global/";

/// Collection of templates used for codegen.
///
/// Files under `_global/` are automatically injected as context variables
/// in every `render()` call. The variable name is derived from the path:
/// `_global/fns/decorators/internal.cs` becomes `_fns_decorators_internal`.
pub struct TemplateEngine {
    assets: Assets,
    tera: Tera,
    globals: HashMap<String, String>,
}

impl TemplateEngine {
    /// Returns the built-in template collection.
    pub fn from_bytes(bytes: impl Read) -> Result<Self, Error> {
        let assets = Assets::from_reader(bytes).expect("Assets must exist");

        let mut tera = Tera::default();
        let mut globals = HashMap::new();

        // We manually add the global files here, since Tera doesn't support proper indentation
        // of multi-line templates when imported with {% include %}. We therefore have to build
        // a convention-based system where 'global' snippets under `_global/foo/bar.cs` are
        // converted to `_foo_bar` variables and made available everywhere.
        for path in assets.list() {
            if let Some(rest) = path.strip_prefix(GLOBAL_PREFIX) {
                let var_name = format!("_{}", rest.trim_end_matches(".cs").replace('/', "_"));
                let content = assets.get_string(path)?;
                globals.insert(var_name, content);
            }
        }

        tera.add_raw_templates(assets.list().filter(|x| !x.starts_with(GLOBAL_PREFIX)).map(|x| (x, assets.get_string(x).unwrap())))?;

        Ok(Self { assets, tera, globals })
    }

    /// Loads the given template.
    pub fn get(&self, path: impl AsRef<str>) -> Result<String, Error> {
        let x = self.assets.get_string(path)?;
        Ok(x)
    }

    /// Renders the named template with the given Tera context.
    ///
    /// All `_global/` files are automatically available as context variables.
    pub fn render(&self, path: impl AsRef<str>, context: &Context) -> Result<String, Error> {
        let mut ctx = context.clone();

        // Inject all globals, see comment above for reasons.
        for (key, value) in &self.globals {
            ctx.insert(key.as_str(), value);
        }

        let rendered = self.tera.render(path.as_ref(), &ctx)?;
        Ok(rendered)
    }
}
