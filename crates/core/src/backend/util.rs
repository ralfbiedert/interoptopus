//! Helpers for backend authors.

use crate::lang::{Function, Type, VariantKind};
use crate::pattern::TypePattern;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

/// Converts an internal name like `fn() -> X` to a safe name like `fn_rval_x`
///
/// # Example
///
/// ```
/// use interoptopus::backend::safe_name;
///
/// assert_eq!(safe_name("fn(u32) -> u8"), "fn_u32_rval_u8");
/// ```
#[must_use]
pub fn safe_name(name: &str) -> String {
    let mut rval = name.to_string();

    rval = rval.replace("fn(", "fn_");
    rval = rval.replace("-> ()", "");
    rval = rval.replace("->", "rval");
    rval = rval.replace('(', "");
    rval = rval.replace(')', "");
    rval = rval.replace('*', "p");
    rval = rval.replace(',', "_");
    rval = rval.replace(' ', "_");

    rval = rval.trim_end_matches('_').to_string();

    rval
}

// TODO: Create a few unit tests for this.
/// Sorts types so the latter entries will find their dependents earlier in this list.
#[must_use]
pub fn sort_types_by_dependencies(mut types: Vec<Type>) -> Vec<Type> {
    let mut rval = Vec::new();

    // Outer loop keeps iterating while there are still more types to sort.
    //
    // Example of input  = [ F(A), D(E), G(?), C(D(E)), A, B(D(E)), E ]
    //            output = [ G(?), A, E, F(A), D(E), C(D(E)), B(D(E)) ]
    //
    // Where A(B) means A depends on B, and ? is a type that cannot be fulfilled by input.
    //
    // The idea is to keep iterating `types`, removing all entries that either have no dependencies, or which
    // have already been satisfied.
    while !types.is_empty() {
        // Types which have dependents fulfilled; we're going to fill this.
        let mut may_add_this_round = Vec::new();

        // Check any top-level type still in the list.
        for t in &types {
            let needed_to_exist = t.embedded_types();

            let t_is_sufficiently_fulfilled = needed_to_exist.iter().all(|x| {
                // All types exist if they, well, already exist in the output array. In addition, if a type
                // cannot be fulfilled by the remaining types we also skip it (this can happen when filtering
                // incomplete type lists which might be fulfilled by 3rd party user code).
                rval.contains(x) || !types.contains(x)
            });

            // Add `t` if it didn't have any dependencies, or if we already added them.
            if needed_to_exist.is_empty() || t_is_sufficiently_fulfilled {
                may_add_this_round.push(t.clone());
            }
        }

        types.retain(|x| !may_add_this_round.contains(x));
        rval.append(&mut may_add_this_round);
    }

    rval
}

