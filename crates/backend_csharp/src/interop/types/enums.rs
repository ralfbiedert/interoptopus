use crate::Interop;
use crate::converter::{is_blittable, to_typespecifier_in_field, to_typespecifier_in_field_unmanaged};
use crate::interop::docs::write_documentation;
use interoptopus::backend::IndentWriter;
use interoptopus::lang::{Enum, Type, VariantKind};
use interoptopus::pattern::TypePattern;
use interoptopus::{Error, indented};

pub fn write_type_definition_enum(i: &Interop, w: &mut IndentWriter, the_type: &Enum) -> Result<(), Error> {
    i.debug(w, "write_type_definition_enum")?;
    write_documentation(w, the_type.meta().docs())?;
    write_type_definition_enum_marshaller(i, w, the_type)?;

    Ok(())
}

pub fn write_type_definition_enum_marshaller(i: &Interop, w: &mut IndentWriter, the_type: &Enum) -> Result<(), Error> {
    i.debug(w, "write_type_definition_enum_marshaller")?;
    let name = the_type.rust_name();

    indented!(w, r"public partial struct {}", name)?;
    indented!(w, r"{{")?;
    write_type_definition_enum_variant_fields_managed(i, w, the_type)?;
    indented!(w, r"}}")?;
    w.newline()?;

    indented!(w, r"[NativeMarshalling(typeof(MarshallerMeta))]")?;
    indented!(w, r"public partial struct {}", name)?;
    indented!(w, r"{{")?;

    write_type_definition_enum_variant_unmanaged_types(i, w, the_type)?;

    indented!(w, [()], r"[StructLayout(LayoutKind.Explicit)]")?;
    indented!(w, [()], r"public unsafe struct Unmanaged")?;
    indented!(w, [()], r"{{")?;
    write_type_definition_enum_variant_fields_unmanaged(i, w, the_type)?;
    i.inline_hint(w, 2)?;
    indented!(w, [()()], r"public {name} IntoManaged()")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"var marshaller = new Marshaller(this);")?;
    indented!(w, [()()()], r"try {{ return marshaller.ToManaged(); }}")?;
    indented!(w, [()()()], r"finally {{ marshaller.Free(); }}")?;
    indented!(w, [()()], r"}}")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;

    i.inline_hint(w, 1)?;
    indented!(w, [()], r"public Unmanaged IntoUnmanaged()")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"var marshaller = new Marshaller(this);")?;
    indented!(w, [()()], r"try {{ return marshaller.ToUnmanaged(); }}")?;
    indented!(w, [()()], r"finally {{ marshaller.Free(); }}")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;

    indented!(w, [()], r"[CustomMarshaller(typeof({}), MarshalMode.Default, typeof(Marshaller))]", name)?;
    indented!(w, [()], r"private struct MarshallerMeta {{ }}")?;
    w.newline()?;

    write_type_definition_enum_variant_utils(i, w, the_type)?;

    indented!(w, [()], r"public ref struct Marshaller")?;
    indented!(w, [()], r"{{")?;
    w.indent();
    indented!(w, [()], r"private {} _managed; // Used when converting managed -> unmanaged", name)?;
    indented!(w, [()], r"private Unmanaged _unmanaged; // Used when converting unmanaged -> managed")?;
    w.newline()?;
    i.inline_hint(w, 1)?;
    indented!(w, [()], r"public Marshaller({} managed) {{ _managed = managed; }}", name)?;
    i.inline_hint(w, 1)?;
    indented!(w, [()], r"public Marshaller(Unmanaged unmanaged) {{ _unmanaged = unmanaged; }}")?;
    w.newline()?;
    i.inline_hint(w, 1)?;
    indented!(w, [()], r"public void FromManaged({} managed) {{ _managed = managed; }}", name)?;
    i.inline_hint(w, 1)?;
    indented!(w, [()], r"public void FromUnmanaged(Unmanaged unmanaged) {{ _unmanaged = unmanaged; }}")?;
    w.newline()?;
    i.inline_hint(w, 1)?;
    indented!(w, [()], r"public unsafe Unmanaged ToUnmanaged()")?;
    indented!(w, [()], r"{{;")?;
    indented!(w, [()()], r"_unmanaged = new Unmanaged();")?;
    indented!(w, [()()], r"_unmanaged._variant = _managed._variant;")?;

    write_type_definition_enum_variant_fields_to_unmanaged(i, w, the_type)?;

    indented!(w, [()()], r"return _unmanaged;")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
    i.inline_hint(w, 1)?;
    indented!(w, [()], r"public unsafe {} ToManaged()", name)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"_managed = new {}();", name)?;
    indented!(w, [()()], r"_managed._variant = _unmanaged._variant;")?;

    write_type_definition_enum_variant_fields_to_managed(i, w, the_type)?;

    indented!(w, [()()], r"return _managed;")?;
    indented!(w, [()], r"}}")?;
    indented!(w, [()], r"public void Free() {{ }}")?;
    indented!(w, r"}}")?;
    w.unindent();
    indented!(w, r"}}")?;

    Ok(())
}

