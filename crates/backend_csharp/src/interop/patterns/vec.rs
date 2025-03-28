use crate::Interop;
use crate::converter::{get_vec_type_argument, is_owned_vec, to_typespecifier_in_param};
use interoptopus::backend::IndentWriter;
use interoptopus::lang::{Parameter, Type};
use interoptopus::pattern::TypePattern;
use interoptopus::pattern::vec::VecType;
use interoptopus::{Error, indented};

pub fn write_pattern_vec(i: &Interop, w: &mut IndentWriter, vec: &VecType) -> Result<(), Error> {
    i.debug(w, "write_pattern_vec")?;
    if is_owned_vec(vec) {
        write_pattern_marshalling_vec(i, w, vec)
    } else {
        write_pattern_fast_vec(i, w, vec)
    }
}

pub fn write_pattern_fast_vec(i: &Interop, w: &mut IndentWriter, vec: &VecType) -> Result<(), Error> {
    i.debug(w, "write_pattern_fast_vec")?;

    let name = vec.rust_name();
    let inner = to_typespecifier_in_param(vec.t());

    write_pattern_vec_struct(i, w, vec)?;

    ////
    indented!(w, r"[NativeMarshalling(typeof(MarshallerMeta))]")?;
    indented!(w, r"public partial class {} : IDisposable", name)?;
    indented!(w, r"{{")?;
    w.indent();

    indented!(w, r"/// Allocates an empty vec on the native side.")?;
    i.inline_hint(w, 0)?;
    indented!(w, r"public {name}() {{ /* TODO - create empty vec */ }}")?;
    w.newline()?;

    indented!(w, r"// An internal helper to create an empty object.")?;
    i.inline_hint(w, 0)?;
    indented!(w, r"private {name}(bool _) {{ }}")?;
    w.newline()?;

    i.inline_hint(w, 0)?;
    indented!(w, r"public unsafe {name}(Span<{inner}> _data)")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"fixed (void* _data_ptr = _data)")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"InteropHelper.interoptopus_vec_create((IntPtr) _data_ptr, (ulong)_data.Length, out var _out);")?;
    indented!(w, [()()], r"_len = _out._len;")?;
    indented!(w, [()()], r"_capacity = _out._capacity;")?;
    indented!(w, [()()], r"_ptr = _out._ptr;")?;
    indented!(w, [()], r"}}")?;
    indented!(w, r"}}")?;
    w.newline()?;

    indented!(w, r"public int Count")?;
    indented!(w, r"{{")?;
    i.inline_hint(w, 1)?;
    indented!(w, [()], r"get {{ if (_ptr == IntPtr.Zero) {{ throw new InteropException(); }} else {{ return (int) _len; }} }}")?;
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, r"public unsafe {} this[int i]", inner)?;
    indented!(w, r"{{")?;
    w.indent();
    i.inline_hint(w, 0)?;
    indented!(w, r"get")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"if (i >= Count) throw new IndexOutOfRangeException();")?;
    indented!(w, [()], r"if (_ptr == IntPtr.Zero) throw new InteropException();")?;
    indented!(w, [()], r"return Marshal.PtrToStructure<{}>(new IntPtr(_ptr.ToInt64() + i * sizeof({})));", inner, inner)?;
    indented!(w, r"}}")?;
    w.unindent();
    indented!(w, r"}}")?;

    write_pattern_vec_to_unmanaged(i, w)?;
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
    w.newline()?;
    indented!(w, r"public ref struct Marshaller")?;
    indented!(w, r"{{")?;
    w.indent();
    indented!(w, r"private {} _managed;", name)?;
    indented!(w, r"private Unmanaged _unmanaged;")?;
    w.newline()?;
    i.inline_hint(w, 0)?;
    indented!(w, r"public Marshaller(Unmanaged unmanaged) {{ _unmanaged = unmanaged; }}")?;
    i.inline_hint(w, 0)?;
    indented!(w, r"public Marshaller({name} managed) {{ _managed = managed; }}")?;
    i.inline_hint(w, 0)?;
    indented!(w, r"public void FromManaged({} managed) {{ _managed = managed; }}", name)?;
    i.inline_hint(w, 0)?;
    indented!(w, r"public void FromUnmanaged(Unmanaged unmanaged) {{ _unmanaged = unmanaged; }}")?;
    w.newline()?;
    i.inline_hint(w, 0)?;
    indented!(w, r"public Unmanaged ToUnmanaged()")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"if (_managed._ptr == IntPtr.Zero) throw new InteropException(); // Don't use for serialization if moved already.")?;
    indented!(w, [()], r"_unmanaged = new Unmanaged();")?;
    indented!(w, [()], r"_unmanaged._len = _managed._len;")?;
    indented!(w, [()], r"_unmanaged._capacity = _managed._capacity;")?;
    indented!(w, [()], r"_unmanaged._ptr = _managed._ptr;")?;
    indented!(w, [()], r"_managed._ptr = IntPtr.Zero; // Mark this instance as moved.")?;
    indented!(w, [()], r"return _unmanaged;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    i.inline_hint(w, 0)?;
    indented!(w, r"public unsafe {name} ToManaged()")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"_managed = new {name}(true);")?;
    indented!(w, [()], r"_managed._len = _unmanaged._len;")?;
    indented!(w, [()], r"_managed._capacity = _unmanaged._capacity;")?;
    indented!(w, [()], r"_managed._ptr = _unmanaged._ptr;")?;
    indented!(w, [()], r"return _managed;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    i.inline_hint(w, 0)?;
    indented!(w, r"public void Free() {{ }}")?;
    w.unindent();
    indented!(w, r"}}")?;
    w.newline()?;
    write_pattern_vec_dispose(i, w, vec)?;
    w.unindent();
    indented!(w, r"}}")?;
    Ok(())
}

