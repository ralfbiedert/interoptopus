//! Helpers for backend authors.

use crate::lang::c::{CType, Function};
use crate::patterns::TypePattern;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

/// Converts an internal name like `fn() -> X` to a safe name like `fn_rval_x`
///
/// # Example
///
/// ```
/// use interoptopus::util::safe_name;
///
/// assert_eq!(safe_name("fn(u32) -> u8"), "fn_u32_rval_u8");
/// ```
pub fn safe_name(name: &str) -> String {
    let mut rval = name.to_string();

    rval = rval.replace("fn(", "fn_");
    rval = rval.replace("-> ()", "");
    rval = rval.replace("->", "rval");
    rval = rval.replace("(", "");
    rval = rval.replace(")", "");
    rval = rval.replace("*", "p");
    rval = rval.replace(",", "_");
    rval = rval.replace(" ", "_");

    rval = rval.trim_end_matches('_').to_string();

    rval
}

/// Sorts types so the latter entries will find their dependents earlier in this list.
pub fn sort_types_by_dependencies(mut types: Vec<CType>) -> Vec<CType> {
    let mut rval = Vec::new();

    // Ugly but was fast to write.
    // TODO: This is guaranteed to terminate by proof of running it at least once on my machine.
    while !types.is_empty() {
        let mut this_round = Vec::new();

        for t in &types {
            let embedded = t.embedded_types();
            let all_exist = embedded.iter().all(|x| rval.contains(x));

            if embedded.is_empty() || all_exist {
                this_round.push(t.clone());
            }
        }

        types.retain(|x| !this_round.contains(x));
        rval.append(&mut this_round);
    }

    rval
}

/// Given a number of functions like [`lib_x`, `lib_y`] return the longest common prefix `lib_`.
///
///
/// # Example
///
/// ```rust
/// # use interoptopus::lang::c::{Function, FunctionSignature, Meta};
/// # use interoptopus::util::longest_common_prefix;
///
/// let functions = [
///     Function::new("my_lib_f".to_string(), FunctionSignature::default(), Meta::default()),
///     Function::new("my_lib_g".to_string(), FunctionSignature::default(), Meta::default()),
///     Function::new("my_lib_h".to_string(), FunctionSignature::default(), Meta::default()),
/// ];
///
/// assert_eq!(longest_common_prefix(&functions), "my_lib_".to_string());
/// ```
pub fn longest_common_prefix(functions: &[Function]) -> String {
    let funcs_as_chars = functions.iter().map(|x| x.name().chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut longest_common: Vec<char> = Vec::new();

    if let Some(first) = funcs_as_chars.first() {
        for (i, c) in first.iter().enumerate() {
            for function in &funcs_as_chars {
                if !function.get(i).map(|x| x == c).unwrap_or(false) {
                    return String::from_iter(&longest_common);
                }
            }
            longest_common.push(*c);
        }
    }

    String::from_iter(&longest_common)
}

/// Given some functions and types, return all used and nested types, without duplicates.
pub(crate) fn ctypes_from_functions_types(functions: &[Function], extra_types: &[CType]) -> Vec<CType> {
    let mut types = HashSet::new();

    for function in functions {
        ctypes_from_type_recursive(function.signature().rval(), &mut types);

        for param in function.signature().params() {
            ctypes_from_type_recursive(param.the_type(), &mut types);
        }
    }

    for ty in extra_types {
        ctypes_from_type_recursive(ty, &mut types);
    }

    types.iter().cloned().collect()
}

/// Recursive helper for [`ctypes_from_functions_types`].
pub(crate) fn ctypes_from_type_recursive(start: &CType, types: &mut HashSet<CType>) {
    types.insert(start.clone());

    match start {
        CType::Composite(inner) => {
            for field in inner.fields() {
                ctypes_from_type_recursive(&field.the_type(), types);
            }
        }
        CType::Array(inner) => ctypes_from_type_recursive(&inner.array_type(), types),
        CType::FnPointer(inner) => {
            ctypes_from_type_recursive(inner.signature().rval(), types);
            for param in inner.signature().params() {
                ctypes_from_type_recursive(param.the_type(), types);
            }
        }
        CType::ReadPointer(inner) => ctypes_from_type_recursive(inner, types),
        CType::ReadWritePointer(inner) => ctypes_from_type_recursive(inner, types),
        CType::Primitive(_) => {}
        CType::Enum(_) => {}
        CType::Opaque(_) => {}
        // Note, patterns must _NEVER_ add themselves as fallbacks. Instead, each code generator should
        // decide on a case-by-case bases whether it wants to use the type's fallback, or generate an
        // entirely new pattern. The exception to this rule are patterns that can embed arbitrary
        // types; which we need to recursively inspect.
        CType::Pattern(x) => match x {
            TypePattern::AsciiPointer => {}
            TypePattern::FFIErrorEnum(_) => {}
            TypePattern::NamedCallback(x) => {
                let inner = x.fnpointer();
                ctypes_from_type_recursive(inner.signature().rval(), types);
                for param in inner.signature().params() {
                    ctypes_from_type_recursive(param.the_type(), types);
                }
            }
            TypePattern::Slice(x) => {
                for field in x.fields() {
                    ctypes_from_type_recursive(field.the_type(), types);
                }
            }
            TypePattern::SliceMut(x) => {
                for field in x.fields() {
                    ctypes_from_type_recursive(field.the_type(), types);
                }
            }
            TypePattern::Option(x) => {
                for field in x.fields() {
                    ctypes_from_type_recursive(field.the_type(), types);
                }
            }
            TypePattern::Bool => {}
            TypePattern::APIVersion => {}
        },
    }
}

/// Extracts annotated namespace strings.
pub(crate) fn extract_namespaces_from_types(types: &[CType], into: &mut HashSet<String>) {
    for t in types {
        match t {
            CType::Primitive(_) => {}
            CType::Array(_) => {}

            CType::Enum(x) => {
                into.insert(x.meta().namespace().to_string());
            }
            CType::Opaque(x) => {
                into.insert(x.meta().namespace().to_string());
            }
            CType::Composite(x) => {
                into.insert(x.meta().namespace().to_string());
            }
            CType::FnPointer(_) => {}
            CType::ReadPointer(_) => {}
            CType::ReadWritePointer(_) => {}
            CType::Pattern(x) => match x {
                TypePattern::AsciiPointer => {}
                TypePattern::APIVersion => {}
                TypePattern::FFIErrorEnum(x) => {
                    into.insert(x.the_enum().meta().namespace().to_string());
                }
                TypePattern::Slice(x) => {
                    into.insert(x.meta().namespace().to_string());
                }
                TypePattern::SliceMut(x) => {
                    into.insert(x.meta().namespace().to_string());
                }
                TypePattern::Option(x) => {
                    into.insert(x.meta().namespace().to_string());
                }
                TypePattern::Bool => {}
                TypePattern::NamedCallback(_) => {}
            },
        }
    }
}

/// Maps an internal namespace like `common` to a language namespace like `Company.Common`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NamespaceMappings {
    mappings: HashMap<String, String>,
}

