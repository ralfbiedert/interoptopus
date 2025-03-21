use crate::converter::{field_name_to_csharp_name, is_blittable, to_typespecifier_in_field};
use crate::interop::docs::write_documentation;
use crate::Interop;
use interoptopus::backend::{IndentWriter, WriteFor};
use interoptopus::lang::{Composite, Field, Layout, Primitive, Type, Visibility};
use interoptopus::pattern::TypePattern;
use interoptopus::{indented, Error};

pub fn write_type_definition_composite(i: &Interop, w: &mut IndentWriter, the_type: &Composite) -> Result<(), Error> {
    i.debug(w, "write_type_definition_composite")?;
    write_documentation(w, the_type.meta().documentation())?;
    write_type_definition_composite_body(i, w, the_type, WriteFor::Code)?;
    write_type_definition_composite_marshaller(i, w, the_type)
}

pub fn write_type_definition_composite_marshaller(i: &Interop, w: &mut IndentWriter, the_type: &Composite) -> Result<(), Error> {
    i.debug(w, "write_type_definition_composite_marshaller")?;
    let name = the_type.rust_name();

    indented!(w, r"[NativeMarshalling(typeof(MarshallerMeta))]")?;
    indented!(w, r"public partial struct {}", name)?;
    indented!(w, r"{{")?;

    indented!(w, [()], r"public {name}({name} other)")?;
    indented!(w, [()], r"{{")?;
    for field in the_type.fields() {
        indented!(w, [()()], r"{} = other.{};", field.name(), field.name())?;
    }
    indented!(w, [()], r"}}")?;
    w.newline()?;
    indented!(w, [()], r"public Unmanaged ToUnmanaged()")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"var marshaller = new Marshaller(this);")?;
    indented!(w, [()()], r"try {{ return marshaller.ToUnmanaged(); }}")?;
    indented!(w, [()()], r"finally {{ marshaller.Free(); }}")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;

    w.indent();
    write_type_definition_composite_layout_annotation(w, the_type)?;
    w.unindent();

    indented!(w, [()], r"public unsafe struct Unmanaged")?;
    indented!(w, [()], r"{{")?;
    for field in the_type.fields() {
        w.indent();
        w.indent();
        write_type_definition_composite_unmanaged_body_field(i, w, field, the_type)?;
        w.unindent();
        w.unindent();
    }
    w.newline()?;
    indented!(w, [()()], r"public {name} ToManaged()")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"var marshaller = new Marshaller(this);")?;
    indented!(w, [()()()], r"try {{ return marshaller.ToManaged(); }}")?;
    indented!(w, [()()()], r"finally {{ marshaller.Free(); }}")?;
    indented!(w, [()()], r"}}")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;

    indented!(w, [()], r"[CustomMarshaller(typeof({}), MarshalMode.Default, typeof(Marshaller))]", name)?;
    indented!(w, [()], r"private struct MarshallerMeta {{ }}")?;
    w.newline()?;

    indented!(w, [()], r"public ref struct Marshaller")?;
    indented!(w, [()], r"{{")?;
    w.indent();
    indented!(w, [()], r"private {} _managed; // Used when converting managed -> unmanaged", name)?;
    indented!(w, [()], r"private Unmanaged _unmanaged; // Used when converting unmanaged -> managed")?;
    w.newline()?;
    indented!(w, [()], r"public Marshaller({} managed) {{ _managed = managed; }}", name)?;
    indented!(w, [()], r"public Marshaller(Unmanaged unmanaged) {{ _unmanaged = unmanaged; }}")?;
    w.newline()?;
    indented!(w, [()], r"public void FromManaged({} managed) {{ _managed = managed; }}", name)?;
    indented!(w, [()], r"public void FromUnmanaged(Unmanaged unmanaged) {{ _unmanaged = unmanaged; }}")?;
    w.newline()?;
    indented!(w, [()], r"public unsafe Unmanaged ToUnmanaged()")?;
    indented!(w, [()], r"{{;")?;
    indented!(w, [()()], r"_unmanaged = new Unmanaged();")?;
    w.newline()?;
    for field in the_type.fields() {
        w.indent();
        w.indent();
        write_type_definition_composite_marshaller_field_to_unmanaged(i, w, field, the_type)?;
        w.unindent();
        w.unindent();
    }
    w.newline()?;
    indented!(w, [()()], r"return _unmanaged;")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
    indented!(w, [()], r"public unsafe {} ToManaged()", name)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"_managed = new {}();", name)?;
    w.newline()?;
    for field in the_type.fields() {
        w.indent();
        w.indent();
        write_type_definition_composite_marshaller_field_from_unmanaged(i, w, field, the_type)?;
        w.unindent();
        w.unindent();
    }
    w.newline()?;
    indented!(w, [()()], r"return _managed;")?;
    indented!(w, [()], r"}}")?;
    indented!(w, [()], r"public void Free() {{ }}")?;
    indented!(w, r"}}")?;
    w.unindent();
    indented!(w, r"}}")?;

    Ok(())
}

