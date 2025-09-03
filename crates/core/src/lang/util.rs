//! Helpers for type manipulations.

/// Capitalizes the first letter of a string.
#[must_use]
#[allow(clippy::redundant_pub_crate)]
pub(crate) fn capitalize_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// For some functions `lib_x`, `lib_y` return the longest common prefix `lib_`.
///
///
/// # Example
///
/// ```rust
/// # use interoptopus::lang::{Function, Signature, Meta};
/// # use interoptopus::lang::util::longest_common_prefix;
///
/// let functions = [
///     Function::new("my_lib_f".to_string(), Signature::default(), Meta::default(), vec![]),
///     Function::new("my_lib_g".to_string(), Signature::default(), Meta::default(), vec![]),
///     Function::new("my_lib_h".to_string(), Signature::default(), Meta::default(), vec![]),
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

use crate::lang::{Function, Type, VariantKind, WirePayload};
use crate::pattern::TypePattern;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::slice::from_ref;

/// Converts an internal name like `fn() -> X` to a safe name like `fn_rval_x`
///
/// # Example
///
/// ```
/// use interoptopus::lang::util::safe_name;
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

/// Return all used and nested types, without duplicates.
#[must_use]
pub(crate) fn types_from_functions_types(functions: &[Function], extra_types: &[Type]) -> Vec<Type> {
    let mut types = HashSet::new();

    for function in functions {
        // Add only non-Wired types from function arguments and return value.
        // These will be ffi types. Domain types are added separately below.

        types_from_type_nowire_recursive(function.signature().rval(), &mut types);

        for param in function.signature().params() {
            types_from_type_nowire_recursive(param.the_type(), &mut types);
        }

        if function.is_wired() {
            for ty in &function.domain_types() {
                types_from_type_recursive(ty, &mut types);
            }
        }
    }

    for ty in extra_types {
        types_from_type_recursive(ty, &mut types);
    }

    types.iter().cloned().collect()
}

/// Given a type, returns all nested types used by it.
#[allow(clippy::implicit_hasher)]
#[allow(clippy::redundant_pub_crate)]
#[must_use]
pub fn types_from_type(start: &Type) -> Vec<Type> {
    types_from_functions_types(&[], from_ref(start))
}

#[derive(PartialEq)]
enum TypeChoice {
    IncludeWire,
    DontIncludeWire,
}

/// Extract all types, without skipping Wire<T>
#[allow(clippy::redundant_pub_crate)]
pub(crate) fn types_from_type_recursive(start: &Type, types: &mut HashSet<Type>) {
    types_from_type_recursive_inner(start, types, TypeChoice::IncludeWire);
}

/// Skip Wire<T> at the root, extracting only Domain types
#[allow(clippy::redundant_pub_crate)]
pub(crate) fn types_from_type_nowire_recursive(start: &Type, types: &mut HashSet<Type>) {
    types_from_type_recursive_inner(start, types, TypeChoice::DontIncludeWire);
}

