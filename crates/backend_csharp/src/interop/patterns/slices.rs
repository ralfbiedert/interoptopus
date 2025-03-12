use crate::Interop;
use crate::converter::{get_slice_type_argument, is_owned_slice};
use interoptopus::backend::writer::IndentWriter;
use interoptopus::lang::c::CType;
use interoptopus::patterns::TypePattern;
use interoptopus::patterns::slice::SliceType;
use interoptopus::{Error, indented};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum SliceKind {
    Slice,
    SliceMut,
}

pub fn write_pattern_slice(i: &Interop, w: &mut IndentWriter, slice: &SliceType, kind: SliceKind) -> Result<(), Error> {
    if is_owned_slice(slice) {
        write_pattern_marshalling_slice(i, w, slice)
    } else {
        write_pattern_fast_slice(i, w, slice, kind)
    }
}

pub fn write_pattern_fast_slice(i: &Interop, w: &mut IndentWriter, slice: &SliceType, kind: SliceKind) -> Result<(), Error> {
    i.debug(w, "write_pattern_fast_slice")?;

    let name = slice.rust_name();
    let inner = get_slice_type_argument(slice);

    indented!(w, r"public partial struct {}", name)?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"GCHandle _handle;")?;
    indented!(w, [()], r"IntPtr _data;")?;
    indented!(w, [()], r"ulong _len;")?;
    indented!(w, r"}}")?;
    w.newline()?;

    ////
    indented!(w, r"[NativeMarshalling(typeof(MarshallerMeta))]")?;
    indented!(w, r"public partial struct {} : IEnumerable<{}>, IDisposable", name, inner)?;
    indented!(w, r"{{")?;
    w.indent();
    indented!(w, r"public int Count => (int) _len;")?;
    w.newline()?;

    //////
    indented!(w, r"public unsafe ReadOnlySpan<{}> ReadOnlySpan", inner)?;
    indented!(w, r"{{")?;
    w.indent();
    indented!(w, r"get => new(_data.ToPointer(), (int)_len);")?;
    w.unindent();
    indented!(w, r"}}")?;
    w.newline()?;

    ///////////
    indented!(w, r"public unsafe {} this[int i]", inner)?;
    indented!(w, r"{{")?;
    w.indent();
    indented!(w, r"get")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"if (i >= Count) throw new IndexOutOfRangeException();")?;
    indented!(w, [()], r"return Unsafe.Read<{inner}>((void*)IntPtr.Add(_data, i * Unsafe.SizeOf<{inner}>()));")?;
    indented!(w, r"}}")?;
    w.newline()?;
    if kind == SliceKind::SliceMut {
        indented!(w, r"set")?;
        indented!(w, r"{{")?;
        indented!(w, [()], r"if (i >= Count) throw new IndexOutOfRangeException();")?;
        indented!(w, [()], r"Unsafe.Write<{inner}>((void*)IntPtr.Add(_data, i * Unsafe.SizeOf<{inner}>()), value);")?;
        indented!(w, r"}}")?;
    }
    w.unindent();
    indented!(w, r"}}")?;
    w.newline()?;

    ///////////////////
    indented!(w, r"public {}(IntPtr data, ulong len)", name)?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"_data = data;")?;
    indented!(w, [()], r"_len = len;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, r"public {}({}[] managed)", name, inner)?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"_handle = GCHandle.Alloc(managed, GCHandleType.Pinned);")?;
    indented!(w, [()], r"_data = _handle.AddrOfPinnedObject();")?;
    indented!(w, [()], r"_len = (ulong) managed.Length;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, r"public IEnumerator<{}> GetEnumerator()", inner)?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"for (var i = 0; i < Count; ++i) {{ yield return this[i]; }}")?;
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, r"IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();")?;
    w.newline()?;
    indented!(w, r"public void Dispose()")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"if (_handle is {{ IsAllocated: true }}) {{ _handle.Free(); }}")?;
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, r"[CustomMarshaller(typeof({}), MarshalMode.Default, typeof(Marshaller))]", name)?;
    indented!(w, r"private struct MarshallerMeta {{ }}")?;
    w.newline()?;
    indented!(w, r"[StructLayout(LayoutKind.Sequential)]")?;
    indented!(w, r"public struct Unmanaged")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"public IntPtr Data;")?;
    indented!(w, [()], r"public ulong Len;")?;
    w.newline()?;
    indented!(w, [()], r"public {} ToManaged()", name)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"return new {}(Data, Len);", name)?;
    indented!(w, [()], r"}}")?;
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, r"public ref struct Marshaller")?;
    indented!(w, r"{{")?;
    w.indent();
    indented!(w, r"private {} _managed;", name)?;
    indented!(w, r"private Unmanaged _unmanaged;")?;
    w.newline()?;
    indented!(w, r"public void FromManaged({} managed) {{ _managed = managed; }}", name)?;
    indented!(w, r"public void FromUnmanaged(Unmanaged unmanaged) {{ _unmanaged = unmanaged; }}")?;
    w.newline()?;
    indented!(w, r"public Unmanaged ToUnmanaged()")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"_unmanaged = new Unmanaged();")?;
    indented!(w, [()], r"_unmanaged.Data = _managed._data;")?;
    indented!(w, [()], r"_unmanaged.Len = _managed._len;")?;
    indented!(w, [()], r"return _unmanaged;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, r"public unsafe {name} ToManaged()")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"_managed = new {name}();")?;
    indented!(w, [()], r"_managed._data = _unmanaged.Data;")?;
    indented!(w, [()], r"_managed._len = _unmanaged.Len;")?;
    indented!(w, [()], r"return _managed;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, r"public void Free() {{ }}")?;
    w.unindent();
    indented!(w, r"}}")?;
    w.unindent();
    indented!(w, r"}}")?;
    Ok(())
}

pub fn write_pattern_marshalling_slice(i: &Interop, w: &mut IndentWriter, slice: &SliceType) -> Result<(), Error> {
    i.debug(w, "write_pattern_marshalling_slice")?;

    let name = slice.rust_name();
    let user_type = match slice.target_type() {
        CType::Pattern(TypePattern::Utf8String(_)) => "string".to_string(),
        _ => get_slice_type_argument(slice),
    };
    let marshaller_type = get_slice_type_argument(slice);

    indented!(w, r"public partial struct {}", name)?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"{user_type}[] _managed;")?;
    indented!(w, r"}}")?;
    w.newline()?;

    ////
    indented!(w, r"[NativeMarshalling(typeof(MarshallerMeta))]")?;
    indented!(w, r"public partial struct {} : IEnumerable<{}>, IDisposable", name, user_type)?;
    indented!(w, r"{{")?;
    w.indent();
    indented!(w, r"public int Count => _managed?.Length ?? (int) 0;")?;
    w.newline()?;

    ///////////
    indented!(w, r"public unsafe {} this[int i]", user_type)?;
    indented!(w, r"{{")?;
    w.indent();
    indented!(w, r"get")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"if (i >= Count) throw new IndexOutOfRangeException();")?;
    indented!(w, [()], r"if (_managed is not null) {{ return _managed[i]; }}")?;
    indented!(w, [()], r"return default;")?;
    indented!(w, r"}}")?;
    w.unindent();
    indented!(w, r"}}")?;
    w.newline()?;

    ///////////////////
    indented!(w, r"public {name}({user_type}[] managed)")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"_managed = managed;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, r"public IEnumerator<{user_type}> GetEnumerator()")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"for (var i = 0; i < Count; ++i) {{ yield return this[i]; }}")?;
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, r"IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();")?;
    w.newline()?;
    indented!(w, r"public void Dispose() {{ }}")?;
    w.newline()?;
    indented!(w, r"[CustomMarshaller(typeof({name}), MarshalMode.Default, typeof(Marshaller))]")?;
    indented!(w, r"private struct MarshallerMeta {{ }}")?;
    w.newline()?;
    indented!(w, r"[StructLayout(LayoutKind.Sequential)]")?;
    indented!(w, r"public struct Unmanaged")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"public IntPtr Data;")?;
    indented!(w, [()], r"public ulong Len;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, r"public ref struct Marshaller")?;
    indented!(w, r"{{")?;
    w.indent();
    indented!(w, r"private {} _managed;", name)?;
    indented!(w, r"private Unmanaged _unmanaged;")?;
    w.newline()?;
    indented!(w, r"public void FromManaged({} managed) {{ _managed = managed; }}", name)?;
    indented!(w, r"public void FromUnmanaged(Unmanaged unmanaged) {{ _unmanaged = unmanaged; }}")?;
    w.newline()?;
    indented!(w, r"public unsafe Unmanaged ToUnmanaged()")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"var size = sizeof({marshaller_type}.Unmanaged);")?;
    indented!(w, [()], r"_unmanaged = new Unmanaged();")?;
    indented!(w, [()], r"_unmanaged.Data = Marshal.AllocHGlobal(size * _managed.Count);")?;
    indented!(w, [()], r"_unmanaged.Len = (ulong) _managed.Count;")?;
    indented!(w, [()], r"for (var i = 0; i < _managed.Count; ++i)")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"var _marshaller = new {marshaller_type}.Marshaller();")?;
    indented!(w, [()()], r"_marshaller.FromManaged(new {marshaller_type}(_managed._managed[i]));")?;
    indented!(w, [()()], r"var unmanaged = _marshaller.ToUnmanaged();")?;
    indented!(w, [()()], r"var dst = IntPtr.Add(_unmanaged.Data, i * size);")?;
    indented!(w, [()()], r"Marshal.StructureToPtr(unmanaged, dst, false);")?;
    indented!(w, [()], r"}}")?;
    indented!(w, [()], r"return _unmanaged;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, r"public unsafe {name} ToManaged()")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"_managed = new {name}();")?;
    // indented!(w, [()], r"_managed._data = _unmanaged.Data;")?;
    // indented!(w, [()], r"_managed._len = _unmanaged.Len;")?;
    indented!(w, [()], r"return _managed;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, r"public void Free() {{ Marshal.FreeHGlobal(_unmanaged.Data); }}")?;
    w.unindent();
    indented!(w, r"}}")?;
    w.unindent();
    indented!(w, r"}}")?;
    Ok(())
}
