//! Helpers for backend authors.

use crate::lang::c::{CType, Function};
use crate::patterns::TypePattern;
use std::collections::HashSet;
use std::iter::FromIterator;

/// Converts an internal name like `fn() -> X` to a safe name like `fn_rval_x`
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
/// # use interoptopus::lang::c::{Function, Documentation, FunctionSignature};
/// # use interoptopus::util::longest_common_prefix;
///
/// let functions = [
///     Function::new("my_lib_f".to_string(), FunctionSignature::default(), Documentation::default()),
///     Function::new("my_lib_g".to_string(), FunctionSignature::default(), Documentation::default()),
///     Function::new("my_lib_h".to_string(), FunctionSignature::default(), Documentation::default()),
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

pub(crate) fn ctypes_from_functions(functions: &[Function]) -> Vec<CType> {
    let mut types = HashSet::new();

    for function in functions {
        ctypes_from_type_recursive(function.signature().rval(), &mut types);

        for param in function.signature().params() {
            ctypes_from_type_recursive(param.the_type(), &mut types);
        }
    }

    types.iter().cloned().collect()
}

pub(crate) fn ctypes_from_type_recursive(start: &CType, types: &mut HashSet<CType>) {
    types.insert(start.clone());

    match start {
        CType::Composite(inner) => {
            for field in inner.fields() {
                ctypes_from_type_recursive(&field.the_type(), types);
            }
        }
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
            TypePattern::SuccessEnum(_) => {}
            TypePattern::FFISlice(x) => {
                for field in x.fields() {
                    ctypes_from_type_recursive(field.the_type(), types);
                }
            }
        },
    }
}
