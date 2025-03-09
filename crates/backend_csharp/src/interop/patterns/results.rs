use crate::converter::to_typespecifier_in_sync_fn_rval;
use crate::Interop;
use interoptopus::lang::c::CType;
use interoptopus::patterns::result::{FFIErrorEnum, FFIResultType};
use interoptopus::patterns::TypePattern;
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error};

pub fn write_pattern_result(i: &Interop, w: &mut IndentWriter, result: &FFIResultType) -> Result<(), Error> {
    i.debug(w, "write_pattern_result")?;
    let enum_name = result.e().the_enum().rust_name();
    let context_type_name = result.composite().rust_name();
    let e = result.e();
    let t = result.t();
    let e_name = e.the_enum().rust_name();
    let success_variant = result.e().success_variant().name();

    let type_string = match t {
        CType::Pattern(TypePattern::Utf8String(_)) => "string".to_string(),
        _ => to_typespecifier_in_sync_fn_rval(t),
    };

    indented!(w, r"{} partial struct {}", i.visibility_types.to_access_modifier(), context_type_name)?;
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
    indented!(w, r"}}")?;
    w.newline()?;
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
