//! Pipeline — wires model and output passes into the public API.
//!
//! The pipeline runs model passes to build the C language model, then
//! output passes to render it into a `.h` header file via Tera templates.

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
}

/// Builder for [`CLibrary`].
pub struct CLibraryBuilder<'a> {
    inventory: &'a RustInventory,
    loader_name: String,
    ifndef: String,
    filename: String,
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

    /// Build the pipeline.
    #[must_use]
    pub fn build(self) -> CLibrary<'a> {
        let filename = if self.filename.is_empty() {
            format!("{}.h", self.loader_name)
        } else {
            self.filename
        };
        CLibrary { inventory: self.inventory, loader_name: self.loader_name, ifndef: self.ifndef, filename }
    }
}

impl CLibrary<'_> {
    /// Create a new builder.
    #[must_use]
    pub fn builder(inventory: &RustInventory) -> CLibraryBuilder<'_> {
        CLibraryBuilder { inventory, loader_name: String::new(), ifndef: "interoptopus_generated".to_string(), filename: String::new() }
    }

    /// Run the full pipeline: model passes → output passes → assemble header.
    pub fn process(&self) -> Result<Multibuf, Error> {
        let engine = TemplateEngine::from_bytes(TEMPLATES)?;

        // Model pass: build the C model from the Rust inventory.
        let c_model = model::build_model(self.inventory);

        // Output pass: render the model into a complete header.
        let header = output::render_header(&engine, &c_model, &self.loader_name, &self.ifndef)?;

        let mut multibuf = Multibuf::new();
        multibuf.add_buffer(&self.filename, header);
        Ok(multibuf)
    }
}