pub fn write_type_definition_enum_variant_fields_managed(i: &Interop, w: &mut IndentWriter, the_type: &Enum) -> Result<(), Error> {
    i.debug(w, "write_type_definition_enum_variant_fields_managed")?;

    indented!(w, [()], r"uint _variant;")?;
    for variant in the_type.variants() {
        match variant.kind() {
            VariantKind::Unit(_) => {}
            VariantKind::Typed(_, t) if !t.is_void() => {
                let ty = to_typespecifier_in_field(t);
                let vname = variant.name();
                indented!(w, [()], r"{ty} _{vname};")?;
            }
            VariantKind::Typed(_, _) => {}
        }
    }

    Ok(())
}

pub fn write_type_definition_enum_variant_unmanaged_types(i: &Interop, w: &mut IndentWriter, the_type: &Enum) -> Result<(), Error> {
    i.debug(w, "write_type_definition_enum_variant_unmanaged_types")?;

    for variant in the_type.variants() {
        match variant.kind() {
            VariantKind::Unit(_) => {}
            VariantKind::Typed(_, t) if !t.is_void() => {
                let ty = to_typespecifier_in_field_unmanaged(t);
                let vname = variant.name();
                indented!(w, [()], r"[StructLayout(LayoutKind.Sequential)]")?;
                indented!(w, [()], r"internal unsafe struct Unmanaged{vname}")?;
                indented!(w, [()], r"{{")?;
                indented!(w, [()()], r"internal uint _variant;")?;
                indented!(w, [()()], r"internal {ty} _{vname};")?;
                indented!(w, [()], r"}}")?;
            }
            VariantKind::Typed(_, _) => {}
        }

        w.newline()?;
    }

    Ok(())
}

pub fn write_type_definition_enum_variant_fields_unmanaged(i: &Interop, w: &mut IndentWriter, the_type: &Enum) -> Result<(), Error> {
    i.debug(w, "write_type_definition_enum_variant_fields_unmanaged")?;

    indented!(w, [()()], r"[FieldOffset(0)]")?;
    indented!(w, [()()], r"internal uint _variant;")?;
    w.newline()?;
    for variant in the_type.variants() {
        match variant.kind() {
            VariantKind::Unit(_) => {}
            VariantKind::Typed(_, t) if !t.is_void() => {
                let vname = variant.name();
                indented!(w, [()()], r"[FieldOffset(0)]")?;
                indented!(w, [()()], r"internal Unmanaged{vname} _{vname};")?;
                w.newline()?;
            }
            VariantKind::Typed(_, _) => {}
        }
    }

    Ok(())
}

