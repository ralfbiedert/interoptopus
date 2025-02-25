pub mod abi_guard;
pub mod options;
pub mod results;
pub mod services;
pub mod slices;
mod slices_legacy;

use crate::converter::{get_slice_type, to_typespecifier_in_param};
use crate::interop::patterns::services::write_pattern_service;
use crate::Interop;
use interoptopus::lang::c::{CType, Parameter};
use interoptopus::patterns::{LibraryPattern, TypePattern};
use interoptopus::writer::IndentWriter;
use interoptopus::Error;

pub fn write_patterns(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    for pattern in i.inventory.patterns() {
        match pattern {
            LibraryPattern::Service(cls) => {
                if i.should_emit_by_meta(cls.the_type().meta()) {
                    write_pattern_service(i, w, cls)?;
                }
            }
            _ => panic!("Pattern not explicitly handled"),
        }
    }

    Ok(())
}

#[must_use]
pub fn pattern_to_native_in_signature(i: &Interop, param: &Parameter) -> String {
    let slice_type_name = |mutable: bool, element_type: &CType| -> String {
        if mutable {
            format!("Span<{}>", to_typespecifier_in_param(element_type))
        } else {
            format!("ReadOnlySpan<{}>", to_typespecifier_in_param(element_type))
        }
    };
    match param.the_type() {
        x @ CType::Pattern(p) => match p {
            TypePattern::Slice(p) if !i.should_emit_marshaller(&get_slice_type(p)) => {
                let element_type = p.try_deref_pointer().expect("Must be pointer");
                slice_type_name(false, &element_type)
            }
            TypePattern::SliceMut(p) if !i.should_emit_marshaller(&get_slice_type(p)) => {
                let element_type = p.try_deref_pointer().expect("Must be pointer");
                slice_type_name(true, &element_type)
            }
            TypePattern::NamedCallback(_) => {
                format!("{}Delegate", to_typespecifier_in_param(x))
            }

            _ => to_typespecifier_in_param(param.the_type()),
        },
        CType::ReadPointer(x) | CType::ReadWritePointer(x) => match &**x {
            CType::Pattern(x) => match x {
                TypePattern::Slice(p) if !i.should_emit_marshaller(&get_slice_type(p)) => {
                    let element_type = p.try_deref_pointer().expect("Must be pointer");
                    slice_type_name(false, &element_type)
                }
                TypePattern::SliceMut(p) if !i.should_emit_marshaller(&get_slice_type(p)) => {
                    let element_type = p.try_deref_pointer().expect("Must be pointer");
                    slice_type_name(true, &element_type)
                }
                _ => to_typespecifier_in_param(param.the_type()),
            },
            _ => to_typespecifier_in_param(param.the_type()),
        },

        x => to_typespecifier_in_param(x),
    }
}
