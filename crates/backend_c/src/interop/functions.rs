use crate::converters::{function_name_to_c_name, to_type_specifier};
use crate::interop::ToNamingStyle;
use crate::interop::docs::write_documentation;
use crate::{DocStyle, Functions, Interop};
use interoptopus::backend::IndentWriter;
use interoptopus::lang::c::{CType, Function};
use interoptopus::{Error, indented};

pub fn write_functions(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    for function in i.inventory.functions() {
        write_function(i, w, function)?;
    }

    Ok(())
}

fn write_function(i: &Interop, w: &mut IndentWriter, function: &Function) -> Result<(), Error> {
    if i.documentation == DocStyle::Inline {
        write_documentation(w, function.meta().documentation())?;
    }

    match i.function_style {
        Functions::Typedefs => write_function_as_typedef_declaration(i, w, function)?,
        Functions::ForwardDeclarations => write_function_declaration(i, w, function, 999)?,
    }

    if i.documentation == DocStyle::Inline {
        w.newline()?;
    }

    Ok(())
}

pub fn write_function_declaration(i: &Interop, w: &mut IndentWriter, function: &Function, max_line: usize) -> Result<(), Error> {
    let attr = &i.function_attribute;
    let rval = to_type_specifier(i, function.signature().rval());
    let name = function_name_to_c_name(function);

    let mut params = Vec::new();

    for p in function.signature().params() {
        match p.the_type() {
            CType::Array(a) => {
                params.push(format!("{} {}[{}]", to_type_specifier(i, a.array_type()), p.name().to_naming_style(&i.function_parameter_naming), a.len(),));
            }
            _ => {
                params.push(format!("{} {}", to_type_specifier(i, p.the_type()), p.name().to_naming_style(&i.function_parameter_naming)));
            }
        }
    }

    // Test print line to see if we need to break it
    let line = format!(r"{}{} {}({});", attr, rval, name, params.join(", "));

    if line.len() <= max_line {
        indented!(w, r"{}{} {}({});", attr, rval, name, params.join(", "))?;
    } else {
        indented!(w, r"{}{} {}(", attr, rval, name)?;
        for p in params {
            indented!(w, [()], r"{}", p)?;
        }
        indented!(w, [()], r");")?;
    }

    Ok(())
}

fn write_function_as_typedef_declaration(i: &Interop, w: &mut IndentWriter, function: &Function) -> Result<(), Error> {
    let rval = to_type_specifier(i, function.signature().rval());
    let name = function_name_to_c_name(function);

    let mut params = Vec::new();

    for p in function.signature().params() {
        match p.the_type() {
            CType::Array(a) => {
                params.push(format!("{} [{}]", to_type_specifier(i, a.array_type()), a.len(),));
            }
            _ => {
                params.push(to_type_specifier(i, p.the_type()).to_string());
            }
        }
    }
    indented!(w, r"typedef {} (*{})({});", rval, name, params.join(", "))?;

    Ok(())
}
