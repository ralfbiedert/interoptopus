use crate::converter::to_typespecifier_in_rval;
use crate::Interop;
use interoptopus::lang::c::CType;
use interoptopus::patterns::result::{FFIErrorEnum, FFIResultType};
use interoptopus::patterns::TypePattern;
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error};

pub fn write_pattern_result(i: &Interop, w: &mut IndentWriter, result: &FFIResultType) -> Result<(), Error> {
    i.debug(w, "write_pattern_result")?;
    let enum_name = result.e().the_enum().rust_name();
    let ffi_error = i
        .inventory
        .ctypes()
        .iter()
        .find_map(|x| match x {
            CType::Pattern(TypePattern::FFIErrorEnum(e)) => Some(e),
            _ => None,
        })
        .expect("When using result type there must be an FFIError in the inventory with an `ok` variant.");

    let context_type_name = result.composite().rust_name();
    let data_type = result
        .composite()
        .fields()
        .iter()
        .find(|x| x.name().eq("t"))
        .expect("Option must contain field called 't'.")
        .the_type();

    let type_string = match data_type {
        CType::Pattern(TypePattern::Utf8String(_)) => "string".to_string(),
        _ => to_typespecifier_in_rval(data_type),
    };

    indented!(w, r"{} partial struct {}", i.visibility_types.to_access_modifier(), context_type_name)?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"public {} Ok()", type_string)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"if (err == {enum_name}.{})", result.e().success_variant().name())?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"return t;")?;
    indented!(w, [()()], r"}}")?;
    indented!(w, [()()], r"throw new InteropException<{}>(err);", ffi_error.the_enum().rust_name())?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
    indented!(w, r"}}")?;
    w.newline()?;
    Ok(())
}

pub fn write_pattern_result_void(i: &Interop, w: &mut IndentWriter, result: &FFIErrorEnum) -> Result<(), Error> {
    i.debug(w, "write_pattern_result_void")?;
    let enum_name = result.the_enum().rust_name();
    let name = format!("Result{enum_name}");
    indented!(w, r"public partial struct {name}")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"internal FFIError _err;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, r"[NativeMarshalling(typeof(MarshallerMeta))]")?;
    indented!(w, r"public partial struct {name}")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"public {name}({enum_name} e) {{ _err = e; }}",)?;
    w.newline()?;
    indented!(w, [()], r"public void Ok()")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"if (_err == {enum_name}.{})", result.success_variant().name())?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"return;")?;
    indented!(w, [()()], r"}}")?;
    indented!(w, [()()], r"throw new InteropException<{}>(_err);", result.the_enum().rust_name())?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
    indented!(w, [()], r"[StructLayout(LayoutKind.Sequential)]")?;
    indented!(w, [()], r"public unsafe struct Unmanaged")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"public FFIError _err;")?;
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
