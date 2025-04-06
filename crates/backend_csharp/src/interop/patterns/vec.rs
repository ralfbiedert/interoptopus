use crate::Interop;
use crate::converter::{is_reusable, param_to_type, vec_t};
use crate::utils::{MoveSemantics, write_common_marshaller};
use interoptopus::backend::IndentWriter;
use interoptopus::lang::{Parameter, Type};
use interoptopus::pattern::TypePattern;
use interoptopus::pattern::vec::VecType;
use interoptopus::{Error, indented};

pub fn write_pattern_vec(i: &Interop, w: &mut IndentWriter, vec: &VecType) -> Result<(), Error> {
    i.debug(w, "write_pattern_vec")?;
    if is_reusable(vec.t()) {
        write_pattern_fast_vec(i, w, vec)
    } else {
        write_pattern_marshalling_vec(i, w, vec)
    }
}

pub fn write_pattern_fast_vec(i: &Interop, w: &mut IndentWriter, vec: &VecType) -> Result<(), Error> {
    i.debug(w, "write_pattern_fast_vec")?;

    let name = vec.rust_name();
    let the_type = param_to_type(vec.t());

    write_pattern_vec_struct(i, w, vec)?;

    ////
    indented!(w, r"[NativeMarshalling(typeof(MarshallerMeta))]")?;
    indented!(w, r"public partial class {} : IDisposable", name)?;
    indented!(w, r"{{")?;
    w.indent();

    indented!(w, r"// An internal helper to create an empty object.")?;
    i.inline_hint(w, 0)?;
    indented!(w, r"private {name}() {{ }}")?;
    w.newline()?;

    i.inline_hint(w, 0)?;
    indented!(w, r"public static unsafe {name} From(Span<{the_type}> _data)")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"var rval = new {name}();")?;
    indented!(w, [()], r"fixed (void* _data_ptr = _data)")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"InteropHelper.interoptopus_vec_create((IntPtr) _data_ptr, (ulong)_data.Length, out var _out);")?;
    indented!(w, [()()], r"rval._len = _out._len;")?;
    indented!(w, [()()], r"rval._capacity = _out._capacity;")?;
    indented!(w, [()()], r"rval._ptr = _out._ptr;")?;
    indented!(w, [()], r"}}")?;
    indented!(w, [()], r"return rval;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    i.inline_hint(w, 0)?;
    indented!(w, r"public static unsafe {name} Empty()")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"InteropHelper.interoptopus_vec_create(IntPtr.Zero, 0, out var _out);")?;
    indented!(w, [()], r"return _out.IntoManaged();")?;
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, r"public int Count")?;
    indented!(w, r"{{")?;
    i.inline_hint(w, 1)?;
    indented!(w, [()], r"get {{ if (_ptr == IntPtr.Zero) {{ throw new InteropException(); }} else {{ return (int) _len; }} }}")?;
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, r"public unsafe {} this[int i]", the_type)?;
    indented!(w, r"{{")?;
    w.indent();
    i.inline_hint(w, 0)?;
    indented!(w, r"get")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"if (i >= Count) throw new IndexOutOfRangeException();")?;
    indented!(w, [()], r"if (_ptr == IntPtr.Zero) throw new InteropException();")?;
    indented!(w, [()], r"return Marshal.PtrToStructure<{}>(new IntPtr(_ptr.ToInt64() + i * sizeof({})));", the_type, the_type)?;
    indented!(w, r"}}")?;
    w.unindent();
    indented!(w, r"}}")?;
    w.newline()?;
    write_pattern_vec_to_unmanaged(i, w)?;
    w.newline()?;
    write_pattern_vec_helpers(i, w, vec)?;
    w.newline()?;
    write_pattern_vec_interop_helper(i, w, vec)?;
    w.newline()?;
    indented!(w, r"[CustomMarshaller(typeof({}), MarshalMode.Default, typeof(Marshaller))]", name)?;
    indented!(w, r"private struct MarshallerMeta {{ }}")?;
    w.newline()?;
    indented!(w, r"[StructLayout(LayoutKind.Sequential)]")?;
    indented!(w, r"public struct Unmanaged")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"internal IntPtr _ptr;")?;
    indented!(w, [()], r"internal ulong _len;")?;
    indented!(w, [()], r"internal ulong _capacity;")?;
    w.newline()?;
    write_pattern_vec_to_managed(i, w, name)?;
    w.newline()?;
    indented!(w, r"}}")?;
    w.newline()?;
    w.unindent();
    write_common_marshaller(i, w, name, MoveSemantics::Move)?;
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, r"public static class {name}Extensions")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"public static {name} Vec(this {the_type}[] s) {{ return {name}.From(s); }}")?;
    indented!(w, r"}}")?;
    w.newline()?;
    Ok(())
}

