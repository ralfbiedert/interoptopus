use crate::converters::{
    composite_to_typename, enum_to_typename, enum_variant_to_name, fnpointer_to_typename, named_callback_to_typename, opaque_to_typename, to_type_specifier,
};
use crate::interop::ToNamingStyle;
use crate::interop::docs::write_documentation;
use crate::{DocStyle, Indentation, Interop};
use interoptopus::backend::util::sort_types_by_dependencies;
use interoptopus::backend::writer::IndentWriter;
use interoptopus::lang::c::{CType, CompositeType, EnumType, Field, FnPointerType, OpaqueType, Variant};
use interoptopus::patterns::TypePattern;
use interoptopus::patterns::callback::NamedCallback;
use interoptopus::{Error, indented};

pub fn write_type_definitions(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    let mut known_function_pointers = vec![];

    for the_type in &sort_types_by_dependencies(i.inventory.ctypes().to_vec()) {
        write_type_definition(i, w, the_type, &mut known_function_pointers)?;
    }

    Ok(())
}

pub fn write_type_definition(i: &Interop, w: &mut IndentWriter, the_type: &CType, known_function_pointers: &mut Vec<String>) -> Result<(), Error> {
    match the_type {
        CType::Primitive(_) => {}
        CType::Array(_) => {}
        CType::Enum(e) => {
            write_type_definition_enum(i, w, e)?;
            w.newline()?;
        }
        CType::Opaque(o) => {
            write_type_definition_opaque(i, w, o)?;
        }
        CType::Composite(c) => {
            write_type_definition_composite(i, w, c)?;
            w.newline()?;
        }
        CType::FnPointer(f) => {
            write_type_definition_fn_pointer(i, w, f, known_function_pointers)?;
            w.newline()?;
        }
        CType::ReadPointer(_) => {}
        CType::ReadWritePointer(_) => {}
        CType::Pattern(p) => match p {
            TypePattern::CStrPointer => {}
            TypePattern::NamedCallback(e) => {
                write_type_definition_named_callback(i, w, e)?;
                w.newline()?;
            }
            TypePattern::FFIErrorEnum(e) => {
                write_type_definition_enum(i, w, e.the_enum())?;
                w.newline()?;
            }
            TypePattern::Slice(x) => {
                write_type_definition_composite(i, w, x.composite_type())?;
                w.newline()?;
            }
            TypePattern::SliceMut(x) => {
                write_type_definition_composite(i, w, x.composite_type())?;
                w.newline()?;
            }
            TypePattern::Option(x) => {
                write_type_definition_composite(i, w, x)?;
                w.newline()?;
            }
            TypePattern::Utf8String(x) => {
                write_type_definition_composite(i, w, x)?;
                w.newline()?;
            }
            TypePattern::Result(x) => {
                write_type_definition_composite(i, w, x.composite())?;
                w.newline()?;
            }
            TypePattern::AsyncCallback(x) => {
                write_type_definition_fn_pointer(i, w, x.fnpointer(), known_function_pointers)?;
                w.newline()?;
            }
            TypePattern::Bool => {}
            TypePattern::CChar => {}
            TypePattern::APIVersion => {}
            _ => panic!("Pattern not explicitly handled"),
        },
    }
    Ok(())
}

fn write_type_definition_fn_pointer(i: &Interop, w: &mut IndentWriter, the_type: &FnPointerType, known_function_pointers: &mut Vec<String>) -> Result<(), Error> {
    write_type_definition_fn_pointer_body(i, w, the_type, known_function_pointers)
}

fn write_type_definition_fn_pointer_body(i: &Interop, w: &mut IndentWriter, the_type: &FnPointerType, known_function_pointers: &mut Vec<String>) -> Result<(), Error> {
    let rval = to_type_specifier(i, the_type.signature().rval());
    let name = fnpointer_to_typename(i, the_type);

    let mut params = Vec::new();
    for (x, param) in the_type.signature().params().iter().enumerate() {
        params.push(format!("{} x{}", to_type_specifier(i, param.the_type()), x));
    }

    let fn_pointer = format!("typedef {} (*{})({});", rval, name, params.join(", "));

    if !known_function_pointers.contains(&fn_pointer) {
        indented!(w, "{}", fn_pointer)?;
        known_function_pointers.push(fn_pointer);
    }

    Ok(())
}

fn write_type_definition_named_callback(i: &Interop, w: &mut IndentWriter, the_type: &NamedCallback) -> Result<(), Error> {
    write_type_definition_named_callback_body(i, w, the_type)
}

fn write_type_definition_named_callback_body(i: &Interop, w: &mut IndentWriter, the_type: &NamedCallback) -> Result<(), Error> {
    let rval = to_type_specifier(i, the_type.fnpointer().signature().rval());
    let name = named_callback_to_typename(i, the_type);

    let mut params = Vec::new();
    for param in the_type.fnpointer().signature().params() {
        params.push(format!("{} {}", to_type_specifier(i, param.the_type()), param.name().to_naming_style(&i.function_parameter_naming)));
    }

    indented!(w, "{}", format!("typedef {} (*{})({});", rval, name, params.join(", ")))?;

    Ok(())
}