pub fn write_type_definition_composite_unmanaged_body_field(i: &Interop, w: &mut IndentWriter, field: &Field, the_type: &Composite) -> Result<(), Error> {
    i.debug(w, "write_type_definition_composite_unmanaged_body_field")?;

    let field_name = field_name_to_csharp_name(field, i.rename_symbols);
    match field.the_type() {
        Type::Array(a) => {
            let type_name = to_typespecifier_in_field(a.array_type(), field, the_type);
            let size = a.len();
            if matches!(a.array_type(), Type::Pattern(TypePattern::CChar)) {
                indented!(w, r"public fixed byte {}[{}];", field_name, size)?;
            } else {
                indented!(w, r"public fixed {} {}[{}];", type_name, field_name, size)?;
            }
        }
        Type::Primitive(Primitive::Bool) => {
            indented!(w, r"public sbyte {};", field_name)?;
        }
        Type::Composite(composite) => {
            indented!(w, r"public {}.Unmanaged {};", composite.rust_name(), field_name)?;
        }
        Type::Pattern(TypePattern::NamedCallback(x)) => {
            indented!(w, r"public {}.Unmanaged {};", x.name(), field_name)?;
        }
        Type::Pattern(TypePattern::CStrPointer) => {
            indented!(w, r"public IntPtr {};", field_name)?;
        }
        Type::Pattern(TypePattern::Utf8String(_)) => {
            indented!(w, r"public Utf8String.Unmanaged {};", field_name)?;
        }
        Type::Pattern(TypePattern::FFIErrorEnum(e)) => {
            indented!(w, r"public {} {};", e.the_enum().rust_name(), field_name)?;
        }

        _ => {
            let type_name = to_typespecifier_in_field(field.the_type(), field, the_type);
            indented!(w, r"public {} {};", type_name, field_name)?;
        }
    }
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
    indented!(w, r"{} partial struct {}", i.visibility_types.to_access_modifier(), the_type.rust_name())?;
    indented!(w, r"{{")?;
    w.indent();

    for field in the_type.fields() {
        if write_for == WriteFor::Code {
            write_documentation(w, field.documentation())?;
        }

        write_type_definition_composite_body_field(i, w, field, the_type)?;
    }

    w.unindent();
    indented!(w, r"}}")?;
    w.newline()?;

    Ok(())
}

pub fn write_type_definition_composite_body_field(i: &Interop, w: &mut IndentWriter, field: &Field, the_type: &Composite) -> Result<(), Error> {
    let field_name = field_name_to_csharp_name(field, i.rename_symbols);
    let visibility = match field.visibility() {
        Visibility::Public => "public ",
        Visibility::Private => "",
        // c::Visibility::Private => "",
    };

    match field.the_type() {
        Type::Array(a) => {
            assert!(is_blittable(a.array_type()), "Array type is not blittable: {:?}", a.array_type());

            let type_name = if matches!(a.array_type(), Type::Pattern(TypePattern::CChar)) {
                "string".to_string()
            } else {
                format!("{}[]", to_typespecifier_in_field(a.array_type(), field, the_type))
            };

            indented!(w, r"{}{} {};", visibility, type_name, field_name)?;
        }
        Type::Pattern(TypePattern::FFIErrorEnum(e)) => {
            let enum_name = e.the_enum().rust_name();
            indented!(w, r"{}{} {};", visibility, enum_name, field_name)?;
        }
        _ => {
            let type_name = to_typespecifier_in_field(field.the_type(), field, the_type);
            indented!(w, r"{}{} {};", visibility, type_name, field_name)?;
        }
    }
    Ok(())
}