pub fn write_pattern_marshalling_vec(i: &Interop, w: &mut IndentWriter, vec: &VecType) -> Result<(), Error> {
    i.debug(w, "write_pattern_fast_vec")?;

    let name = vec.rust_name();
    let the_type = vec_t(vec);

    write_pattern_vec_struct(i, w, vec)?;

    ////
    indented!(w, r"[NativeMarshalling(typeof(MarshallerMeta))]")?;
    indented!(w, r"public partial class {} : IDisposable", name)?;
    indented!(w, r"{{")?;
    w.indent();

    indented!(w, r"// An internal helper to create an empty object.")?;
    i.inline_hint(w, 0)?;
    indented!(w, r"private {name}() {{ }}")?;
    w.newline()?;

    i.inline_hint(w, 0)?;
    indented!(w, r"public static unsafe {name} From(Span<{the_type}> _data)")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"var _temp = new {the_type}.Unmanaged[_data.Length];")?;
    indented!(w, [()], r"for (var i = 0; i < _data.Length; ++i)")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"_temp[i] = _data[i].IntoUnmanaged();")?;
    indented!(w, [()], r"}}")?;
    indented!(w, [()], r"fixed (void* _data_ptr = _temp)")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"InteropHelper.interoptopus_vec_create((IntPtr) _data_ptr, (ulong)_data.Length, out var _out);")?;
    indented!(w, [()()], r"return _out.IntoManaged();")?;
    indented!(w, [()], r"}}")?;
    indented!(w, r"}}")?;
    w.newline()?;
    i.inline_hint(w, 0)?;
    indented!(w, r"public static unsafe {name} Empty()")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"InteropHelper.interoptopus_vec_create(IntPtr.Zero, 0, out var _out);")?;
    indented!(w, [()], r"return _out.IntoManaged();")?;
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, r"public int Count")?;
    indented!(w, r"{{")?;
    i.inline_hint(w, 1)?;
    indented!(w, [()], r"get {{ if (_ptr == IntPtr.Zero) {{ throw new InteropException(); }} else {{ return (int) _len; }} }}")?;
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, r"public unsafe {the_type} this[int i]")?;
    indented!(w, r"{{")?;
    w.indent();
    i.inline_hint(w, 0)?;
    indented!(w, r"get")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"if (i >= Count) throw new IndexOutOfRangeException();")?;
    indented!(w, [()], r"if (_ptr == IntPtr.Zero) throw new InteropException();")?;
    indented!(w, [()], r"var _element = Marshal.PtrToStructure<{the_type}.Unmanaged>(new IntPtr(_ptr.ToInt64() + i * sizeof({the_type}.Unmanaged)));")?;
    indented!(w, [()], r"return _element.IntoManaged();")?;
    indented!(w, r"}}")?;
    w.unindent();
    indented!(w, r"}}")?;
    write_pattern_vec_to_unmanaged(i, w)?;
    w.newline()?;
    write_pattern_vec_helpers(i, w, vec)?;
    w.newline()?;
    write_pattern_vec_interop_helper(i, w, vec)?;
    w.newline()?;
    indented!(w, r"[CustomMarshaller(typeof({}), MarshalMode.Default, typeof(Marshaller))]", name)?;
    indented!(w, r"private struct MarshallerMeta {{ }}")?;
    w.newline()?;
    indented!(w, r"[StructLayout(LayoutKind.Sequential)]")?;
    indented!(w, r"public struct Unmanaged")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"internal IntPtr _ptr;")?;
    indented!(w, [()], r"internal ulong _len;")?;
    indented!(w, [()], r"internal ulong _capacity;")?;
    write_pattern_vec_to_managed(i, w, name)?;
    w.newline()?;
    indented!(w, r"}}")?;
    w.unindent();
    w.newline()?;
    write_common_marshaller(i, w, name, MoveSemantics::Move)?;
    w.newline()?;
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, r"public static class {name}Extensions")?;
    indented!(w, r"{{")?;
    i.inline_hint(w, 1)?;
    indented!(w, [()], r"public static {name} IntoVec(this {the_type}[] s) {{ return {name}.From(s); }}")?;
    indented!(w, r"}}")?;
    w.newline()?;

    Ok(())
}

pub fn write_pattern_vec_struct(_: &Interop, w: &mut IndentWriter, vec: &VecType) -> Result<(), Error> {
    let name = vec.rust_name();

    indented!(w, r"// This must be a class because we only ever want to hold on to the")?;
    indented!(w, r"// same instance, as we overwrite fields when this is sent over the FFI")?;
    indented!(w, r"// boundary")?;
    indented!(w, r"public partial class {}", name)?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"internal IntPtr _ptr;")?;
    indented!(w, [()], r"internal ulong _len;")?;
    indented!(w, [()], r"internal ulong _capacity;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    Ok(())
}

