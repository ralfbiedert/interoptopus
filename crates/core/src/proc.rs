/// Strips module paths from a fully-qualified Rust type name, preserving generic structure.
///
/// For example, `"my_crate::module::Struct<alloc::string::String>"` becomes `"Struct<String>"`.
/// Handles nested generics and multiple type parameters.
#[must_use]
pub fn strip_module_paths(full: &str) -> String {
    // Find the first top-level '<' (not nested)
    let mut depth = 0usize;
    let mut angle_pos = None;
    for (i, b) in full.bytes().enumerate() {
        match b {
            b'<' if depth == 0 => {
                angle_pos = Some(i);
                break;
            }
            b'<' => depth += 1,
            b'>' => depth -= 1,
            _ => {}
        }
    }

    if let Some(pos) = angle_pos {
        // Split into base path and <...> suffix
        let base = &full[..pos];
        let rest = &full[pos..]; // includes '<' and '>'

        // Strip module path from base: take last :: segment
        let short_base = base.rsplit("::").next().unwrap_or(base);

        // Recursively strip inside angle brackets
        // rest is "<inner_content>" — strip the outer < >
        let inner = &rest[1..rest.len() - 1];

        // Split inner by top-level commas and strip each part
        let mut parts = Vec::new();
        let mut part_start = 0;
        let mut d = 0usize;
        for (i, b) in inner.bytes().enumerate() {
            match b {
                b'<' => d += 1,
                b'>' => d -= 1,
                b',' if d == 0 => {
                    parts.push(inner[part_start..i].trim());
                    part_start = i + 1;
                }
                _ => {}
            }
        }
        parts.push(inner[part_start..].trim());

        let stripped_parts: Vec<String> = parts.iter().map(|p| strip_module_paths(p)).collect();
        format!("{}<{}>", short_base, stripped_parts.join(", "))
    } else {
        // No generics — just strip the module path
        full.rsplit("::").next().unwrap_or(full).to_string()
    }
}
