//! Pipeline — wires model and output passes into the public API.
//!
//! The pipeline runs model passes to build the C language model, then
//! output passes to render it into a `.h` header file via Tera templates.

use crate::lang::{NamingConfig, NamingStyle};
use crate::pass::{model, output};
use interoptopus::inventory::RustInventory;
use interoptopus_backends::Error;
use interoptopus_backends::output::Multibuf;
use interoptopus_backends::template::TemplateEngine;

/// Embedded template archive, packed at build time.
static TEMPLATES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/templates.tar"));

/// C library code generator — the main pipeline entry point.
///
/// Configure via the builder pattern, then call [`process`](CLibrary::process)
/// to run the pipeline and produce the output header.
pub struct CLibrary<'a> {
    inventory: &'a RustInventory,
    loader_name: String,
    ifndef: String,
    filename: String,
    naming: NamingConfig,
}

/// Builder for [`CLibrary`].
pub struct CLibraryBuilder<'a> {
    inventory: &'a RustInventory,
    loader_name: String,
    ifndef: String,
    filename: String,
    naming: NamingConfig,
}

impl<'a> CLibraryBuilder<'a> {
    /// Set the loader / dispatch table name prefix.
    ///
    /// This controls names like `{name}_api_t`, `{name}_load`, etc.
    #[must_use]
    pub fn loader_name(mut self, name: impl Into<String>) -> Self {
        self.loader_name = name.into();
        self
    }

    /// Override the `#ifndef` header guard name.
    #[must_use]
    pub fn ifndef(mut self, guard: impl Into<String>) -> Self {
        self.ifndef = guard.into();
        self
    }

    /// Set the output filename (default: `"{loader_name}.h"`).
    #[must_use]
    pub fn filename(mut self, name: impl Into<String>) -> Self {
        self.filename = name.into();
        self
    }

    /// Set a prefix prepended to all generated type and function names.
    #[must_use]
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.naming.prefix = Some(prefix.into());
        self
    }

    /// Set the naming style for type names (structs, enums, opaques, patterns).
    #[must_use]
    pub fn type_naming(mut self, style: NamingStyle) -> Self {
        self.naming.type_naming = style;
        self
    }

    /// Set the naming style for enum variant names.
    #[must_use]
    pub fn enum_variant_naming(mut self, style: NamingStyle) -> Self {
        self.naming.enum_variant_naming = style;
        self
    }

    /// Set the naming style for function names.
    #[must_use]
    pub fn function_naming(mut self, style: NamingStyle) -> Self {
        self.naming.function_naming = style;
        self
    }

    /// Set the naming style for function parameter names.
    #[must_use]
    pub fn function_parameter_naming(mut self, style: NamingStyle) -> Self {
        self.naming.function_parameter_naming = style;
        self
    }

    /// Set the naming style for constant names (e.g. `_TAG` tag enums).
    #[must_use]
    pub fn const_naming(mut self, style: NamingStyle) -> Self {
        self.naming.const_naming = style;
        self
    }

    /// Replace the entire naming configuration at once.
    #[must_use]
    pub fn naming_config(mut self, config: NamingConfig) -> Self {
        self.naming = config;
        self
    }

    /// Build the pipeline.
    #[must_use]
    pub fn build(self) -> CLibrary<'a> {
        let filename = if self.filename.is_empty() {
            format!("{}.h", self.loader_name)
        } else {
            self.filename
        };
        CLibrary { inventory: self.inventory, loader_name: self.loader_name, ifndef: self.ifndef, filename, naming: self.naming }
    }
}

impl CLibrary<'_> {
    /// Create a new builder.
    #[must_use]
    pub fn builder(inventory: &RustInventory) -> CLibraryBuilder<'_> {
        CLibraryBuilder { inventory, loader_name: String::new(), ifndef: "interoptopus_generated".to_string(), filename: String::new(), naming: NamingConfig::default() }
    }

    /// Run the full pipeline: model passes → output passes → assemble header.
    pub fn process(&self) -> Result<Multibuf, Error> {
        let engine = TemplateEngine::from_bytes(TEMPLATES)?;

        // Model pass: build the C model from the Rust inventory.
        let c_model = model::build_model(self.inventory, &self.naming);

        // Output pass: render the model into a complete header.
        let header = output::render_header(&engine, &c_model, &self.loader_name, &self.ifndef)?;

        let mut multibuf = Multibuf::new();
        multibuf.add_buffer(&self.filename, header);
        Ok(multibuf)
    }
}
