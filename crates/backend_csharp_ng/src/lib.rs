pub mod dispatch;
pub mod id;
pub mod lang;
pub mod plugin;
pub mod stage;
pub mod template;

mod error;
mod macros;
mod pipeline;

pub use error::Error;
pub use pipeline::{RustPlugin, RustPluginConfig};
