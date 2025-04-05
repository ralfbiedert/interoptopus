use crate::Interop;
use crate::converter::{is_blittable, slice_t};
use crate::utils::{MoveSemantics, write_common_marshaller};
use interoptopus::backend::IndentWriter;
use interoptopus::pattern::slice::SliceType;
use interoptopus::{Error, indented};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum SliceKind {
    Slice,
    SliceMut,
}

pub fn write_pattern_slice(i: &Interop, w: &mut IndentWriter, slice: &SliceType, kind: SliceKind) -> Result<(), Error> {
    if is_blittable(slice.t()) {
        write_pattern_fast_slice(i, w, slice, kind)
    } else {
        write_pattern_marshalling_slice(i, w, slice)
    }
}

pub fn write_pattern_fast_slice(i: &Interop, w: &mut IndentWriter, slice: &SliceType, kind: SliceKind) -> Result<(), Error> {
    i.debug(w, "write_pattern_fast_slice")?;

    let name = slice.rust_name();
    let inner = slice_t(slice);

    indented!(w, r"public partial class {name}")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"GCHandle _handle;")?;
    indented!(w, [()], r"IntPtr _data;")?;
    indented!(w, [()], r"ulong _len;")?;
    indented!(w, r"}}")?;
    w.newline()?;

    ////
    indented!(w, r"[NativeMarshalling(typeof(MarshallerMeta))]")?;
    indented!(w, r"public partial class {name} : IEnumerable<{inner}>, IDisposable")?;
    indented!(w, r"{{")?;
    w.indent();
    indented!(w, r"public int Count => (int) _len;")?;
    w.newline()?;

    //////
    indented!(w, r"public unsafe ReadOnlySpan<{inner}> ReadOnlySpan")?;
    indented!(w, r"{{")?;
    w.indent();
    i.inline_hint(w, 0)?;
    indented!(w, r"get => new(_data.ToPointer(), (int)_len);")?;
    w.unindent();
    indented!(w, r"}}")?;
    w.newline()?;

    ///////////
    indented!(w, r"public unsafe {inner} this[int i]")?;
    indented!(w, r"{{")?;
    w.indent();
    i.inline_hint(w, 0)?;
    indented!(w, r"get")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"if (i >= Count) throw new IndexOutOfRangeException();")?;
    indented!(w, [()], r"return Unsafe.Read<{inner}>((void*)IntPtr.Add(_data, i * Unsafe.SizeOf<{inner}>()));")?;
    indented!(w, r"}}")?;
    w.newline()?;
    if kind == SliceKind::SliceMut {
        i.inline_hint(w, 0)?;
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
    i.inline_hint(w, 0)?;
    indented!(w, r"{name}() {{ }}")?;
    w.newline()?;
    indented!(w, r"public static {name} From(IntPtr data, ulong len)")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"var rval = new {name}();")?;
    indented!(w, [()], r"rval._data = data;")?;
    indented!(w, [()], r"rval._len = len;")?;
    indented!(w, [()], r"return rval;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    i.inline_hint(w, 0)?;
    indented!(w, r"public static {name} From({inner}[] managed)")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"var rval = new {name}();")?;
    indented!(w, [()], r"rval._handle = GCHandle.Alloc(managed, GCHandleType.Pinned);")?;
    indented!(w, [()], r"rval._data = rval._handle.AddrOfPinnedObject();")?;
    indented!(w, [()], r"rval._len = (ulong) managed.Length;")?;
    indented!(w, [()], r"return rval;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    i.inline_hint(w, 0)?;
    indented!(w, r"public IEnumerator<{inner}> GetEnumerator()")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"for (var i = 0; i < Count; ++i) {{ yield return this[i]; }}")?;
    indented!(w, r"}}")?;
    w.newline()?;
    i.inline_hint(w, 0)?;
    indented!(w, r"IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();")?;
    w.newline()?;
    i.inline_hint(w, 0)?;
    indented!(w, r"public void Dispose()")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"if (_handle is {{ IsAllocated: true }}) {{ _handle.Free(); }}")?;
    indented!(w, r"}}")?;
    w.newline()?;
    write_pattern_slice_to_unmanaged(i, w)?;
    w.newline()?;
    indented!(w, r"[CustomMarshaller(typeof({name}), MarshalMode.Default, typeof(Marshaller))]")?;
    indented!(w, r"private struct MarshallerMeta {{ }}")?;
    w.newline()?;
    indented!(w, r"[StructLayout(LayoutKind.Sequential)]")?;
    indented!(w, r"public struct Unmanaged")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"public IntPtr _data;")?;
    indented!(w, [()], r"public ulong _len;")?;
    w.newline()?;
    i.inline_hint(w, 1)?;
    indented!(w, [()], r"internal {name} ToManaged()")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"return {name}.From(_data, _len);")?;
    indented!(w, [()], r"}}")?;
    indented!(w, r"}}")?;
    w.newline()?;
    w.unindent();
    write_common_marshaller(i, w, name, MoveSemantics::Copy)?;
    indented!(w, r"}}")?;
    Ok(())
}