impl NamespaceMappings {
    /// Creates a new mapping, assinging namespace id `""` to `default`.
    pub fn new(default: &str) -> Self {
        let mut mappings = HashMap::new();
        mappings.insert("".to_string(), default.to_string());
        mappings.insert("_global".to_string(), default.to_string());

        Self { mappings }
    }

    /// Adds a mapping between namespace `id` to string `value`.
    pub fn add(mut self, id: &str, value: &str) -> Self {
        self.mappings.insert(id.to_string(), value.to_string());
        self
    }

    /// Returns the default namespace mapping
    pub fn default_namespace(&self) -> &str {
        self.get("").expect("This must exist")
    }

    /// Obtains a mapping for the given ID.
    pub fn get(&self, id: &str) -> Option<&str> {
        self.mappings.get(id).map(|x| x.as_str())
    }
}

/// Allows, for example, `my_id` to be converted to `MyId`.
pub struct IdPrettifier {
    tokens: Vec<String>,
}

impl IdPrettifier {
    /// Creates a new prettifier from a `my_name` identifier.
    pub fn from_rust_lower(id: &str) -> Self {
        Self {
            tokens: id.split('_').map(|x| x.to_string()).collect(),
        }
    }

    pub fn to_camel_case(&self) -> String {
        self.tokens
            .iter()
            .map(|x| {
                x.chars()
                    .enumerate()
                    .map(|(i, x)| if i == 0 { x.to_ascii_uppercase() } else { x })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("")
    }
}

/// Checks whether the given type should be "the same type everywhere".
///
/// In complex setups we sometimes want to use types between two (otherwise unrelated) bindings.
/// For example, we would like to produce a `FFISlice<u8>` in library A, and consume that in
/// library B. On the other hand, a  `FFISlice<MyStruct>` is not something everyone should know of.
///
/// For our bindings to know whether some types should go to a shared namespace this function
/// will inform them whether the underlying type should be shared.
///
///
pub fn is_global_type(t: &CType) -> bool {
    match t {
        CType::Primitive(_) => true,
        CType::Array(x) => is_global_type(x.array_type()),
        CType::Enum(_) => false,
        CType::Opaque(_) => false,
        CType::Composite(_) => false,
        CType::FnPointer(_) => false,
        CType::ReadPointer(x) => is_global_type(x),
        CType::ReadWritePointer(x) => is_global_type(x),
        CType::Pattern(x) => match x {
            TypePattern::AsciiPointer => true,
            TypePattern::APIVersion => false,
            TypePattern::FFIErrorEnum(_) => false,
            TypePattern::Slice(x) => x.fields().iter().all(|x| is_global_type(x.the_type())),
            TypePattern::SliceMut(x) => x.fields().iter().all(|x| is_global_type(x.the_type())),
            TypePattern::Option(x) => x.fields().iter().all(|x| is_global_type(x.the_type())),
            TypePattern::Bool => true,
            TypePattern::NamedCallback(_) => false,
        },
    }
}

/// Debug macro resolving to the current file and line number.
///
/// ```
/// use interoptopus::here;
///
/// println!(here!())
/// ```
#[macro_export]
macro_rules! here {
    () => {
        concat!(file!(), ":", line!())
    };
}

/// Logs an error if compiled with feature `log`.
#[cfg(feature = "log")]
#[inline(always)]
pub fn log_error<S: AsRef<str>, F: Fn() -> S>(f: F) {
    log::error!("{}", f().as_ref());
}

/// Logs an error if compiled with feature `log`.
#[cfg(not(feature = "log"))]
#[inline(always)]
pub fn log_error<S: AsRef<str>, F: Fn() -> S>(_f: F) {}

#[cfg(test)]
mod test {
    use crate::util::IdPrettifier;

    #[test]
    fn is_pretty() {
        assert_eq!(IdPrettifier::from_rust_lower("hello_world").to_camel_case(), "HelloWorld");
        assert_eq!(IdPrettifier::from_rust_lower("single").to_camel_case(), "Single");
    }
}
