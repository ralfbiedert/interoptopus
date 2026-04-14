//! Naming strategies for converting Rust identifiers into C names.
//!
//! The [`NamingConfig`] struct controls how each category of identifier
//! (types, enum variants, functions, parameters, constants) is cased,
//! and an optional prefix that is prepended to types and functions.

/// Controls how a Rust identifier is converted into a C name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NamingStyle {
    /// `SCREAMING_SNAKE_CASE` — split into word segments, join with `_`, uppercase.
    /// This is the default for types, enum variants, and constants.
    #[default]
    ScreamingSnake,
    /// `UpperCamelCase` (`PascalCase`) — e.g. `OptionVec2`.
    UpperCamel,
    /// `snake_case` — e.g. `option_vec2`.
    Snake,
    /// Preserve the original Rust name, only replacing characters invalid in
    /// C identifiers with `_`.
    Raw,
}

/// Per-category naming configuration for the C backend.
#[derive(Debug, Clone)]
pub struct NamingConfig {
    /// Naming style for type names (structs, enums, opaque types, patterns).
    pub type_naming: NamingStyle,
    /// Naming style for enum variant names (both simple enums and tagged unions).
    pub enum_variant_naming: NamingStyle,
    /// Naming style for function names.
    pub function_naming: NamingStyle,
    /// Naming style for function parameter names.
    pub function_parameter_naming: NamingStyle,
    /// Naming style for constant names (e.g. `_TAG` suffixed tag enums).
    pub const_naming: NamingStyle,
    /// Optional prefix prepended to type names and function names.
    pub prefix: Option<String>,
}

impl Default for NamingConfig {
    fn default() -> Self {
        Self {
            type_naming: NamingStyle::ScreamingSnake,
            enum_variant_naming: NamingStyle::ScreamingSnake,
            function_naming: NamingStyle::Raw,
            function_parameter_naming: NamingStyle::Raw,
            const_naming: NamingStyle::ScreamingSnake,
            prefix: None,
        }
    }
}

/// Sanitize `name` for C and apply the given [`NamingStyle`].
#[must_use]
pub fn apply_naming_style(name: &str, style: NamingStyle) -> String {
    match style {
        NamingStyle::ScreamingSnake => to_screaming_snake(&sanitize(name)),
        NamingStyle::UpperCamel => to_upper_camel(&sanitize(name)),
        NamingStyle::Snake => to_snake_case(&sanitize(name)),
        NamingStyle::Raw => sanitize(name),
    }
}

/// Prepend `prefix` to `name` if a prefix is set.
#[must_use]
pub fn apply_prefix(name: &str, prefix: &Option<String>) -> String {
    match prefix {
        Some(p) if !p.is_empty() => format!("{p}{name}"),
        _ => name.to_string(),
    }
}

/// Replace characters invalid in C identifiers with `_`, collapse runs of
/// `_`, and strip trailing `_`. Leading underscores are preserved (valid in C).
fn sanitize(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for c in name.chars() {
        if c.is_ascii_alphanumeric() || c == '_' {
            out.push(c);
        } else {
            // Collapse consecutive separators into one `_`.
            if !out.ends_with('_') {
                out.push('_');
            }
        }
    }
    out.trim_end_matches('_').to_string()
}

/// Split a sanitized identifier into word segments at boundaries:
/// - underscores (`my_type` → `my`, `type`)
/// - transitions from lowercase/digit to uppercase (`vec2D` → `vec2`, `D`)
/// - transitions from a run of uppercase to uppercase+lowercase (`HTTPError` → `HTTP`, `Error`)
fn word_segments(name: &str) -> Vec<&str> {
    let mut segments = Vec::new();
    let chars: Vec<char> = name.chars().collect();
    let mut start = 0;

    for i in 0..chars.len() {
        if chars[i] == '_' {
            if i > start {
                segments.push(&name[start..i]);
            }
            start = i + 1;
            continue;
        }
        if i > start && chars[i].is_ascii_uppercase() {
            let prev = chars[i - 1];
            // lowercase/digit → uppercase: new word
            if prev.is_ascii_lowercase() || prev.is_ascii_digit() {
                segments.push(&name[start..i]);
                start = i;
            }
            // Run of uppercase followed by lowercase: split before the last
            // uppercase char. E.g. "HTTPError" → ["HTTP", "Error"].
            else if prev.is_ascii_uppercase()
                && let Some(&next) = chars.get(i + 1)
                && next.is_ascii_lowercase()
                && i > start + 1
            {
                segments.push(&name[start..i]);
                start = i;
            }
        }
    }
    if start < chars.len() {
        // Skip trailing underscores that might leave empty segments
        let tail = &name[start..];
        let tail = tail.trim_end_matches('_');
        if !tail.is_empty() {
            segments.push(tail);
        }
    }
    segments
}

/// Convert to `UpperCamelCase`: capitalize the first letter of each word segment.
fn to_upper_camel(name: &str) -> String {
    word_segments(name)
        .iter()
        .map(|seg| {
            let mut chars = seg.chars();
            match chars.next() {
                Some(first) => {
                    let mut s = first.to_ascii_uppercase().to_string();
                    s.extend(chars.map(|c| c.to_ascii_lowercase()));
                    s
                }
                None => String::new(),
            }
        })
        .collect()
}