/// Recursively checks.
#[allow(clippy::implicit_hasher)]
#[allow(clippy::needless_pass_by_value, reason = "clippy is mad")]
fn types_from_type_recursive_inner(start: &Type, types: &mut HashSet<Type>, choice: TypeChoice) {
    if choice == TypeChoice::IncludeWire || !matches!(start, Type::Wire(_)) {
        types.insert(start.clone());
    }

    match start {
        Type::Composite(inner) => {
            for field in inner.fields() {
                types_from_type_recursive(field.the_type(), types);
            }
        }
        // a Wired struct knows how to serialize itself and contained types,
        // there's never a need to embed Wire<T> inside a struct
        Type::Wire(inner) => {
            for field in inner.fields() {
                // Wire only contains a WireBuffer as the only field, but we need to extract it to provide a helper.
                types_from_type_recursive(field.the_type(), types);
            }
        }
        // Domain types are native to the backend language, but they should provide Serialize and Deserialize support.
        Type::WirePayload(dom) => match dom {
            WirePayload::Composite(inner) => {
                for field in inner.fields() {
                    types_from_type_recursive(field.the_type(), types);
                }
            }
            WirePayload::String => {}
            WirePayload::Enum(x) => {
                for variant in x.variants() {
                    match variant.kind() {
                        VariantKind::Unit(_) => {}
                        VariantKind::Typed(_, x) => types_from_type_recursive(x, types),
                    }
                }
            }
            WirePayload::Option(inner) => types_from_type_recursive(inner, types),
            WirePayload::Vec(inner) => types_from_type_recursive(inner, types),
            WirePayload::Map(u, v) => {
                types_from_type_recursive(u, types);
                types_from_type_recursive(v, types);
            }
        },
        Type::Array(inner) => types_from_type_recursive(inner.the_type(), types),
        Type::FnPointer(inner) => {
            types_from_type_recursive(inner.signature().rval(), types);
            for param in inner.signature().params() {
                types_from_type_recursive(param.the_type(), types);
            }
        }
        Type::ReadPointer(inner) => types_from_type_recursive(inner, types),
        Type::ReadWritePointer(inner) => types_from_type_recursive(inner, types),
        Type::Primitive(_) => {}
        Type::Enum(x) => {
            for variant in x.variants() {
                match variant.kind() {
                    VariantKind::Unit(_) => {}
                    VariantKind::Typed(_, x) => types_from_type_recursive(x, types),
                }
            }
        }
        Type::Opaque(_) => {}
        // Note, patterns must _NEVER_ add themselves as fallbacks. Instead, each code generator should
        // decide on a case-by-case bases whether it wants to use the type's fallback, or generate an
        // entirely new pattern. The exception to this rule are patterns that can embed arbitrary
        // types; which we need to recursively inspect.
        Type::Pattern(x) => match x {
            TypePattern::AsyncCallback(x) => {
                for field in x.fnpointer().signature().params() {
                    types_from_type_recursive(field.the_type(), types);
                }
            }
            TypePattern::CStrPointer => {}
            TypePattern::NamedCallback(x) => {
                let inner = x.fnpointer();
                types_from_type_recursive(inner.signature().rval(), types);
                for param in inner.signature().params() {
                    types_from_type_recursive(param.the_type(), types);
                }
            }
            TypePattern::Slice(x) => types_from_type_recursive(x.t(), types),
            TypePattern::SliceMut(x) => types_from_type_recursive(x.t(), types),
            TypePattern::Option(x) => types_from_type_recursive(x.t(), types),
            TypePattern::Result(x) => {
                for variant in x.the_enum().variants() {
                    match variant.kind() {
                        VariantKind::Typed(_, t) => types_from_type_recursive(t, types),
                        VariantKind::Unit(_) => {}
                    }
                }
            }
            TypePattern::Vec(x) => types_from_type_recursive(x.t(), types),
            TypePattern::Bool => {}
            TypePattern::CChar => {}
            TypePattern::APIVersion => {}
            TypePattern::Utf8String(_) => {}
        },
        Type::ExternType(_) => { /* Nothing to do */ }
    }
}

/// Extract only types annotated with Wire<T>
#[allow(clippy::redundant_pub_crate)]
pub(crate) fn extract_wire_types_from_functions(functions: &[Function]) -> Vec<Type> {
    let mut types = HashSet::new();

    for function in functions {
        if let Type::Wire(_) = function.signature().rval() {
            types.insert(function.signature().rval().clone());
        }

        for param in function.signature().params() {
            if let Type::Wire(_) = param.the_type() {
                types.insert(param.the_type().clone());
            }
        }
    }

    types.into_iter().collect()
}

/// Extracts annotated namespace strings.
#[allow(clippy::implicit_hasher)]
#[allow(clippy::redundant_pub_crate)]
pub(crate) fn extract_namespaces_from_types(types: &[Type], into: &mut HashSet<String>) {
    for t in types {
        match t {
            Type::Primitive(_) => {}
            Type::Array(_) => {}

            Type::Enum(x) => {
                into.insert(x.meta().module().to_string());
            }
            Type::Opaque(x) => {
                into.insert(x.meta().module().to_string());
            }
            Type::Composite(x) => {
                into.insert(x.meta().module().to_string());
            }
            Type::Wire(inner) => {
                into.insert(inner.meta().module().to_string());
            }
            Type::WirePayload(dom) => match dom {
                WirePayload::Composite(inner) => {
                    into.insert(inner.meta().module().to_string());
                }
                WirePayload::String => {}
                WirePayload::Enum(_) => {}
                WirePayload::Option(_) => {}
                WirePayload::Vec(_) => {}
                WirePayload::Map(_, _) => {}
            },
            Type::FnPointer(_) => {}
            Type::ReadPointer(_) => {}
            Type::ReadWritePointer(_) => {}
            Type::Pattern(x) => match x {
                TypePattern::AsyncCallback(x) => {
                    into.insert(x.meta().module().to_string());
                }
                TypePattern::CStrPointer => {}
                TypePattern::APIVersion => {}
                TypePattern::Slice(x) => {
                    into.insert(x.meta().module().to_string());
                }
                TypePattern::SliceMut(x) => {
                    into.insert(x.meta().module().to_string());
                }
                TypePattern::Option(x) => {
                    into.insert(x.meta().module().to_string());
                }
                TypePattern::Result(x) => {
                    into.insert(x.meta().module().to_string());
                }
                TypePattern::Bool => {}
                TypePattern::CChar => {}
                TypePattern::NamedCallback(_) => {}
                TypePattern::Utf8String(_) => {}
                TypePattern::Vec(x) => {
                    into.insert(x.meta().module().to_string());
                }
            },
            Type::ExternType(_) => { /* Nothing to do */ }
        }
    }
}

