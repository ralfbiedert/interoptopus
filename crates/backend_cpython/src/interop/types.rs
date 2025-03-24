use crate::Interop;
use crate::converter::{to_ctypes_name, to_type_hint_in, to_type_hint_out};
use crate::interop::patterns::write_slice;
use interoptopus::backend::sort_types_by_dependencies;
use interoptopus::backend::{IndentWriter, WriteFor};
use interoptopus::lang::{Composite, Enum, Layout, Type, VariantKind};
use interoptopus::pattern::TypePattern;
use interoptopus::{Error, indented};

pub fn write_types(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    let all_types = i.inventory.ctypes().to_vec();
    let sorted_types = sort_types_by_dependencies(all_types);

    for t in &sorted_types {
        match t {
            Type::Composite(c) => write_struct(i, w, c, WriteFor::Code)?,
            Type::Enum(e) => write_enum(i, w, e, WriteFor::Code)?,
            Type::Pattern(p) => match p {
                TypePattern::Slice(c) => write_slice(i, w, c, false)?,
                TypePattern::SliceMut(c) => write_slice(i, w, c, true)?,
                TypePattern::Result(c) => write_enum(i, w, c.the_enum(), WriteFor::Code)?,
                TypePattern::Utf8String(c) => write_struct(i, w, c, WriteFor::Code)?,
                TypePattern::Option(c) => write_enum(i, w, c.the_enum(), WriteFor::Code)?,
                _ => continue,
            },
            _ => continue,
        }

        w.newline()?;
        w.newline()?;
    }

    Ok(())
}

pub fn write_struct(_i: &Interop, w: &mut IndentWriter, c: &Composite, write_for: WriteFor) -> Result<(), Error> {
    let documentation = c.meta().documentation().lines().join("\n");

    indented!(w, r"class {}(ctypes.Structure):", c.rust_name())?;
    if !documentation.is_empty() && write_for == WriteFor::Code {
        indented!(w, [()], r#""""{}""""#, documentation)?;
    }

    if c.repr().layout() == Layout::Packed {
        indented!(w, [()], r"_pack_ = 1")?;
    }

    let alignment = c.repr().alignment();
    if let Some(align) = alignment {
        indented!(w, [()], r"_align_ = {}", align)?;
    }

    w.newline()?;
    if write_for == WriteFor::Code {
        indented!(w, [()], r"# These fields represent the underlying C data layout")?;
    }
    indented!(w, [()], r"_fields_ = [")?;
    for f in c.fields() {
        let type_name = to_ctypes_name(f.the_type(), true);
        indented!(w, [()()], r#"("{}", {}),"#, f.name(), type_name)?;
    }
    indented!(w, [()], r"]")?;

    // Ctor
    let extra_args = c
        .fields()
        .iter()
        .map(|x| {
            let type_hint_in = to_type_hint_in(x.the_type(), false);

            format!("{}{} = None", x.name(), type_hint_in)
        })
        .collect::<Vec<_>>()
        .join(", ");

    if !c.fields().is_empty() {
        w.newline()?;
        indented!(w, [()], r"def __init__(self, {}):", extra_args)?;

        if write_for == WriteFor::Code {
            for field in c.fields() {
                indented!(w, [()()], r"if {} is not None:", field.name())?;
                indented!(w, [()()()], r"self.{} = {}", field.name(), field.name())?;
            }
        } else {
            indented!(w, [()()], r"...")?;
        }
    }

    if write_for == WriteFor::Docs {
        return Ok(());
    }

    // Fields
    for f in c.fields() {
        let documentation = f.documentation().lines().join("\n");

        w.newline()?;

        let hint_in = to_type_hint_in(f.the_type(), false);
        let hint_out = to_type_hint_out(f.the_type());

        indented!(w, [()], r"@property")?;
        indented!(w, [()], r"def {}(self){}:", f.name(), hint_out)?;

        if !documentation.is_empty() {
            indented!(w, [()()], r#""""{}""""#, documentation)?;
        }

        match f.the_type() {
            Type::Pattern(_) => indented!(w, [()()], r#"return ctypes.Structure.__get__(self, "{}")"#, f.name())?,
            _ => indented!(w, [()()], r#"return ctypes.Structure.__get__(self, "{}")"#, f.name())?,
        }

        w.newline()?;

        indented!(w, [()], r"@{}.setter", f.name())?;
        indented!(w, [()], r"def {}(self, value{}):", f.name(), hint_in)?;
        if !documentation.is_empty() {
            indented!(w, [()()], r#""""{}""""#, documentation)?;
        }
        indented!(w, [()()], r#"return ctypes.Structure.__set__(self, "{}", value)"#, f.name())?;
    }

    Ok(())
}

pub fn write_enum(_i: &Interop, w: &mut IndentWriter, e: &Enum, write_for: WriteFor) -> Result<(), Error> {
    let documentation = e.meta().documentation().lines().join("\n");

    indented!(w, r"class {}:", e.rust_name())?;
    if !documentation.is_empty() && write_for == WriteFor::Code {
        indented!(w, [()], r#""""{}""""#, documentation)?;
    }

    for v in e.variants() {
        if write_for == WriteFor::Code {
            for line in v.documentation().lines() {
                indented!(w, [()], r"# {}", line)?;
            }
        }

        match v.kind() {
            VariantKind::Unit(x) => indented!(w, [()], r"{} = {}", v.name(), x)?,
            VariantKind::Typed(_, _) => indented!(w, r"# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN")?,
        }
    }

    Ok(())
}