pub fn write_pattern_marshalling_vec(i: &Interop, w: &mut IndentWriter, vec: &VecType) -> Result<(), Error> {
    i.debug(w, "write_pattern_fast_vec")?;

    let name = vec.rust_name();
    let user_type = match vec.t() {
        Type::Pattern(TypePattern::Utf8String(_)) => "string".to_string(),
        _ => get_vec_type_argument(vec),
    };
    let marshaller_type = get_vec_type_argument(vec);

    write_pattern_vec_struct(i, w, vec)?;

    ////
    indented!(w, r"[NativeMarshalling(typeof(MarshallerMeta))]")?;
    indented!(w, r"public partial class {} : IDisposable", name)?;
    indented!(w, r"{{")?;
    w.indent();

    indented!(w, r"/// Allocates an empty vec on the native side.")?;
    i.inline_hint(w, 0)?;
    indented!(w, r"public {name}() {{ /* TODO - create empty vec */ }}")?;
    w.newline()?;

    indented!(w, r"// An internal helper to create an empty object.")?;
    i.inline_hint(w, 0)?;
    indented!(w, r"private {name}(bool _) {{ }}")?;
    w.newline()?;

    i.inline_hint(w, 0)?;
    indented!(w, r"public unsafe {name}(Span<{user_type}> _data)")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"var _temp = new {marshaller_type}.Unmanaged[_data.Length];")?;
    indented!(w, [()], r"for (var i = 0; i < _data.Length; ++i)")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"_temp[i] = new {marshaller_type}(_data[i]).ToUnmanaged();")?;
    indented!(w, [()], r"}}")?;
    indented!(w, [()], r"fixed (void* _data_ptr = _temp)")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"InteropHelper.interoptopus_vec_create((IntPtr) _data_ptr, (ulong)_data.Length, out var _out);")?;
    indented!(w, [()()], r"_len = _out._len;")?;
    indented!(w, [()()], r"_capacity = _out._capacity;")?;
    indented!(w, [()()], r"_ptr = _out._ptr;")?;
    indented!(w, [()], r"}}")?;
    indented!(w, r"}}")?;
    w.newline()?;

    indented!(w, r"public int Count")?;
    indented!(w, r"{{")?;
    i.inline_hint(w, 1)?;
    indented!(w, [()], r"get {{ if (_ptr == IntPtr.Zero) {{ throw new InteropException(); }} else {{ return (int) _len; }} }}")?;
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, r"public unsafe {user_type} this[int i]")?;
    indented!(w, r"{{")?;
    w.indent();
    i.inline_hint(w, 0)?;
    indented!(w, r"get")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"if (i >= Count) throw new IndexOutOfRangeException();")?;
    indented!(w, [()], r"if (_ptr == IntPtr.Zero) throw new InteropException();")?;
    indented!(w, [()], r"var _element = Marshal.PtrToStructure<{marshaller_type}.Unmanaged>(new IntPtr(_ptr.ToInt64() + i * sizeof({marshaller_type}.Unmanaged)));")?;
    indented!(w, [()], r"return _element.ToManaged();")?;
    indented!(w, r"}}")?;
    w.unindent();
    indented!(w, r"}}")?;

    write_pattern_vec_to_unmanaged(i, w)?;
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
    w.newline()?;
    indented!(w, r"public ref struct Marshaller")?;
    indented!(w, r"{{")?;
    w.indent();
    indented!(w, r"private {} _managed;", name)?;
    indented!(w, r"private Unmanaged _unmanaged;")?;
    w.newline()?;
    i.inline_hint(w, 0)?;
    indented!(w, r"public Marshaller(Unmanaged unmanaged) {{ _unmanaged = unmanaged; }}")?;
    i.inline_hint(w, 0)?;
    indented!(w, r"public Marshaller({name} managed) {{ _managed = managed; }}")?;
    i.inline_hint(w, 0)?;
    indented!(w, r"public void FromManaged({} managed) {{ _managed = managed; }}", name)?;
    i.inline_hint(w, 0)?;
    indented!(w, r"public void FromUnmanaged(Unmanaged unmanaged) {{ _unmanaged = unmanaged; }}")?;
    w.newline()?;
    i.inline_hint(w, 0)?;
    indented!(w, r"public Unmanaged ToUnmanaged()")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"if (_managed._ptr == IntPtr.Zero) throw new InteropException(); // Don't use for serialization if moved already.")?;
    indented!(w, [()], r"_unmanaged = new Unmanaged();")?;
    indented!(w, [()], r"_unmanaged._len = _managed._len;")?;
    indented!(w, [()], r"_unmanaged._capacity = _managed._capacity;")?;
    indented!(w, [()], r"_unmanaged._ptr = _managed._ptr;")?;
    indented!(w, [()], r"_managed._ptr = IntPtr.Zero; // Mark this instance as moved.")?;
    indented!(w, [()], r"return _unmanaged;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    i.inline_hint(w, 0)?;
    indented!(w, r"public unsafe {name} ToManaged()")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"_managed = new {name}(true);")?;
    indented!(w, [()], r"_managed._len = _unmanaged._len;")?;
    indented!(w, [()], r"_managed._capacity = _unmanaged._capacity;")?;
    indented!(w, [()], r"_managed._ptr = _unmanaged._ptr;")?;
    indented!(w, [()], r"return _managed;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    i.inline_hint(w, 0)?;
    indented!(w, r"public void Free() {{ }}")?;
    w.unindent();
    indented!(w, r"}}")?;
    w.newline()?;
    write_pattern_vec_dispose(i, w, vec)?;
    w.unindent();
    indented!(w, r"}}")?;
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

pub fn write_pattern_vec_dispose(i: &Interop, w: &mut IndentWriter, _: &VecType) -> Result<(), Error> {
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
                        indented!(w, [()], r"public static partial long interoptopus_vec_create(IntPtr vec, ulong len, out Unmanaged rval);")?;
                    }
                }
            }
        }

        if name.starts_with("interoptopus_vec_destroy") && first_param == Some(Type::Pattern(TypePattern::Vec(vec.clone()))) {
            indented!(w, [()], r#"[LibraryImport(Interop.NativeLib, EntryPoint = "{name}")]"#)?;
            indented!(w, [()], r"public static partial long interoptopus_vec_destroy(Unmanaged vec);")?;
        }
    }
    indented!(w, r"}}")?;
    Ok(())
}

pub fn write_pattern_vec_to_managed(_: &Interop, w: &mut IndentWriter, managed: &str) -> Result<(), Error> {
    indented!(w, [()], r"public {managed} ToManaged()")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"var marshaller = new Marshaller(this);")?;
    indented!(w, [()()], r"try {{ return marshaller.ToManaged(); }}")?;
    indented!(w, [()()], r"finally {{ marshaller.Free(); }}")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
    Ok(())
}

pub fn write_pattern_vec_to_unmanaged(_: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    indented!(w, r"public Unmanaged ToUnmanaged()")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"var marshaller = new Marshaller(this);")?;
    indented!(w, [()], r"try {{ return marshaller.ToUnmanaged(); }}")?;
    indented!(w, [()], r"finally {{ marshaller.Free(); }}")?;
    indented!(w, r"}}")?;
    w.newline()?;
    Ok(())
}