fn write_type_definition_enum(i: &Interop, w: &mut IndentWriter, the_type: &EnumType) -> Result<(), Error> {
    let name = enum_to_typename(i, the_type);

    if i.documentation == DocStyle::Inline {
        write_documentation(w, the_type.meta().documentation())?;
    }

    write_braced_declaration_opening(i, w, &format!("typedef enum {name}"))?;

    for variant in the_type.variants() {
        write_type_definition_enum_variant(i, w, variant, the_type)?;
    }

    write_braced_declaration_closing(i, w, name.as_str())
}

fn write_type_definition_enum_variant(i: &Interop, w: &mut IndentWriter, variant: &Variant, the_enum: &EnumType) -> Result<(), Error> {
    let variant_name = enum_variant_to_name(i, the_enum, variant);
    let variant_value = variant.value();

    if i.documentation == DocStyle::Inline {
        write_documentation(w, variant.documentation())?;
    }

    indented!(w, r"{} = {},", variant_name, variant_value)
}

fn write_type_definition_opaque(i: &Interop, w: &mut IndentWriter, the_type: &OpaqueType) -> Result<(), Error> {
    if i.documentation == DocStyle::Inline {
        write_documentation(w, the_type.meta().documentation())?;
    }

    write_type_definition_opaque_body(i, w, the_type)?;

    if i.documentation == DocStyle::Inline {
        w.newline()?;
    }

    Ok(())
}

fn write_type_definition_opaque_body(i: &Interop, w: &mut IndentWriter, the_type: &OpaqueType) -> Result<(), Error> {
    let name = opaque_to_typename(i, the_type);
    indented!(w, r"typedef struct {} {};", name, name)
}

fn write_type_definition_composite(i: &Interop, w: &mut IndentWriter, the_type: &CompositeType) -> Result<(), Error> {
    if i.documentation == DocStyle::Inline {
        write_documentation(w, the_type.meta().documentation())?;
    }

    let name = composite_to_typename(i, the_type);

    if the_type.is_empty() {
        // C doesn't allow us writing empty structs.
        indented!(w, r"typedef struct {} {};", name, name)?;
        Ok(())
    } else {
        write_type_definition_composite_body(i, w, the_type)
    }
}

fn write_type_definition_composite_body(i: &Interop, w: &mut IndentWriter, the_type: &CompositeType) -> Result<(), Error> {
    let name = composite_to_typename(i, the_type);

    let alignment = the_type.repr().alignment();
    if let Some(align) = alignment {
        indented!(w, "#pragma pack(push, {})", align)?;
    }

    write_braced_declaration_opening(i, w, format!(r"typedef struct {name}").as_str())?;

    for field in the_type.fields() {
        write_type_definition_composite_body_field(i, w, field, the_type)?;
    }

    write_braced_declaration_closing(i, w, name.as_str())?;

    if alignment.is_some() {
        indented!(w, "#pragma pack(pop)")?;
    }
    Ok(())
}

fn write_type_definition_composite_body_field(i: &Interop, w: &mut IndentWriter, field: &Field, _the_type: &CompositeType) -> Result<(), Error> {
    if i.documentation == DocStyle::Inline {
        write_documentation(w, field.documentation())?;
    }

    let field_name = field.name();

    if let CType::Array(x) = field.the_type() {
        let type_name = to_type_specifier(i, x.array_type());
        indented!(w, r"{} {}[{}];", type_name, field_name, x.len())
    } else {
        let field_name = field.name();
        let type_name = to_type_specifier(i, field.the_type());
        indented!(w, r"{} {};", type_name, field_name)
    }
}

fn write_braced_declaration_opening(i: &Interop, w: &mut IndentWriter, definition: &str) -> Result<(), Error> {
    match i.indentation {
        Indentation::Allman => {
            indented!(w, "{}", definition)?;
            indented!(w, "{{")?;
            w.indent();
        }
        Indentation::KAndR => {
            indented!(w, "{} {{", definition)?;
            w.indent();
        }
        Indentation::GNU => {
            indented!(w, "{}", definition)?;
            indented!(w, "  {{")?;
            w.indent();
        }
        Indentation::Whitesmiths => {
            indented!(w, "{}", definition)?;
            indented!(w, [()], "{{")?;
            w.indent();
        }
    }

    Ok(())
}

fn write_braced_declaration_closing(i: &Interop, w: &mut IndentWriter, name: &str) -> Result<(), Error> {
    match i.indentation {
        Indentation::Allman | Indentation::KAndR => {
            w.unindent();
            indented!(w, "}} {};", name)?;
        }
        Indentation::GNU => {
            w.unindent();
            indented!(w, "  }} {};", name)?;
        }
        Indentation::Whitesmiths => {
            w.unindent();
            indented!(w, [()], "}} {};", name)?;
        }
    }

    Ok(())
}
