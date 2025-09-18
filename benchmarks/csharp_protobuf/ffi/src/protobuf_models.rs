#![allow(clippy::absolute_paths)]

pub mod models {
    include!(concat!(env!("OUT_DIR"), "/models.rs"));
}

pub use models::*;