/// Checks whether the given type holds an opaque not behind some pointer.
#[must_use]
#[allow(clippy::redundant_pub_crate)]
pub(crate) fn holds_opaque_without_ref(typ: &Type) -> bool {
    match typ {
        Type::Primitive(_) => false,
        Type::Array(x) => holds_opaque_without_ref(x.the_type()),
        Type::Enum(x) => {
            for variant in x.variants() {
                match variant.kind() {
                    VariantKind::Typed(_, t) => {
                        if holds_opaque_without_ref(t) {
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
        Type::Wire(_) => false,
        Type::WirePayload(dom) => match dom {
            WirePayload::Composite(_) => false,
            WirePayload::String => todo!(),
            WirePayload::Enum(_) => todo!(),
            WirePayload::Option(_) => todo!(),
            WirePayload::Vec(_) => todo!(),
            WirePayload::Map(_, _) => todo!(),
        },
        Type::FnPointer(_) => false,
        Type::ReadPointer(_) => false,
        Type::ReadWritePointer(_) => false,
        Type::Pattern(x) => match x {
            TypePattern::CStrPointer => false,
            TypePattern::Utf8String(_) => false,
            TypePattern::APIVersion => false,
            TypePattern::Slice(x) => holds_opaque_without_ref(x.t()),
            TypePattern::SliceMut(x) => holds_opaque_without_ref(x.t()),
            TypePattern::Option(x) => holds_opaque_without_ref(&x.the_enum().to_type()),
            TypePattern::Result(x) => holds_opaque_without_ref(&x.the_enum().to_type()),
            TypePattern::Bool => false,
            TypePattern::CChar => false,
            TypePattern::NamedCallback(_) => false,
            TypePattern::AsyncCallback(_) => false,
            TypePattern::Vec(x) => holds_opaque_without_ref(x.t()),
        },
        Type::ExternType(_) => true /* ? think so ? */,
    }
}

/// Converts, for example, identifiers like `my_id` to `MyId`.
pub struct Prettifier {
    tokens: Vec<String>,
}

impl Prettifier {
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

/// Checks whether the given type does not contained anything user-defined.
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
        Type::Array(x) => is_global_type(x.the_type()),
        Type::Enum(x) => {
            for variant in x.variants() {
                match variant.kind() {
                    VariantKind::Typed(_, t) => {
                        if !is_global_type(t) {
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
        Type::Wire(_) => false,
        Type::WirePayload(dom) => match dom {
            WirePayload::Composite(_) => false,
            WirePayload::Enum(_) => false,
            WirePayload::Option(_) => false,
            WirePayload::String => true,
            WirePayload::Vec(_) => true,
            WirePayload::Map(_, _) => true,
        },
        Type::FnPointer(_) => false,
        Type::ReadPointer(x) => is_global_type(x),
        Type::ReadWritePointer(x) => is_global_type(x),
        Type::Pattern(x) => match x {
            TypePattern::CStrPointer => true,
            TypePattern::APIVersion => false,
            TypePattern::Slice(x) => is_global_type(x.t()),
            TypePattern::SliceMut(x) => is_global_type(x.t()),
            TypePattern::Option(x) => is_global_type(&x.the_enum().to_type()),
            TypePattern::Result(x) => is_global_type(&x.the_enum().to_type()),
            TypePattern::Bool => true,
            TypePattern::CChar => true,
            TypePattern::NamedCallback(_) => false,
            TypePattern::AsyncCallback(_) => false,
            TypePattern::Utf8String(_) => true,
            TypePattern::Vec(x) => is_global_type(x.t()),
        },
        Type::ExternType(_) => true,
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

#[cfg(test)]
mod test {
    use crate::lang::util::Prettifier;

    #[test]
    fn is_pretty() {
        assert_eq!(Prettifier::from_rust_lower("hello_world").to_camel_case(), "HelloWorld");
        assert_eq!(Prettifier::from_rust_lower("single").to_camel_case(), "Single");
    }
}