pub fn write_pattern_marshalling_slice(i: &Interop, w: &mut IndentWriter, slice: &SliceType) -> Result<(), Error> {
    i.debug(w, "write_pattern_marshalling_slice")?;

    let name = slice.rust_name();
    let user_type = slice_t(slice);
    let marshaller_type = slice_t(slice);

    indented!(w, r"public partial class {name}")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"IntPtr _data;")?;
    indented!(w, [()], r"ulong _len;")?;
    indented!(w, r"}}")?;
    w.newline()?;

    ////
    indented!(w, r"[NativeMarshalling(typeof(MarshallerMeta))]")?;
    indented!(w, r"public partial class {name} : IDisposable")?;
    indented!(w, r"{{")?;
    w.indent();
    indented!(w, r"public int Count => (int) _len;")?;
    w.newline()?;

    ///////////
    indented!(w, r"public unsafe {user_type} this[int i]")?;
    indented!(w, r"{{")?;
    w.indent();
    i.inline_hint(w, 0)?;
    indented!(w, r"get")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"if (i >= (int) _len) throw new IndexOutOfRangeException();")?;
    indented!(w, [()], r"if (_data == IntPtr.Zero) {{ throw new Exception(); }}")?;
    indented!(w, [()], r"// TODO")?;
    indented!(w, [()], r"throw new Exception();")?;
    indented!(w, r"}}")?;
    w.unindent();
    indented!(w, r"}}")?;
    w.newline()?;

    ///////////////////
    i.inline_hint(w, 0)?;
    indented!(w, r"{name}() {{ }}")?;
    w.newline()?;

    i.inline_hint(w, 0)?;
    indented!(w, r"public static unsafe {name} From({user_type}[] managed)")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"var rval = new {name}();")?;
    indented!(w, [()], r"var size = sizeof({marshaller_type}.Unmanaged);")?;
    indented!(w, [()], r"rval._data  = Marshal.AllocHGlobal(size * managed.Length);")?;
    indented!(w, [()], r"rval._len = (ulong) managed.Length;")?;
    indented!(w, [()], r"for (var i = 0; i < managed.Length; ++i)")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"var unmanaged = managed[i].IntoUnmanaged();")?;
    indented!(w, [()()], r"var dst = IntPtr.Add(rval._data, i * size);")?;
    indented!(w, [()()], r"Marshal.StructureToPtr(unmanaged, dst, false);")?;
    indented!(w, [()], r"}}")?;
    indented!(w, [()], r"return rval;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    i.inline_hint(w, 0)?;
    indented!(w, r"public void Dispose()")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"if (_data == IntPtr.Zero) return;")?;
    indented!(w, [()], r"Marshal.FreeHGlobal(_data);")?;
    indented!(w, [()], r"_data = IntPtr.Zero;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    write_pattern_slice_to_unmanaged(i, w)?;
    w.newline()?;
    indented!(w, r"[CustomMarshaller(typeof({name}), MarshalMode.Default, typeof(Marshaller))]")?;
    indented!(w, r"private struct MarshallerMeta {{ }}")?;
    w.newline()?;
    indented!(w, r"[StructLayout(LayoutKind.Sequential)]")?;
    indented!(w, r"public struct Unmanaged")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"public IntPtr _data;")?;
    indented!(w, [()], r"public ulong _len;")?;
    w.newline()?;
    write_pattern_slice_to_managed(i, w, name)?;
    indented!(w, r"}}")?;
    w.newline()?;
    w.unindent();
    write_common_marshaller(i, w, name, MoveSemantics::Copy)?;
    indented!(w, r"}}")?;
    Ok(())
}

pub fn write_pattern_slice_to_managed(i: &Interop, w: &mut IndentWriter, managed: &str) -> Result<(), Error> {
    i.inline_hint(w, 1)?;
    indented!(w, [()], r"internal unsafe {managed} ToManaged()")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"var _managed = new {managed}();")?;
    indented!(w, [()()], r"_managed._data = _data;")?;
    indented!(w, [()()], r"_managed._len = _len;")?;
    indented!(w, [()()], r"return _managed;")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;

    Ok(())
}

pub fn write_pattern_slice_to_unmanaged(_: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    indented!(w, r"internal Unmanaged ToUnmanaged()")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"var unmanaged = new Unmanaged();")?;
    indented!(w, [()], r"unmanaged._data = _data;")?;
    indented!(w, [()], r"unmanaged._len = _len; ")?;
    indented!(w, [()], r"return unmanaged;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    Ok(())
}
