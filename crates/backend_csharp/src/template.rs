//! Loads the embedded Tera templates used for C# code generation.

use interoptopus_backends::template::TemplateEngine;

// Include the tar file that was created by build.rs
const ASSET_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/templates.tar"));

/// Returns the built-in C# template engine with all embedded `.cs` templates loaded.
#[must_use]
pub fn templates() -> TemplateEngine {
    TemplateEngine::from_bytes(ASSET_BYTES).expect("Assets must exist")
}
