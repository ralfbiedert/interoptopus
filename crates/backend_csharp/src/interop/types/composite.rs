use crate::Interop;
use crate::converter::{field_name, field_to_managed, field_to_type, field_to_type_declaration_unmanaged, field_to_unmanaged, is_blittable};
use crate::interop::docs::write_documentation;
use crate::utils::{MoveSemantics, write_common_marshaller};
use interoptopus::backend::{IndentWriter, WriteFor};
use interoptopus::lang::{Composite, Field, Layout, Type, Visibility};
use interoptopus::{Error, indented};

pub fn write_type_definition_composite(i: &Interop, w: &mut IndentWriter, the_type: &Composite) -> Result<(), Error> {
    i.debug(w, "write_type_definition_composite")?;
    write_documentation(w, the_type.meta().docs())?;
    write_type_definition_composite_body(i, w, the_type, WriteFor::Code)?;
    write_type_definition_composite_marshaller(i, w, the_type)
}

pub fn write_type_definition_composite_marshaller(i: &Interop, w: &mut IndentWriter, the_type: &Composite) -> Result<(), Error> {
    i.debug(w, "write_type_definition_composite_marshaller")?;
    let name = the_type.rust_name();
    let self_kind = if is_blittable(&the_type.to_type()) { "struct" } else { "class" };
    let into = if is_blittable(&the_type.to_type()) { "To" } else { "Into" };
    let move_semantics = if is_blittable(&the_type.to_type()) {
        MoveSemantics::Copy
    } else {
        MoveSemantics::Move
    };

    indented!(w, r"[NativeMarshalling(typeof(MarshallerMeta))]")?;
    indented!(w, r"public partial {self_kind} {}", name)?;
    indented!(w, r"{{")?;

    indented!(w, [()], r"public {name}() {{ }}")?;
    w.newline()?;
    indented!(w, [()], r"public unsafe Unmanaged {into}Unmanaged()")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"var _unmanaged = new Unmanaged();")?;
    for field in the_type.fields() {
        let name = field.name();

        if let Type::Array(x) = field.the_type() {
            let array_type = field_to_type(x.the_type());
            indented!(w, [()()], "{{")?;
            indented!(w, [()()()], r#"if ({} == null) {{ throw new InvalidOperationException("Array '{}' must not be null"); }}"#, name, name)?;
            indented!(w, [()()()], r#"if ({}.Length != {}) {{ throw new InvalidOperationException("Array size mismatch for '{}'"); }}"#, name, x.len(), name)?;
            indented!(w, [()()()], "var src = new ReadOnlySpan<{}>({}, 0, {});", array_type, name, x.len())?;
            indented!(w, [()()()], "var dst = new Span<{array_type}>(_unmanaged.{name}, {});", x.len())?;
            indented!(w, [()()()], "src.CopyTo(dst);")?;
            indented!(w, [()()], "}}")?;
        } else {
            let to_unmanaged = field_to_unmanaged(field);
            indented!(w, [()()], "_unmanaged.{name} = {to_unmanaged};")?;
        }
    }
    indented!(w, [()()], r"return _unmanaged;")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;

    w.indent();
    write_type_definition_composite_layout_annotation(w, the_type)?;
    w.unindent();

    indented!(w, [()], r"public unsafe struct Unmanaged")?;
    indented!(w, [()], r"{{")?;
    for field in the_type.fields() {
        let field_decl = field_to_type_declaration_unmanaged(field);
        indented!(w, [()()], r"public {field_decl};")?;
    }
    w.newline()?;
    indented!(w, [()()], r"public unsafe {name} {into}Managed()")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"var _managed = new {name}();")?;
    for field in the_type.fields() {
        let name = field.name();

        if let Type::Array(x) = field.the_type() {
            let array_type = field_to_type(x.the_type());
            indented!(w, [()()()], "fixed({}* _fixed = {})", array_type, name)?;
            indented!(w, [()()()], "{{")?;
            indented!(w, [()()()()], "_managed.{} = new {}[{}];", name, array_type, x.len())?;
            indented!(w, [()()()()], "var src = new ReadOnlySpan<{array_type}>(_fixed, {});", x.len())?;
            indented!(w, [()()()()], "var dst = new Span<{}>(_managed.{}, 0, {});", array_type, name, x.len())?;
            indented!(w, [()()()()], "src.CopyTo(dst);")?;
            indented!(w, [()()()], "}}")?;
        } else {
            let to_unmanaged = field_to_managed(field);
            indented!(w, [()()()], "_managed.{name} = {to_unmanaged};")?;
        }
    }
    indented!(w, [()()()], r"return _managed;")?;
    indented!(w, [()()], r"}}")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;

    indented!(w, [()], r"[CustomMarshaller(typeof({}), MarshalMode.Default, typeof(Marshaller))]", name)?;
    indented!(w, [()], r"private struct MarshallerMeta {{ }}")?;

    write_common_marshaller(i, w, name, move_semantics)?;

    indented!(w, r"}}")?;

    Ok(())
}

#[allow(clippy::unused_self)]
pub fn write_type_definition_composite_layout_annotation(w: &mut IndentWriter, the_type: &Composite) -> Result<(), Error> {
    match the_type.repr().layout() {
        Layout::C | Layout::Transparent | Layout::Opaque => indented!(w, r"[StructLayout(LayoutKind.Sequential)]"),
        Layout::Packed => indented!(w, r"[StructLayout(LayoutKind.Sequential, Pack = 1)]"),
        Layout::Primitive(_) => panic!("Primitive layout not supported for structs."),
    }
}

pub fn write_type_definition_composite_body(i: &Interop, w: &mut IndentWriter, the_type: &Composite, write_for: WriteFor) -> Result<(), Error> {
    let visibility = i.visibility_types.to_access_modifier();
    let self_kind = if is_blittable(&the_type.to_type()) { "struct" } else { "class" };
    let rust_name = the_type.rust_name();

    indented!(w, r"{visibility} partial {self_kind} {rust_name}")?;
    indented!(w, r"{{")?;
    w.indent();

    for field in the_type.fields() {
        if write_for == WriteFor::Code {
            write_documentation(w, field.docs())?;
        }

        write_type_definition_composite_body_field(i, w, field, the_type)?;
    }

    w.unindent();
    indented!(w, r"}}")?;
    w.newline()?;

    Ok(())
}

#[allow(clippy::single_match_else)]
pub fn write_type_definition_composite_body_field(i: &Interop, w: &mut IndentWriter, field: &Field, _: &Composite) -> Result<(), Error> {
    let field_name = field_name(field, i.rename_symbols);
    let visibility = match field.visibility() {
        Visibility::Public => "public ",
        Visibility::Private => "",
    };

    let type_name = field_to_type(field.the_type());
    indented!(w, r"{}{} {};", visibility, type_name, field_name)?;

    Ok(())
}
