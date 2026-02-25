//! Pass infrastructure for the C backend.
//!
//! Model passes transform the Rust inventory into the C language model.
//! Output passes render the model into C header fragments via Tera templates.

pub mod model;
pub mod output;
