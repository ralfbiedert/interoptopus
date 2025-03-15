//! Utilities for backends.

mod testing;
mod util;
mod writer;

pub use testing::assert_file_matches_generated;
pub use util::*;
pub use writer::{FOUR_SPACES, IndentWriter, WriteFor};
