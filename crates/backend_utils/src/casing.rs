//! Utilities for converting Rust type names into valid target-language identifiers.

/// Converts a name to PascalCase by capitalizing after `_` and space boundaries.
///
/// # Examples
/// - `my_type` → `MyType`
/// - `vec3_f32` → `Vec3F32`
pub fn rust_to_pascal(name: &str) -> String {
    let mut result = String::with_capacity(name.len());
    let mut capitalize_next = true;
    for c in name.chars() {
        if c == '_' || c == ' ' {
            capitalize_next = true;
        } else if capitalize_next {
            result.extend(c.to_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    result
}

/// Sanitizes a Rust type name into a valid target-language identifier.
///
/// Strips angle brackets, commas, semicolons, square brackets, and spaces,
/// then PascalCases the fragments.
///
/// # Examples
/// - `Weird2<u8, 5>` → `Weird2U85`
/// - `[u8; 5]` → `U85`
/// - `MyStruct` → `MyStruct` (unchanged)
pub fn sanitize_rust_name(name: &str) -> String {
    let mut result = String::with_capacity(name.len());
    let mut capitalize_next = true;

    for c in name.chars() {
        match c {
            '<' | '>' | ',' | ';' | '[' | ']' | ' ' => {
                capitalize_next = true;
            }
            '_' => {
                capitalize_next = true;
            }
            _ if capitalize_next => {
                result.extend(c.to_uppercase());
                capitalize_next = false;
            }
            _ => {
                result.push(c);
            }
        }
    }

    result
}

/// Converts a PascalCase or camelCase name to snake_case.
///
/// Inserts `_` before each uppercase letter that follows a lowercase letter or digit,
/// and lowercases the result.
///
/// # Examples
/// - `ServiceBasic` → `service_basic`
/// - `MyHTTPService` → `my_h_t_t_p_service`
/// - `Vec3` → `vec3`
pub fn pascal_to_snake(name: &str) -> String {
    let mut result = String::with_capacity(name.len() + 4);
    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            let prev = name.as_bytes()[i - 1];
            if prev.is_ascii_lowercase() || prev.is_ascii_digit() {
                result.push('_');
            }
        }
        result.extend(c.to_lowercase());
    }
    result
}

/// Sanitizes a Rust delegate/fn-pointer name into a valid target-language identifier.
///
/// Strips the `extern "C" ` prefix, removes void return types (`-> ()`),
/// and treats all non-alphanumeric characters as word boundaries for PascalCase.
///
/// # Examples
/// - `extern "C" fn(u8) -> u8` → `FnU8U8`
/// - `extern "C" fn(Vec3f32) -> ()` → `FnVec3f32`
/// - `fn(u32, u32) -> bool` → `FnU32U32Bool`
pub fn sanitize_delegate_name(name: &str) -> String {
    let stripped = name.strip_prefix("extern \"C\" ").unwrap_or(name);

    // Strip void return type entirely
    let stripped = stripped.replace("-> ()", "");

    let mut result = String::with_capacity(stripped.len());
    let mut capitalize_next = true;

    for c in stripped.chars() {
        if c.is_alphanumeric() {
            if capitalize_next {
                result.extend(c.to_uppercase());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        } else {
            capitalize_next = true;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pascal_to_snake() {
        assert_eq!(pascal_to_snake("ServiceBasic"), "service_basic");
        assert_eq!(pascal_to_snake("Vec3"), "vec3");
        assert_eq!(pascal_to_snake("MyType"), "my_type");
        assert_eq!(pascal_to_snake("A"), "a");
        assert_eq!(pascal_to_snake(""), "");
        assert_eq!(pascal_to_snake("already_snake"), "already_snake");
    }

    #[test]
    fn test_rust_to_pascal() {
        assert_eq!(rust_to_pascal("my_type"), "MyType");
        assert_eq!(rust_to_pascal("vec3_f32"), "Vec3F32");
        assert_eq!(rust_to_pascal("already"), "Already");
        assert_eq!(rust_to_pascal("a_b_c"), "ABC");
    }

    #[test]
    fn test_sanitize_rust_name() {
        assert_eq!(sanitize_rust_name("Weird2<u8, 5>"), "Weird2U85");
        assert_eq!(sanitize_rust_name("[u8; 5]"), "U85");
        assert_eq!(sanitize_rust_name("MyStruct"), "MyStruct");
    }

    #[test]
    fn test_sanitize_delegate_name() {
        assert_eq!(sanitize_delegate_name("extern \"C\" fn(u8) -> u8"), "FnU8U8");
        assert_eq!(sanitize_delegate_name("extern \"C\" fn(Vec3f32) -> ()"), "FnVec3f32");
        assert_eq!(sanitize_delegate_name("fn(u32, u32) -> bool"), "FnU32U32Bool");
    }
}
