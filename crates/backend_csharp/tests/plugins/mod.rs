use std::path::{Path, PathBuf};

mod concurrent;
/// Returns the path to a compiled plugin DLL.
fn plugin_path_for(path: impl AsRef<Path>) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/reference_plugins/_plugins").join(path.as_ref())
}
