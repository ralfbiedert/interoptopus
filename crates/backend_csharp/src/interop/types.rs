pub mod bools;
pub mod callbacks;
pub mod composite;
pub mod enums;
pub mod fnptrs;

use crate::interop::patterns::options::write_pattern_option;
use crate::interop::patterns::results::write_pattern_result;
use crate::interop::patterns::slices::{write_pattern_slice, write_pattern_slice_mut};
use crate::interop::types::bools::write_type_definition_ffibool;
use crate::interop::types::callbacks::write_type_definition_named_callback;
use crate::interop::types::composite::write_type_definition_composite;
use crate::interop::types::enums::write_type_definition_enum;
use crate::interop::types::fnptrs::write_type_definition_fn_pointer;
use crate::Interop;
use interoptopus::lang::c::CType;
use interoptopus::patterns::TypePattern;
use interoptopus::writer::{IndentWriter, WriteFor};
use interoptopus::Error;

pub fn write_type_definitions(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    for the_type in i.inventory.ctypes() {
        write_type_definition(i, w, the_type)?;
    }

    Ok(())
}

pub fn write_type_definition(i: &Interop, w: &mut IndentWriter, the_type: &CType) -> Result<(), Error> {
    if !i.should_emit_by_type(the_type) {
        return Ok(());
    }

    match the_type {
        CType::Primitive(_) => {}
        CType::Array(_) => {}
        CType::Enum(e) => {
            write_type_definition_enum(i, w, e, WriteFor::Code)?;
            w.newline()?;
        }
        CType::Opaque(_) => {}
        CType::Composite(c) => {
            write_type_definition_composite(i, w, c)?;
            w.newline()?;
        }
        CType::FnPointer(f) => {
            write_type_definition_fn_pointer(i, w, f)?;
            w.newline()?;
        }
        CType::ReadPointer(_) => {}
        CType::ReadWritePointer(_) => {}
        CType::Pattern(x) => match x {
            TypePattern::CStrPointer => {}
            TypePattern::FFIErrorEnum(e) => {
                write_type_definition_enum(i, w, e.the_enum(), WriteFor::Code)?;
                w.newline()?;
            }
            TypePattern::Slice(x) => {
                write_pattern_slice(i, w, x)?;
                w.newline()?;
            }
            TypePattern::SliceMut(x) => {
                write_pattern_slice_mut(i, w, x)?;
                w.newline()?;
            }
            TypePattern::Option(x) => {
                write_type_definition_composite(i, w, x)?;
                w.newline()?;
                write_pattern_option(i, w, x)?;
                w.newline()?;
            }
            TypePattern::Result(x) => {
                write_type_definition_composite(i, w, x)?;
                w.newline()?;
                write_pattern_result(i, w, x)?;
                w.newline()?;
            }
            TypePattern::NamedCallback(x) => {
                // Handle this better way
                write_type_definition_named_callback(i, w, x)?;
                w.newline()?;
            }
            TypePattern::Bool => {
                write_type_definition_ffibool(i, w)?;
                w.newline()?;
            }

            TypePattern::CChar => {}
            TypePattern::APIVersion => {}
            TypePattern::AsyncCallback(_) => {}
            _ => panic!("Pattern not explicitly handled"),
        },
    }
    Ok(())
}
