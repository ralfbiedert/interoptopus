//! Helpers for backend authors.

use crate::lang::c::{CType, Function};
use std::collections::HashSet;

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

pub(crate) fn types_from_functions(functions: &[Function]) -> Vec<CType> {
    let mut types = HashSet::new();

    for function in functions {
        types_from_type_recursive(function.signature().rval(), &mut types);

        for param in function.signature().params() {
            types_from_type_recursive(param.the_type(), &mut types);
        }
    }

    types.iter().cloned().collect()
}

pub(crate) fn types_from_type_recursive(start: &CType, types: &mut HashSet<CType>) {
    types.insert(start.clone());

    match start {
        CType::Composite(inner) => {
            for field in inner.fields() {
                types_from_type_recursive(&field.the_type(), types);
            }
        }
        CType::FnPointer(inner) => {
            types_from_type_recursive(inner.signature().rval(), types);
            for param in inner.signature().params() {
                types_from_type_recursive(param.the_type(), types);
            }
        }
        CType::ReadPointer(inner) => types_from_type_recursive(inner, types),
        CType::ReadWritePointer(inner) => types_from_type_recursive(inner, types),
        CType::Primitive(_) => {}
        CType::Enum(_) => {}
        CType::Opaque(_) => {}
    }
}
