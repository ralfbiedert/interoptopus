use crate::converter::to_typespecifier_in_field;
use crate::interop::docs::write_documentation;
use crate::Interop;
use interoptopus::backend::IndentWriter;
use interoptopus::lang::{Enum, VariantKind};
use interoptopus::{indented, Error};

pub fn write_type_definition_enum(i: &Interop, w: &mut IndentWriter, the_type: &Enum) -> Result<(), Error> {
    i.debug(w, "write_type_definition_enum")?;
    write_documentation(w, the_type.meta().documentation())?;
    write_type_definition_enum_marshaller(i, w, the_type)?;

    Ok(())
}

pub fn write_type_definition_enum_marshaller(i: &Interop, w: &mut IndentWriter, the_type: &Enum) -> Result<(), Error> {
    i.debug(w, "write_type_definition_enum_marshaller")?;
    let name = the_type.rust_name();

    indented!(w, r"public partial struct {}", name)?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"uint _variant;")?;
    for variant in the_type.variants() {
        match variant.kind() {
            VariantKind::Unit(_) => {}
            VariantKind::Typed(x, t) if !t.is_void() => {
                let ty = to_typespecifier_in_field(t);
                let vname = variant.name();
                indented!(w, [()], r"{ty} _{vname};")?;
            }
            VariantKind::Typed(x, t) => {}
        }
    }
    indented!(w, r"}}")?;
    w.newline()?;

    indented!(w, r"public partial struct {}", name)?;
    indented!(w, r"{{")?;
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
            VariantKind::Typed(x, t) => {
                let vname = variant.name();
                indented!(w, [()], r"public static {name} {vname} => new() {{ _variant = {x} }};")?;
            }
        }
    }
    w.newline()?;
    for variant in the_type.variants() {
        match variant.kind() {
            VariantKind::Unit(x) => {
                let vname = variant.name();
                indented!(w, [()], r"public bool Is{vname} => _variant == {x};")?;
            }
            VariantKind::Typed(x, t) => {
                let vname = variant.name();
                indented!(w, [()], r"public bool Is{vname} => _variant == {x};")?;
            }
        }
    }
    w.newline()?;

    indented!(w, [()], r"[StructLayout(LayoutKind.Explicit)]")?;
    indented!(w, [()], r"public unsafe struct Unmanaged")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"[FieldOffset(0)]")?;
    indented!(w, [()()], r"internal uint _variant;")?;
    w.newline()?;
    for variant in the_type.variants() {
        match variant.kind() {
            VariantKind::Unit(_) => {}
            VariantKind::Typed(x, t) if !t.is_void() => {
                let ty = to_typespecifier_in_field(t);
                let vname = variant.name();
                indented!(w, [()()], r"[FieldOffset(2)]")?;
                indented!(w, [()()], r"internal {ty} {vname};")?;
                w.newline()?;
            }
            VariantKind::Typed(x, t) => {}
        }
    }
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
    indented!(w, [()()], r"_unmanaged._variant = _managed._variant;")?;
    indented!(w, [()()], r"return _unmanaged;")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
    indented!(w, [()], r"public unsafe {} ToManaged()", name)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"_managed = new {}();", name)?;
    indented!(w, [()()], r"_managed._variant = _unmanaged._variant;")?;
    indented!(w, [()()], r"return _managed;")?;
    indented!(w, [()], r"}}")?;
    indented!(w, [()], r"public void Free() {{ }}")?;
    indented!(w, r"}}")?;
    w.unindent();
    indented!(w, r"}}")?;

    Ok(())
}
