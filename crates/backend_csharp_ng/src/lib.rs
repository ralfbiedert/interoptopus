pub mod dispatch;
pub mod lang;
pub mod output;
pub mod pass;
pub mod plugin;
pub mod template;

mod error;
mod macros;
mod model;
mod pipeline;

pub use error::Error;
pub use pipeline::{RustLibrary, RustLibraryConfig};
