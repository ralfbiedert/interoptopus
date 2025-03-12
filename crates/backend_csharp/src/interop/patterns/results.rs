use crate::Interop;
use crate::converter::to_typespecifier_in_sync_fn_rval;
use crate::interop::docs::write_documentation;
use crate::interop::types::composite::{
    write_type_definition_composite_body, write_type_definition_composite_layout_annotation, write_type_definition_composite_marshaller_field_from_unmanaged,
    write_type_definition_composite_marshaller_field_to_unmanaged, write_type_definition_composite_unmanaged_body_field,
};
use interoptopus::lang::c::{CType, Field};
use interoptopus::patterns::TypePattern;
use interoptopus::patterns::result::{FFIErrorEnum, FFIResultType};
use interoptopus::writer::{IndentWriter, WriteFor};
use interoptopus::{Error, indented};

pub fn write_pattern_result(i: &Interop, w: &mut IndentWriter, result: &FFIResultType) -> Result<(), Error> {
    i.debug(w, "write_pattern_result")?;

    let enum_name = result.e().the_enum().rust_name();
    let name = result.composite().rust_name();
    let e = result.e();
    let t = result.t();
    let e_name = e.the_enum().rust_name();
    let success_variant = result.e().success_variant().name();

    let type_string = match t {
        CType::Pattern(TypePattern::Utf8String(_)) => "string".to_string(),
        _ => to_typespecifier_in_sync_fn_rval(t),
    };

    write_documentation(w, result.meta().documentation())?;
    write_type_definition_composite_body(i, w, result.composite(), WriteFor::Code)?;

    indented!(w, r"[NativeMarshalling(typeof(MarshallerMeta))]")?;
    indented!(w, r"public partial struct {}", name)?;
    indented!(w, r"{{")?;

    indented!(w, [()], r"public {} Ok()", type_string)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"if (err == {enum_name}.{success_variant})")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"return t;")?;
    indented!(w, [()()], r"}}")?;
    indented!(w, [()()], r"throw new InteropException<{e_name}>(err);")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
    indented!(w, [()], r"public bool IsOk() {{ return err == {enum_name}.{success_variant}; }}")?;
    indented!(w, [()], r"public {enum_name} Err() {{ return err; }}")?;
    w.newline()?;

    indented!(w, [()], r"public {name}({name} other)")?;
    indented!(w, [()], r"{{")?;
    for field in result.composite().fields() {
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
    write_type_definition_composite_layout_annotation(w, result.composite())?;
    w.unindent();

    indented!(w, [()], r"public unsafe struct Unmanaged")?;
    indented!(w, [()], r"{{")?;
    for field in result.composite().fields() {
        w.indent();
        w.indent();
        write_type_definition_composite_unmanaged_body_field(i, w, field, result.composite())?;
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
    for field in result.composite().fields() {
        w.indent();
        w.indent();
        write_type_definition_composite_marshaller_field_to_unmanaged(i, w, field, result.composite())?;
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
    let field_t = Field::new("t".to_string(), t.clone());
    let field_e = Field::new("err".to_string(), CType::Pattern(TypePattern::FFIErrorEnum(e.clone())));
    w.newline()?;
    w.indent();
    w.indent();
    write_type_definition_composite_marshaller_field_from_unmanaged(i, w, &field_e, result.composite())?;
    indented!(w, r"if (_managed.err == {enum_name}.{success_variant})")?;
    indented!(w, r"{{")?;
    w.indent();
    write_type_definition_composite_marshaller_field_from_unmanaged(i, w, &field_t, result.composite())?;
    w.unindent();
    indented!(w, r"}}")?;
    w.unindent();
    w.unindent();
    w.newline()?;
    indented!(w, [()()], r"return _managed;")?;
    indented!(w, [()], r"}}")?;
    indented!(w, [()], r"public void Free() {{ }}")?;
    indented!(w, r"}}")?;
    w.unindent();
    indented!(w, r"}}")?;

    Ok(())
}

pub fn write_pattern_result_void(i: &Interop, w: &mut IndentWriter, error: &FFIErrorEnum) -> Result<(), Error> {
    i.debug(w, "write_pattern_result_void")?;
    let enum_name = error.the_enum().rust_name();
    let name = format!("Result{enum_name}");
    let success_variant = error.success_variant().name();

    indented!(w, r"public partial struct {name}")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"internal {enum_name} _err;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, r"[NativeMarshalling(typeof(MarshallerMeta))]")?;
    indented!(w, r"public partial struct {name}")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"public {name}({enum_name} e) {{ _err = e; }}",)?;
    w.newline()?;
    for x in error.the_enum().variants() {
        let vname = x.name();
        let vname_upper = vname.to_uppercase();
        indented!(w, [()], r"public static {name} {vname_upper} => new {name}({enum_name}.{vname});",)?;
    }
    w.newline()?;
    indented!(w, [()], r"public void Ok()")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"if (_err == {enum_name}.{success_variant})")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"return;")?;
    indented!(w, [()()], r"}}")?;
    indented!(w, [()()], r"throw new InteropException<{enum_name}>(_err);")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
    indented!(w, [()], r"public bool IsOk() {{ return _err == {enum_name}.{success_variant}; }}")?;
    indented!(w, [()], r"public {enum_name} Err() {{ return _err; }}")?;
    w.newline()?;
    indented!(w, [()], r"[StructLayout(LayoutKind.Sequential)]")?;
    indented!(w, [()], r"public unsafe struct Unmanaged")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"public {enum_name} _err;")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
    indented!(w, [()], r"[CustomMarshaller(typeof({name}), MarshalMode.Default, typeof(Marshaller))]")?;
    indented!(w, [()], r"private struct MarshallerMeta {{ }}")?;
    w.newline()?;
    w.newline()?;
    indented!(w, [()], r"public ref struct Marshaller")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"private {name} _managed; // Used when converting managed -> unmanaged")?;
    indented!(w, [()()], r"private Unmanaged _unmanaged; // Used when converting unmanaged -> managed")?;
    w.newline()?;
    indented!(w, [()()], r"public Marshaller({name} managed) {{ _managed = managed; }}")?;
    indented!(w, [()()], r"public Marshaller(Unmanaged unmanaged) {{ _unmanaged = unmanaged; }}")?;
    w.newline()?;
    indented!(w, [()()], r"public void FromManaged({name} managed) {{ _managed = managed; }}")?;
    indented!(w, [()()], r"public void FromUnmanaged(Unmanaged unmanaged) {{ _unmanaged = unmanaged; }}")?;
    w.newline()?;
    indented!(w, [()()], r"public unsafe Unmanaged ToUnmanaged()")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"_unmanaged = new Unmanaged();")?;
    indented!(w, [()()()], r"_unmanaged._err = _managed._err;")?;
    indented!(w, [()()()], r"return _unmanaged;")?;
    indented!(w, [()()], r"}}")?;
    w.newline()?;
    indented!(w, [()()], r"public unsafe {name} ToManaged()")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"_managed = new {name}();")?;
    indented!(w, [()()()], r"_managed._err = _unmanaged._err;")?;
    indented!(w, [()()()], r"return _managed;")?;
    indented!(w, [()()], r"}}")?;
    w.newline()?;
    indented!(w, [()()], r"public void Free() {{ }}")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
    indented!(w, r"}}")?;
    w.newline()?;
    Ok(())
}