/// Given a number of functions like [`lib_x`, `lib_y`] return the longest common prefix `lib_`.
///
///
/// # Example
///
/// ```rust
/// # use interoptopus::backend::longest_common_prefix;
/// # use interoptopus::lang::{Function, FunctionSignature, Meta};
///
/// let functions = [
///     Function::new("my_lib_f".to_string(), FunctionSignature::default(), Meta::default()),
///     Function::new("my_lib_g".to_string(), FunctionSignature::default(), Meta::default()),
///     Function::new("my_lib_h".to_string(), FunctionSignature::default(), Meta::default()),
/// ];
///
/// assert_eq!(longest_common_prefix(&functions), "my_lib_".to_string());
/// ```
#[must_use]
pub fn longest_common_prefix(functions: &[Function]) -> String {
    let funcs_as_chars = functions.iter().map(|x| x.name().chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut longest_common: Vec<char> = Vec::new();

    if let Some(first) = funcs_as_chars.first() {
        for (i, c) in first.iter().enumerate() {
            for function in &funcs_as_chars {
                if function.get(i).is_none_or(|x| x != c) {
                    return String::from_iter(&longest_common);
                }
            }
            longest_common.push(*c);
        }
    }

    String::from_iter(&longest_common)
}

/// Given some functions and types, return all used and nested types, without duplicates.
#[must_use]
pub fn ctypes_from_functions_types(functions: &[Function], extra_types: &[Type]) -> Vec<Type> {
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
#[allow(clippy::implicit_hasher)]
pub fn ctypes_from_type_recursive(start: &Type, types: &mut HashSet<Type>) {
    types.insert(start.clone());

    match start {
        Type::Composite(inner) => {
            for field in inner.fields() {
                ctypes_from_type_recursive(field.the_type(), types);
            }
        }
        Type::Array(inner) => ctypes_from_type_recursive(inner.array_type(), types),
        Type::FnPointer(inner) => {
            ctypes_from_type_recursive(inner.signature().rval(), types);
            for param in inner.signature().params() {
                ctypes_from_type_recursive(param.the_type(), types);
            }
        }
        Type::ReadPointer(inner) => ctypes_from_type_recursive(inner, types),
        Type::ReadWritePointer(inner) => ctypes_from_type_recursive(inner, types),
        Type::Primitive(_) => {}
        Type::Enum(_) => {}
        Type::Opaque(_) => {}
        // Note, patterns must _NEVER_ add themselves as fallbacks. Instead, each code generator should
        // decide on a case-by-case bases whether it wants to use the type's fallback, or generate an
        // entirely new pattern. The exception to this rule are patterns that can embed arbitrary
        // types; which we need to recursively inspect.
        Type::Pattern(x) => match x {
            TypePattern::AsyncCallback(x) => {
                for field in x.fnpointer().signature().params() {
                    ctypes_from_type_recursive(field.the_type(), types);
                }
            }
            TypePattern::CStrPointer => {}
            TypePattern::NamedCallback(x) => {
                let inner = x.fnpointer();
                ctypes_from_type_recursive(inner.signature().rval(), types);
                for param in inner.signature().params() {
                    ctypes_from_type_recursive(param.the_type(), types);
                }
            }
            TypePattern::Slice(x) => ctypes_from_type_recursive(x.target_type(), types),
            TypePattern::SliceMut(x) => ctypes_from_type_recursive(x.target_type(), types),
            TypePattern::Option(x) => {
                for field in x.fields() {
                    ctypes_from_type_recursive(field.the_type(), types);
                }
            }
            TypePattern::Result(x) => {
                for variant in x.the_enum().variants() {
                    match variant.kind() {
                        VariantKind::Typed(x) => ctypes_from_type_recursive(x, types),
                        VariantKind::Unit(_) => {}
                    }
                }
            }
            TypePattern::Bool => {}
            TypePattern::CChar => {}
            TypePattern::APIVersion => {}
            TypePattern::Utf8String(_) => {}
        },
    }
}

/// Extracts annotated namespace strings.
#[allow(clippy::implicit_hasher)]
pub fn extract_namespaces_from_types(types: &[Type], into: &mut HashSet<String>) {
    for t in types {
        match t {
            Type::Primitive(_) => {}
            Type::Array(_) => {}

            Type::Enum(x) => {
                into.insert(x.meta().namespace().to_string());
            }
            Type::Opaque(x) => {
                into.insert(x.meta().namespace().to_string());
            }
            Type::Composite(x) => {
                into.insert(x.meta().namespace().to_string());
            }
            Type::FnPointer(_) => {}
            Type::ReadPointer(_) => {}
            Type::ReadWritePointer(_) => {}
            Type::Pattern(x) => match x {
                TypePattern::AsyncCallback(x) => {
                    into.insert(x.meta().namespace().to_string());
                }
                TypePattern::CStrPointer => {}
                TypePattern::APIVersion => {}
                TypePattern::Slice(x) => {
                    into.insert(x.meta().namespace().to_string());
                }
                TypePattern::SliceMut(x) => {
                    into.insert(x.meta().namespace().to_string());
                }
                TypePattern::Option(x) => {
                    into.insert(x.meta().namespace().to_string());
                }
                TypePattern::Result(x) => {
                    into.insert(x.meta().namespace().to_string());
                }
                TypePattern::Bool => {}
                TypePattern::CChar => {}
                TypePattern::NamedCallback(_) => {}
                TypePattern::Utf8String(_) => {}
            },
        }
    }
}

#[must_use]
pub fn holds_opaque_without_ref(typ: &Type) -> bool {
    match typ {
        Type::Primitive(_) => false,
        Type::Array(x) => holds_opaque_without_ref(x.array_type()),
        Type::Enum(x) => {
            for variant in x.variants() {
                match variant.kind() {
                    VariantKind::Typed(x) => {
                        if holds_opaque_without_ref(x) {
                            return true;
                        }
                    }
                    VariantKind::Unit(_) => {}
                }
            }
            false
        }
        Type::Opaque(_) => true,
        Type::Composite(x) => {
            for field in x.fields() {
                if holds_opaque_without_ref(field.the_type()) {
                    return true;
                }
            }
            false
        }
        Type::FnPointer(_) => false,
        Type::ReadPointer(_) => false,
        Type::ReadWritePointer(_) => false,
        Type::Pattern(x) => match x {
            TypePattern::CStrPointer => false,
            TypePattern::Utf8String(_) => false,
            TypePattern::APIVersion => false,
            TypePattern::Slice(x) => holds_opaque_without_ref(x.target_type()),
            TypePattern::SliceMut(x) => holds_opaque_without_ref(x.target_type()),
            TypePattern::Option(x) => holds_opaque_without_ref(&x.to_ctype()),
            TypePattern::Result(x) => holds_opaque_without_ref(&x.the_enum().to_ctype()),
            TypePattern::Bool => false,
            TypePattern::CChar => false,
            TypePattern::NamedCallback(_) => false,
            TypePattern::AsyncCallback(_) => false,
        },
    }
}

/// Maps an internal namespace like `common` to a language namespace like `Company.Common`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NamespaceMappings {
    mappings: HashMap<String, String>,
}

impl NamespaceMappings {
    /// Creates a new mapping, assinging namespace id `""` to `default`.
    #[must_use]
    pub fn new(default: &str) -> Self {
        let mut mappings = HashMap::new();
        mappings.insert(String::new(), default.to_string());
        mappings.insert("_global".to_string(), default.to_string());

        Self { mappings }
    }

    /// Adds a mapping between namespace `id` to string `value`.
    #[must_use]
    pub fn add(mut self, id: &str, value: &str) -> Self {
        self.mappings.insert(id.to_string(), value.to_string());
        self
    }

    /// Returns the default namespace mapping
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn default_namespace(&self) -> &str {
        self.get("").expect("This must exist")
    }

    /// Obtains a mapping for the given ID.
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&str> {
        self.mappings.get(id).map(String::as_str)
    }
}

/// Allows, for example, `my_id` to be converted to `MyId`.
pub struct IdPrettifier {
    tokens: Vec<String>,
}

impl IdPrettifier {
    /// Creates a new prettifier from a `my_name` identifier.
    #[must_use]
    pub fn from_rust_lower(id: &str) -> Self {
        Self { tokens: id.split('_').map(std::string::ToString::to_string).collect() }
    }

    #[must_use]
    pub fn to_camel_case(&self) -> String {
        self.tokens
            .iter()
            .map(|x| {
                x.chars()
                    .enumerate()
                    .map(|(i, x)| if i == 0 { x.to_ascii_uppercase() } else { x })
                    .collect::<String>()
            })
            .collect::<String>()
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
#[must_use]
pub fn is_global_type(t: &Type) -> bool {
    match t {
        Type::Primitive(_) => true,
        Type::Array(x) => is_global_type(x.array_type()),
        Type::Enum(x) => {
            for variant in x.variants() {
                match variant.kind() {
                    VariantKind::Typed(x) => {
                        if !is_global_type(x) {
                            return false;
                        }
                    }
                    VariantKind::Unit(_) => {}
                }
            }
            true
        }
        Type::Opaque(_) => false,
        Type::Composite(_) => false,
        Type::FnPointer(_) => false,
        Type::ReadPointer(x) => is_global_type(x),
        Type::ReadWritePointer(x) => is_global_type(x),
        Type::Pattern(x) => match x {
            TypePattern::CStrPointer => true,
            TypePattern::APIVersion => false,
            TypePattern::Slice(x) => is_global_type(x.target_type()),
            TypePattern::SliceMut(x) => is_global_type(x.target_type()),
            TypePattern::Option(x) => x.fields().iter().all(|x| is_global_type(x.the_type())),
            TypePattern::Result(x) => is_global_type(&x.the_enum().to_ctype()),
            TypePattern::Bool => true,
            TypePattern::CChar => true,
            TypePattern::NamedCallback(_) => false,
            TypePattern::AsyncCallback(_) => false,
            TypePattern::Utf8String(_) => true,
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
#[doc(hidden)]
#[macro_export]
macro_rules! here {
    () => {
        concat!(file!(), ":", line!())
    };
}

/// Capitalizes the first letter of a string.
#[must_use]
pub fn capitalize_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

#[cfg(test)]
mod test {
    use crate::backend::util::IdPrettifier;

    #[test]
    fn is_pretty() {
        assert_eq!(IdPrettifier::from_rust_lower("hello_world").to_camel_case(), "HelloWorld");
        assert_eq!(IdPrettifier::from_rust_lower("single").to_camel_case(), "Single");
    }
}
