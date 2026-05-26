use interoptopus::{extra_type, ffi, function};
use interoptopus_csharp::pass::normalize_blank_lines;

#[ffi]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[ffi]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[ffi]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[ffi(export = unique)]
pub fn ws_sum_u32(a: u32, b: u32) -> u32 {
    a + b
}

/// Asserts that the generated output never contains more than one consecutive
/// blank line (i.e., no runs of 3+ newlines).
#[test]
fn no_consecutive_empty_lines() {
    use interoptopus::inventory::RustInventory;
    use interoptopus_csharp::RustLibrary;
    use interoptopus_csharp::config::HeaderConfig;
    use interoptopus_csharp::dispatch::Dispatch;
    use interoptopus_csharp::output::Target;
    use interoptopus::lang::meta::FileEmission;

    let mut inventory = RustInventory::new();
    let _ = inventory.register(extra_type!(Vec2));
    let _ = inventory.register(extra_type!(Vec3));
    let _ = inventory.register(extra_type!(Color));
    let _ = inventory.register(function!(ws_sum_u32));
    let inventory = inventory.validate();

    let multibuf = RustLibrary::builder(inventory)
        .dispatch(Dispatch::custom(|x, _| match x.emission {
            FileEmission::Common => Target::new("Interop.Common.cs", "My.Company.Common"),
            FileEmission::Default | FileEmission::CustomModule(_) => Target::new("Interop.cs", "My.Company"),
        }))
        .headers(HeaderConfig { emit_version: false })
        .build()
        .process()
        .unwrap();

    for (name, content) in multibuf.iter() {
        // Find all occurrences of 3+ consecutive newlines
        let mut violations = Vec::new();
        for (i, window) in content.as_bytes().windows(3).enumerate() {
            if window == b"\n\n\n" {
                let line = content[..i].matches('\n').count() + 1;
                violations.push(line);
            }
        }
        // Deduplicate adjacent reports (a run of 4 newlines reports twice)
        violations.dedup();

        assert!(
            violations.is_empty(),
            "File '{name}' contains consecutive empty lines at lines: {violations:?}\n\
             Output:\n{content}"
        );
    }
}

#[test]
fn normalize_collapses_lf_blank_lines() {
    assert_eq!(normalize_blank_lines("a\n\n\nb"), "a\n\nb\n");
    assert_eq!(normalize_blank_lines("a\n\n\n\nb"), "a\n\nb\n");
    assert_eq!(normalize_blank_lines("a\n\nb"), "a\n\nb\n");
}

#[test]
fn normalize_collapses_crlf_blank_lines() {
    assert_eq!(normalize_blank_lines("a\r\n\r\n\r\nb"), "a\r\n\r\nb\n");
    assert_eq!(normalize_blank_lines("a\r\n\r\n\r\n\r\nb"), "a\r\n\r\nb\n");
    assert_eq!(normalize_blank_lines("a\r\n\r\nb"), "a\r\n\r\nb\n");
}