/// Convert to `SCREAMING_SNAKE_CASE`: uppercase each word segment, join with `_`.
fn to_screaming_snake(name: &str) -> String {
    word_segments(name).iter().map(|seg| seg.to_ascii_uppercase()).collect::<Vec<_>>().join("_")
}

/// Convert to `snake_case`: lowercase each word segment, join with `_`.
fn to_snake_case(name: &str) -> String {
    word_segments(name).iter().map(|seg| seg.to_ascii_lowercase()).collect::<Vec<_>>().join("_")
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── apply_naming_style ──

    #[test]
    fn screaming_snake_simple() {
        assert_eq!(apply_naming_style("Color", NamingStyle::ScreamingSnake), "COLOR");
    }

    #[test]
    fn screaming_snake_generic() {
        assert_eq!(apply_naming_style("Option<Vec2>", NamingStyle::ScreamingSnake), "OPTION_VEC2");
    }

    #[test]
    fn screaming_snake_preserves_underscores() {
        assert_eq!(apply_naming_style("my_type", NamingStyle::ScreamingSnake), "MY_TYPE");
    }

    #[test]
    fn upper_camel_from_snake() {
        assert_eq!(apply_naming_style("option_vec2", NamingStyle::UpperCamel), "OptionVec2");
    }

    #[test]
    fn upper_camel_from_generic() {
        assert_eq!(apply_naming_style("Option<Vec2>", NamingStyle::UpperCamel), "OptionVec2");
    }

    #[test]
    fn upper_camel_already_camel() {
        assert_eq!(apply_naming_style("OptionVec2", NamingStyle::UpperCamel), "OptionVec2");
    }

    #[test]
    fn upper_camel_all_caps() {
        assert_eq!(apply_naming_style("HTTP", NamingStyle::UpperCamel), "Http");
    }

    #[test]
    fn snake_from_camel() {
        assert_eq!(apply_naming_style("OptionVec2", NamingStyle::Snake), "option_vec2");
    }

    #[test]
    fn snake_from_generic() {
        assert_eq!(apply_naming_style("Option<Vec2>", NamingStyle::Snake), "option_vec2");
    }

    #[test]
    fn snake_already_snake() {
        assert_eq!(apply_naming_style("my_type", NamingStyle::Snake), "my_type");
    }

    #[test]
    fn raw_preserves_case() {
        assert_eq!(apply_naming_style("MyType", NamingStyle::Raw), "MyType");
    }

    #[test]
    fn raw_sanitizes_generics() {
        assert_eq!(apply_naming_style("Option<Vec2>", NamingStyle::Raw), "Option_Vec2");
    }

    #[test]
    fn raw_snake_case_passthrough() {
        assert_eq!(apply_naming_style("my_function", NamingStyle::Raw), "my_function");
    }

    #[test]
    fn screaming_snake_from_camel() {
        assert_eq!(apply_naming_style("OptionVec2", NamingStyle::ScreamingSnake), "OPTION_VEC2");
    }

    #[test]
    fn screaming_snake_acronym() {
        assert_eq!(apply_naming_style("HTTPError", NamingStyle::ScreamingSnake), "HTTP_ERROR");
    }

    #[test]
    fn screaming_snake_single_word() {
        assert_eq!(apply_naming_style("color", NamingStyle::ScreamingSnake), "COLOR");
    }

    // ── apply_prefix ──

    #[test]
    fn prefix_some() {
        assert_eq!(apply_prefix("Color", &Some("mylib_".into())), "mylib_Color");
    }

    #[test]
    fn prefix_none() {
        assert_eq!(apply_prefix("Color", &None), "Color");
    }

    #[test]
    fn prefix_empty_string() {
        assert_eq!(apply_prefix("Color", &Some(String::new())), "Color");
    }

    // ── word_segments ──

    #[test]
    fn segments_camel() {
        assert_eq!(word_segments("OptionVec2"), vec!["Option", "Vec2"]);
    }

    #[test]
    fn segments_snake() {
        assert_eq!(word_segments("option_vec2"), vec!["option", "vec2"]);
    }

    #[test]
    fn segments_all_caps_suffix() {
        assert_eq!(word_segments("HTTPError"), vec!["HTTP", "Error"]);
    }

    #[test]
    fn segments_single_word() {
        assert_eq!(word_segments("color"), vec!["color"]);
    }

    #[test]
    fn segments_empty() {
        assert_eq!(word_segments(""), Vec::<&str>::new());
    }

    #[test]
    fn segments_only_underscores() {
        assert_eq!(word_segments("___"), Vec::<&str>::new());
    }

    #[test]
    fn segments_leading_underscore() {
        assert_eq!(word_segments("_foo_bar"), vec!["foo", "bar"]);
    }

    #[test]
    fn segments_consecutive_underscores() {
        assert_eq!(word_segments("my__type"), vec!["my", "type"]);
    }

    #[test]
    fn segments_digits_in_middle() {
        assert_eq!(word_segments("Vec2D"), vec!["Vec2", "D"]);
    }

    #[test]
    fn segments_all_uppercase() {
        assert_eq!(word_segments("HTTP"), vec!["HTTP"]);
    }

    // ── sanitize ──

    #[test]
    fn sanitize_collapses_separators() {
        assert_eq!(sanitize("A<B<C>>"), "A_B_C");
    }

    #[test]
    fn sanitize_trailing_special() {
        assert_eq!(sanitize("Foo<>"), "Foo");
    }

    #[test]
    fn sanitize_empty() {
        assert_eq!(sanitize(""), "");
    }
}
