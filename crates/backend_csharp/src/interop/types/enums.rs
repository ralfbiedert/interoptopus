use crate::Interop;
use crate::converter::{field_as_unmanaged, field_to_managed, field_to_type, field_to_type_unmanaged, field_to_unmanaged, has_dispose, is_reusable};
use crate::interop::docs::write_documentation;
use crate::utils::{MoveSemantics, write_common_marshaller};
use interoptopus::backend::IndentWriter;
use interoptopus::lang::{Enum, Field, VariantKind};
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
    let self_kind = if is_reusable(&the_type.to_type()) { "struct" } else { "class" };
    let into = if is_reusable(&the_type.to_type()) { "To" } else { "Into" };
    let move_semantics = if is_reusable(&the_type.to_type()) {
        MoveSemantics::Copy
    } else {
        MoveSemantics::Move
    };
    let idisposable = if has_dispose(&the_type.to_type()) { ": IDisposable" } else { "" };

    indented!(w, r"public partial {self_kind} {name}")?;
    indented!(w, r"{{")?;
    write_type_definition_enum_variant_fields_managed(i, w, the_type)?;
    indented!(w, r"}}")?;
    w.newline()?;

    indented!(w, r"[NativeMarshalling(typeof(MarshallerMeta))]")?;
    indented!(w, r"public partial {self_kind} {name} {idisposable}")?;
    indented!(w, r"{{")?;

    write_type_definition_enum_variant_unmanaged_types(i, w, the_type)?;

    if !idisposable.is_empty() {
        indented!(w, [()], r"public void Dispose()")?;
        indented!(w, [()], r"{{")?;
        for variant in the_type.variants() {
            let VariantKind::Typed(i, t) = variant.kind() else {
                continue;
            };

            if !has_dispose(t) {
                continue;
            }

            let name = variant.name();
            indented!(w, [()()], r"if (_variant == {i}) {{ _{name}.Dispose(); }}")?;
        }
        indented!(w, [()], r"}}")?;
    }
    w.newline()?;

    indented!(w, [()], r"[StructLayout(LayoutKind.Explicit)]")?;
    indented!(w, [()], r"public unsafe struct Unmanaged")?;
    indented!(w, [()], r"{{")?;
    write_type_definition_enum_variant_fields_unmanaged(i, w, the_type)?;
    i.inline_hint(w, 2)?;
    indented!(w, [()()], r"internal {name} {into}Managed()")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"var _managed = new {name}();")?;
    indented!(w, [()()()], r"_managed._variant = _variant;")?;
    write_type_definition_enum_variant_fields_to_managed(i, w, the_type)?;
    indented!(w, [()()()], r"return _managed;")?;
    indented!(w, [()()], r"}}")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;

    i.inline_hint(w, 1)?;
    indented!(w, [()], r"internal Unmanaged {into}Unmanaged()")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"var _unmanaged = new Unmanaged();")?;
    indented!(w, [()()], r"_unmanaged._variant = _variant;")?;
    write_type_definition_enum_variant_fields_to_unmanaged(i, w, the_type)?;
    indented!(w, [()()], r"return _unmanaged;")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;

    i.inline_hint(w, 1)?;
    indented!(w, [()], r"internal Unmanaged AsUnmanaged()")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"var _unmanaged = new Unmanaged();")?;
    indented!(w, [()()], r"_unmanaged._variant = _variant;")?;
    write_type_definition_enum_variant_fields_as_unmanaged(i, w, the_type)?;
    indented!(w, [()()], r"return _unmanaged;")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;

    indented!(w, [()], r"[CustomMarshaller(typeof({name}), MarshalMode.Default, typeof(Marshaller))]")?;
    indented!(w, [()], r"private struct MarshallerMeta {{ }}")?;
    w.newline()?;

    write_type_definition_enum_variant_utils(i, w, the_type)?;

    write_common_marshaller(i, w, name, move_semantics)?;

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
                let ty = field_to_type(t);
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
                let ty = field_to_type_unmanaged(t);
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
                let ty = field_to_type(t);
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
                let ty = field_to_type(t);
                indented!(w, [()], r"public {ty} As{vname}() {{ if (_variant != {x}) {{ {throw} }} else {{ return _{vname}; }} }}")?;
            }
            VariantKind::Typed(x, _) => {
                let vname = variant.name();
                indented!(w, [()], r"public void As{vname}() {{ if (_variant != {x}) {throw} }}")?;
            }
        }
    }
    w.newline()?;

    // Somewhat of a ToString implementation.
    indented!(w, [()], r"public override readonly string ToString() {{")?;
    indented!(w, [()()], r"return _variant switch {{")?;
    for variant in the_type.variants() {
        let vname = format!(r#""{}""#, variant.name());
        match variant.kind() {
            VariantKind::Unit(x) => {
                indented!(w, [()()], r"{x} => {vname},")?;
            }
            VariantKind::Typed(x, _) => {
                indented!(w, [()()], r"{x} => {vname},")?;
            }
        }
    }
    let throw = "throw new InteropException()";
    indented!(w, [()()], r"_ => {throw}")?;
    indented!(w, [()()], r"}};")?;
    indented!(w, [()], r"}}")?;
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
                let convert = field_to_unmanaged(&Field::new(vname.to_string(), t.to_type()));
                indented!(w, [()()], r"if (_variant == {x}) _unmanaged._{vname}._{vname} = _{convert};")?;
            }
            _ => panic!("This should never happen"),
        }
    }

    Ok(())
}

#[allow(clippy::match_wildcard_for_single_variants)]
pub fn write_type_definition_enum_variant_fields_as_unmanaged(i: &Interop, w: &mut IndentWriter, the_type: &Enum) -> Result<(), Error> {
    i.debug(w, "write_type_definition_enum_variant_fields_as_unmanaged")?;

    for variant in the_type.variants() {
        match variant.kind() {
            VariantKind::Unit(_) => (),
            VariantKind::Typed(_, t) if t.is_void() => (),
            VariantKind::Typed(x, t) if !t.is_void() => {
                let vname = variant.name();
                let convert = field_as_unmanaged(&Field::new(vname.to_string(), t.to_type()));
                indented!(w, [()()], r"if (_variant == {x}) _unmanaged._{vname}._{vname} = _{convert};")?;
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
                let convert = field_to_managed(&Field::new(vname.to_string(), t.to_type()));
                indented!(w, [()()()], r"if (_variant == {x}) _managed._{vname} = _{vname}._{convert};")?;
            }
            _ => panic!("This should never happen"),
        }
    }

    Ok(())
}
