mod error;
mod render;
mod testing;
mod writer;

pub use error::Error;
pub use render::indent_all_with;
pub use testing::assert_file_unchanged;
pub use writer::{FOUR_SPACES, IndentWriter, WriteFor};
