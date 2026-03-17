//! Utilities for converting Rust type names into valid target-language identifiers.

/// Converts a name to `PascalCase` by capitalizing after `_` and space boundaries.
///
/// # Examples
/// - `my_type` → `MyType`
/// - `vec3_f32` → `Vec3F32`
#[must_use]
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
/// then `PascalCase`s the fragments.
///
/// # Examples
/// - `Weird2<u8, 5>` → `Weird2U85`
/// - `[u8; 5]` → `U85`
/// - `MyStruct` → `MyStruct` (unchanged)
#[must_use]
pub fn sanitize_rust_name(name: &str) -> String {
    let mut result = String::with_capacity(name.len());
    let mut capitalize_next = true;

    for c in name.chars() {
        match c {
            '<' | '>' | ',' | ';' | '[' | ']' | ' ' | '\'' => {
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

/// Converts a `PascalCase` or `camelCase` name to `snake_case`.
///
/// Inserts `_` before each uppercase letter that follows a lowercase letter or digit,
/// and lowercases the result.
///
/// # Examples
/// - `ServiceBasic` → `service_basic`
/// - `MyHTTPService` → `my_h_t_t_p_service`
/// - `Vec3` → `vec3`
#[must_use]
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

/// Derives a method name from a service function name.
///
/// Given a `PascalCase` type name (e.g. `ServiceBasic`) and a `snake_case` function name
/// (e.g. `service_basic_do_something`), strips the prefix to produce `DoSomething`.
/// If the function name doesn't start with the prefix, the full name is `PascalCase`d.
///
/// # Examples
/// - `("ServiceBasic", "service_basic_do_something")` → `"DoSomething"`
/// - `("ServiceBasic", "unrelated_name")` → `"UnrelatedName"`
#[must_use]
pub fn service_method_name(type_name: &str, fn_name: &str) -> String {
    let prefix = pascal_to_snake(type_name);
    let method_part = fn_name.strip_prefix(&prefix).and_then(|s| s.strip_prefix('_')).unwrap_or(fn_name);
    rust_to_pascal(method_part)
}

/// Sanitizes a Rust delegate/fn-pointer name into a valid target-language identifier.
///
/// Strips the `extern "C" ` prefix, removes void return types (`-> ()`),
/// and treats all non-alphanumeric characters as word boundaries for `PascalCase`.
///
/// # Examples
/// - `extern "C" fn(u8) -> u8` → `FnU8U8`
/// - `extern "C" fn(Vec3f32) -> ()` → `FnVec3f32`
/// - `fn(u32, u32) -> bool` → `FnU32U32Bool`
#[must_use]
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
    fn test_service_method_name() {
        assert_eq!(service_method_name("ServiceBasic", "service_basic_do_something"), "DoSomething");
        assert_eq!(service_method_name("ServiceBasic", "unrelated_name"), "UnrelatedName");
        assert_eq!(service_method_name("MyService", "my_service_new"), "New");
    }

    #[test]
    fn test_sanitize_rust_name() {
        assert_eq!(sanitize_rust_name("Weird2<u8, 5>"), "Weird2U85");
        assert_eq!(sanitize_rust_name("[u8; 5]"), "U85");
        assert_eq!(sanitize_rust_name("MyStruct"), "MyStruct");
        assert_eq!(sanitize_rust_name("Generic<'_, u32>"), "GenericU32");
        assert_eq!(sanitize_rust_name("Weird2<'_, u8, 5>"), "Weird2U85");
        assert_eq!(sanitize_rust_name("Phantom<'_, u8>"), "PhantomU8");
    }

    #[test]
    fn test_sanitize_delegate_name() {
        assert_eq!(sanitize_delegate_name("extern \"C\" fn(u8) -> u8"), "FnU8U8");
        assert_eq!(sanitize_delegate_name("extern \"C\" fn(Vec3f32) -> ()"), "FnVec3f32");
        assert_eq!(sanitize_delegate_name("fn(u32, u32) -> bool"), "FnU32U32Bool");
    }
}