pub fn write_pattern_vec_helpers(i: &Interop, w: &mut IndentWriter, v: &VecType) -> Result<(), Error> {
    let name = v.rust_name();

    i.inline_hint(w, 0)?;
    indented!(w, r"public void Dispose()")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"if (_ptr == IntPtr.Zero) return;")?;
    indented!(w, [()], r"var _unmanaged = new Unmanaged();")?;
    indented!(w, [()], r"_unmanaged._ptr = _unmanaged._ptr;")?;
    indented!(w, [()], r"_unmanaged._len = _unmanaged._len;")?;
    indented!(w, [()], r"_unmanaged._capacity = _unmanaged._capacity;")?;
    indented!(w, [()], r"InteropHelper.interoptopus_vec_destroy(_unmanaged);")?;
    indented!(w, [()], r"_ptr = IntPtr.Zero;")?;
    indented!(w, [()], r"_len = 0;")?;
    indented!(w, [()], r"_capacity = 0;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    i.inline_hint(w, 0)?;
    indented!(w, r"public override string ToString()")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r#"return "{name} {{ ... }}";"#)?;
    indented!(w, r"}}")?;
    w.newline()?;

    Ok(())
}

pub fn write_pattern_vec_interop_helper(i: &Interop, w: &mut IndentWriter, vec: &VecType) -> Result<(), Error> {
    indented!(w, r"public partial class InteropHelper")?;
    indented!(w, r"{{")?;
    for f in i.inventory.functions() {
        let first_param = f.signature().params().first().map(Parameter::the_type).cloned();
        let last_param = f.signature().params().last().map(Parameter::the_type).cloned();
        let name = f.name();

        if name.starts_with("interoptopus_vec_create") {
            if let Some(Type::ReadWritePointer(x)) = last_param {
                if let Type::Pattern(TypePattern::Vec(x)) = x.as_ref() {
                    if x == vec {
                        indented!(w, [()], r#"[LibraryImport(Interop.NativeLib, EntryPoint = "{name}")]"#)?;
                        indented!(w, [()], r"internal static partial long interoptopus_vec_create(IntPtr vec, ulong len, out Unmanaged rval);")?;
                    }
                }
            }
        }

        if name.starts_with("interoptopus_vec_destroy") && first_param == Some(Type::Pattern(TypePattern::Vec(vec.clone()))) {
            indented!(w, [()], r#"[LibraryImport(Interop.NativeLib, EntryPoint = "{name}")]"#)?;
            indented!(w, [()], r"internal static partial long interoptopus_vec_destroy(Unmanaged vec);")?;
        }
    }
    indented!(w, r"}}")?;
    Ok(())
}

pub fn write_pattern_vec_to_managed(i: &Interop, w: &mut IndentWriter, managed: &str) -> Result<(), Error> {
    i.inline_hint(w, 1)?;
    indented!(w, [()], r"public {managed} IntoManaged()")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"var rval = new {managed}();")?;
    indented!(w, [()()], r"rval._len = _len;")?;
    indented!(w, [()()], r"rval._capacity = _capacity;")?;
    indented!(w, [()()], r"rval._ptr = _ptr;")?;
    indented!(w, [()()], r"return rval;")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
    Ok(())
}

pub fn write_pattern_vec_to_unmanaged(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    i.inline_hint(w, 0)?;
    indented!(w, r"public Unmanaged IntoUnmanaged()")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"if (_ptr == IntPtr.Zero) throw new InteropException(); // Don't use for serialization if moved already.")?;
    indented!(w, [()], r"var rval = new Unmanaged();")?;
    indented!(w, [()], r"rval._len = _len;")?;
    indented!(w, [()], r"rval._capacity = _capacity;")?;
    indented!(w, [()], r"rval._ptr = _ptr;")?;
    indented!(w, [()], r"_ptr = IntPtr.Zero; // Mark this instance as moved.")?;
    indented!(w, [()], r"return rval;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    i.inline_hint(w, 0)?;
    indented!(w, r"public Unmanaged AsUnmanaged()")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"if (_ptr == IntPtr.Zero) throw new InteropException(); // Don't use for serialization if moved already.")?;
    indented!(w, [()], r"var rval = new Unmanaged();")?;
    indented!(w, [()], r"rval._len = _len;")?;
    indented!(w, [()], r"rval._capacity = _capacity;")?;
    indented!(w, [()], r"rval._ptr = _ptr;")?;
    indented!(w, [()], r"return rval;")?;
    indented!(w, r"}}")?;

    Ok(())
}