pub fn write_type_definition_composite_marshaller_field_to_unmanaged(i: &Interop, w: &mut IndentWriter, field: &Field, the_type: &Composite) -> Result<(), Error> {
    i.debug(w, "write_type_definition_composite_marshaller_unmanaged_invoke")?;

    let name = field.name();
    match field.the_type() {
        Type::Primitive(Primitive::Bool) => indented!(w, "_unmanaged.{name} = (sbyte) (_managed.{name} ? 1 : 0);")?,
        Type::Primitive(_) => indented!(w, "_unmanaged.{name} = _managed.{name};")?,
        Type::Enum(_) => indented!(w, "_unmanaged.{name} = _managed.{name};")?,
        Type::ReadPointer(_) => indented!(w, "_unmanaged.{name} = _managed.{name};")?,
        Type::ReadWritePointer(_) => indented!(w, "_unmanaged.{name} = _managed.{name};")?,
        Type::Array(x) => {
            let array_type = to_typespecifier_in_field(x.array_type(), field, the_type);
            indented!(w, "fixed({}* _fixed = _unmanaged.{})", array_type, name)?;
            indented!(w, "{{")?;
            indented!(w, [()], r#"if (_managed.{} == null) {{ throw new InvalidOperationException("Array '{}' must not be null"); }}"#, name, name)?;
            indented!(w, [()], r#"if (_managed.{}.Length != {}) {{ throw new InvalidOperationException("Array size mismatch for '{}'"); }}"#, name, x.len(), name)?;
            indented!(w, [()], "var src = new ReadOnlySpan<{}>(_managed.{}, 0, {});", array_type, name, x.len())?;
            indented!(w, [()], "var dst = new Span<{}>(_fixed, {});", array_type, x.len())?;
            indented!(w, [()], "src.CopyTo(dst);")?;
            indented!(w, "}}")?;
        }
        Type::Pattern(TypePattern::Bool) => indented!(w, "_unmanaged.{name} = (sbyte) (_managed.{name} ? 1 : 0);")?,
        Type::Pattern(TypePattern::FFIErrorEnum(_)) => indented!(w, "_unmanaged.{name} = _managed.{name};")?,
        Type::Pattern(TypePattern::CStrPointer) => {
            indented!(w, "_unmanaged.{name} = Marshal.StringToHGlobalAnsi(_managed.{name});")?;
        }
        Type::Pattern(TypePattern::Utf8String(_)) => {
            indented!(w, "var _{name} = new Utf8String.Marshaller(new Utf8String(_managed.{}));", name)?;
            indented!(w, "_unmanaged.{name} = _{name}.ToUnmanaged();")?;
        }
        x => {
            indented!(w, "var _{name} = new {}.Marshaller(_managed.{});", to_typespecifier_in_field(x, field, the_type), name)?;
            indented!(w, "_unmanaged.{name} = _{name}.ToUnmanaged();")?;
        }
    }

    Ok(())
}

pub fn write_type_definition_composite_marshaller_field_from_unmanaged(i: &Interop, w: &mut IndentWriter, field: &Field, the_type: &Composite) -> Result<(), Error> {
    i.debug(w, "write_type_definition_composite_marshaller_field_from_unmanaged")?;

    let name = field.name();
    match field.the_type() {
        Type::Primitive(Primitive::Bool) => indented!(w, "_managed.{name} = _unmanaged.{name} == 1 ? true : false;")?,
        Type::Primitive(_) => indented!(w, "_managed.{name} = _unmanaged.{name};")?,
        Type::Enum(_) => indented!(w, "_managed.{name} = _unmanaged.{name};")?,
        Type::ReadPointer(_) => indented!(w, "_managed.{name} = _unmanaged.{name};")?,
        Type::ReadWritePointer(_) => indented!(w, "_managed.{name} = _unmanaged.{name};")?,
        Type::Array(x) => {
            let array_type = to_typespecifier_in_field(x.array_type(), field, the_type);
            indented!(w, "fixed({}* _fixed = _unmanaged.{})", array_type, name)?;
            indented!(w, "{{")?;
            indented!(w, [()], "_managed.{} = new {}[{}];", name, array_type, x.len())?;
            indented!(w, [()], "var src = new ReadOnlySpan<{}>(_fixed, {});", array_type, x.len())?;
            indented!(w, [()], "var dst = new Span<{}>(_managed.{}, 0, {});", array_type, name, x.len())?;
            indented!(w, [()], "src.CopyTo(dst);")?;
            indented!(w, "}}")?;
        }
        Type::Pattern(TypePattern::FFIErrorEnum(_)) => indented!(w, "_managed.{name} = _unmanaged.{name};")?,
        Type::Pattern(TypePattern::CStrPointer) => {
            indented!(w, "_managed.{name} = Marshal.PtrToStringAnsi(_unmanaged.{name});")?;
        }
        Type::Pattern(TypePattern::Utf8String(_)) => {
            indented!(w, "var _{name} = new Utf8String.Marshaller(_unmanaged.{name});")?;
            indented!(w, "_managed.{name} = _{name}.ToManaged().String;")?;
        }

        x => {
            indented!(w, "var _{name} = new {}.Marshaller(_unmanaged.{});", to_typespecifier_in_field(x, field, the_type), name)?;
            indented!(w, "_managed.{name} = _{name}.ToManaged();")?;
        }
    }

    Ok(())
}