pub fn write_type_definition_enum_variant_utils(i: &Interop, w: &mut IndentWriter, the_type: &Enum) -> Result<(), Error> {
    i.debug(w, "write_type_definition_enum_variant_utils")?;
    let name = the_type.rust_name();

    // Constructors
    for variant in the_type.variants() {
        match variant.kind() {
            VariantKind::Unit(x) => {
                let vname = variant.name();
                indented!(w, [()], r"public static {name} {vname} => new() {{ _variant = {x} }};")?;
            }
            VariantKind::Typed(x, t) if !t.is_void() => {
                let vname = variant.name();
                let ty = to_typespecifier_in_field(t);
                indented!(w, [()], r"public static {name} {vname}({ty} value) => new() {{ _variant = {x}, _{vname} = value }};")?;
            }
            VariantKind::Typed(x, _) => {
                let vname = variant.name();
                indented!(w, [()], r"public static {name} {vname} => new() {{ _variant = {x} }};")?;
            }
        }
    }
    w.newline()?;

    // Is... checks
    for variant in the_type.variants() {
        match variant.kind() {
            VariantKind::Unit(x) => {
                let vname = variant.name();
                indented!(w, [()], r"public bool Is{vname} => _variant == {x};")?;
            }
            VariantKind::Typed(x, _) => {
                let vname = variant.name();
                indented!(w, [()], r"public bool Is{vname} => _variant == {x};")?;
            }
        }
    }
    w.newline()?;

    // As... "unwraps"
    for variant in the_type.variants() {
        let throw = "throw new InteropException();";
        match variant.kind() {
            VariantKind::Unit(x) => {
                let vname = variant.name();
                indented!(w, [()], r"public void As{vname}() {{ if (_variant != {x}) {throw} }}")?;
            }
            VariantKind::Typed(x, t) if !t.is_void() => {
                let vname = variant.name();
                let ty = to_typespecifier_in_field(t);
                indented!(w, [()], r"public {ty} As{vname}() {{ if (_variant != {x}) {{ {throw} }} else {{ return _{vname}; }} }}")?;
            }
            VariantKind::Typed(x, _) => {
                let vname = variant.name();
                indented!(w, [()], r"public void As{vname}() {{ if (_variant != {x}) {throw} }}")?;
            }
        }
    }
    w.newline()?;

    Ok(())
}

#[allow(clippy::match_wildcard_for_single_variants)]
pub fn write_type_definition_enum_variant_fields_to_unmanaged(i: &Interop, w: &mut IndentWriter, the_type: &Enum) -> Result<(), Error> {
    i.debug(w, "write_type_definition_enum_variant_fields_to_unmanaged")?;

    for variant in the_type.variants() {
        match variant.kind() {
            VariantKind::Unit(_) => (),
            VariantKind::Typed(_, t) if t.is_void() => (),
            VariantKind::Typed(x, t) if !t.is_void() => {
                let vname = variant.name();

                let convert = match &**t {
                    Type::Primitive(_) => format!("_managed._{vname}"),
                    Type::ReadWritePointer(_) => format!("_managed._{vname}"),
                    Type::ReadPointer(_) => format!("_managed._{vname}"),
                    _ if is_blittable(t) => format!("_managed._{vname}.ToUnmanaged()"),
                    _ => format!("_managed._{vname}.IntoUnmanaged()"),
                };

                indented!(w, [()()], r"if (_unmanaged._variant == {x}) _unmanaged._{vname}._{vname} = {convert};")?;
            }
            _ => panic!("This should never happen"),
        }
    }

    Ok(())
}

#[allow(clippy::match_wildcard_for_single_variants)]
pub fn write_type_definition_enum_variant_fields_to_managed(i: &Interop, w: &mut IndentWriter, the_type: &Enum) -> Result<(), Error> {
    i.debug(w, "write_type_definition_enum_variant_fields_to_managed")?;

    for variant in the_type.variants() {
        match variant.kind() {
            VariantKind::Unit(_) => (),
            VariantKind::Typed(_, t) if t.is_void() => (),
            VariantKind::Typed(x, t) if !t.is_void() => {
                let vname = variant.name();

                let convert = match &**t {
                    Type::Primitive(_) => format!("_unmanaged._{vname}._{vname}"),
                    Type::ReadWritePointer(_) => format!("_unmanaged._{vname}._{vname}"),
                    Type::ReadPointer(_) => format!("_unmanaged._{vname}._{vname}"),
                    Type::Pattern(TypePattern::Utf8String(_)) => format!("_unmanaged._{vname}._{vname}.IntoManaged()"),
                    _ if is_blittable(t) => format!("_unmanaged._{vname}._{vname}.ToManaged()"),
                    _ => format!("_unmanaged._{vname}._{vname}.IntoManaged()"),
                };

                indented!(w, [()()], r"if (_managed._variant == {x}) _managed._{vname} = {convert};")?;
            }
            _ => panic!("This should never happen"),
        }
    }

    Ok(())
}
