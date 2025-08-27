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
use crate::interop::wires::{write_type_definition_wired_enum, write_type_definitions_domain_wired, write_type_definitions_wired};
use interoptopus::Error;
use interoptopus::backend::IndentWriter;
use interoptopus::lang::{DomainType, Type};
use interoptopus::pattern::TypePattern;

pub fn write_type_definitions(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    for the_type in i.inventory.c_types() {
        write_type_definition(i, w, the_type)?;
    }

    Ok(())
}

pub fn write_type_definition(i: &Interop, w: &mut IndentWriter, the_type: &Type) -> Result<(), Error> {
    // eprintln!("🤡 Will {}write_type_definition {}", if i.should_emit_by_type(the_type) { "" } else { "NOT " }, the_type.name_within_lib());
    if !i.should_emit_by_type(the_type) {
        return Ok(());
    }

    // eprintln!("🤡 write_type_definition {the_type:?}");

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
        Type::Wired(_) => panic!("Wired is emitted without counterpart"),
        Type::Domain(dom) => match dom {
            DomainType::Composite(wired) => {
                write_type_definitions_domain_wired(i, w, wired)?;
                w.newline()?;
                let wire_counterpart = i.wired_counterpart(&Type::Composite(wired.clone()));
                eprintln!("✅ Wire counterpart for {} is {:?} ✅", wired.rust_name(), wire_counterpart);
                if let Some(Type::Wired(wire)) = wire_counterpart {
                    write_type_definitions_wired(i, w, &wire)?;
                    w.newline()?;
                }
            }
            DomainType::String => {} // nothing todo!(),
            DomainType::Enum(e) => {
                write_type_definition_wired_enum(i, w, e)?;
                w.newline()?;
                if let Some(Type::Wired(wire)) = i.wired_counterpart(&Type::Enum(e.clone())) {
                    write_type_definitions_wired(i, w, &wire)?;
                    w.newline()?;
                }
            }
            DomainType::Option(_) => {} // nothing todo!(),
            DomainType::Vec(_) => {}    // nothing todo!(),
            DomainType::Map(_, _) => {} // nothing todo!(),
        },
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
