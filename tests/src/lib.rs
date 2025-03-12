pub mod backend_c;
pub mod backend_cpython;
pub mod backend_csharp;

pub use tempfile::tempdir;

/// Set this to `true` if you want to update bindings.
pub static UPDATE_BINDINGS: bool = true;

#[macro_export]
macro_rules! validate_output {
    ($folder:expr, $file:expr, $generated:expr) => {
        let file = format!("{}/{}", $folder, $file);

        if $crate::UPDATE_BINDINGS {
            std::fs::write(file, $generated).unwrap();
        } else {
            let expected = std::fs::read_to_string(file)?;
            assert_eq!($generated, expected);
        }
    };
}
