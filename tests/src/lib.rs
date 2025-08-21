pub mod backend_c;
pub mod backend_cpython;
pub mod backend_csharp;

pub use tempfile::tempdir;

// Env variable, set it to any value to regenerate the bindings.
pub const UPDATE_BINDINGS: &str = "INTEROPTOPUS_UPDATE_BINDINGS";
pub const IGNORED_LINES: &[&str; 1] = &["// Builder"];

#[macro_export]
macro_rules! validate_output {
    ($folder:expr, $file:expr, $generated:expr) => {
        let file = format!("{}/{}", $folder, $file);

        if std::env::var($crate::UPDATE_BINDINGS).is_ok() {
            ::std::fs::write(file, $generated).unwrap();
        } else {
            let expected = ::std::fs::read_to_string(file.clone())?;
            for (i, (actual_line, expected_line)) in $generated.lines().zip(expected.lines()).enumerate() {
                // Mild hack to ignore certain lines.
                if $crate::IGNORED_LINES.iter().any(|x| expected_line.starts_with(x)) {
                    continue;
                }
                assert_eq!(actual_line, expected_line, "Difference {}:{}", file, i);
            }
        }
    };
}
