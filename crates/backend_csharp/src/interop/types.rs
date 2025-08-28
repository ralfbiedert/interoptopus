pub mod bools;
pub mod composite;
pub mod enums;
pub mod fnptrs;

use crate::Interop;
use crate::interop::patterns::asynk::write_pattern_async_trampoline;
use crate::interop::patterns::callbacks::write_type_definition_named_callback;
use crate::interop::patterns::slices::{SliceKind, write_pattern_slice};
use crate::interop::patterns::vec::write_pattern_vec;
use crate::interop::types::bools::write_type_definition_ffibool;
use crate::interop::types::composite::write_type_definition_composite;
use crate::interop::types::enums::write_type_definition_enum;
use crate::interop::types::fnptrs::write_type_definition_fn_pointer;
use interoptopus::Error;
use interoptopus::backend::IndentWriter;
use interoptopus::lang::Type;
use interoptopus::pattern::TypePattern;

pub fn write_type_definitions(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    for the_type in i.inventory.c_types() {
        write_type_definition(i, w, the_type)?;
    }

    Ok(())
}

pub fn write_type_definition(i: &Interop, w: &mut IndentWriter, the_type: &Type) -> Result<(), Error> {
    if !i.should_emit_by_type(the_type) {
        return Ok(());
    }

    match the_type {
        Type::Primitive(_) => {}
        Type::Array(_) => {}
        Type::Enum(e) => {
            write_type_definition_enum(i, w, e)?;
            w.newline()?;
        }
        Type::Opaque(_) => {}
        Type::Composite(c) => {
            write_type_definition_composite(i, w, c)?;
            w.newline()?;
        }
        Type::FnPointer(f) => {
            write_type_definition_fn_pointer(i, w, f)?;
            w.newline()?;
        }
        Type::ReadPointer(_) => {}
        Type::ReadWritePointer(_) => {}
        Type::Pattern(x) => match x {
            TypePattern::CStrPointer => {}
            TypePattern::Slice(x) => {
                write_pattern_slice(i, w, x, SliceKind::Slice)?;
                w.newline()?;
            }
            TypePattern::SliceMut(x) => {
                write_pattern_slice(i, w, x, SliceKind::SliceMut)?;
                w.newline()?;
            }
            TypePattern::Option(x) => {
                write_type_definition_enum(i, w, x.the_enum())?;
                w.newline()?;
                // write_pattern_option(i, w, x)?;
                // w.newline()?;
            }
            TypePattern::Result(x) => {
                write_type_definition_enum(i, w, x.the_enum())?;
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
            TypePattern::Vec(x) => {
                write_pattern_vec(i, w, x)?;
                w.newline()?;
            }
            TypePattern::Utf8String(_) => {}
            TypePattern::CChar => {}
            TypePattern::APIVersion => {}
            TypePattern::AsyncCallback(x) => {
                write_pattern_async_trampoline(i, w, x)?;
                w.newline()?;
            }
        },
    }
    Ok(())
}
