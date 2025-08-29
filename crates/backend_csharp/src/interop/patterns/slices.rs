use crate::Interop;
use crate::converter::{is_reusable, slice_t};
use crate::utils::{MoveSemantics, write_common_marshaller};
use interoptopus::pattern::slice::SliceType;
use interoptopus_backend_utils::{Error, IndentWriter, indented};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum SliceKind {
    Slice,
    SliceMut,
}

pub fn write_pattern_slice(i: &Interop, w: &mut IndentWriter, slice: &SliceType, kind: SliceKind) -> Result<(), Error> {
    if is_reusable(slice.t()) {
        write_pattern_fast_slice(i, w, slice, kind)
    } else {
        write_pattern_marshalling_slice(i, w, slice)
    }
}

pub fn write_pattern_fast_slice(i: &Interop, w: &mut IndentWriter, slice: &SliceType, kind: SliceKind) -> Result<(), Error> {
    i.debug(w, "write_pattern_fast_slice")?;

    let name = slice.rust_name();
    let the_type = slice_t(slice);
    let method = if kind == SliceKind::SliceMut { "SliceMut" } else { "Slice" };

    indented!(w, r"public partial class {name}")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"GCHandle _handle;")?;
    indented!(w, [()], r"IntPtr _data;")?;
    indented!(w, [()], r"ulong _len;")?;
    indented!(w, r"}}")?;
    w.newline()?;

    ////
    indented!(w, r"[NativeMarshalling(typeof(MarshallerMeta))]")?;
    indented!(w, r"public partial class {name} : IEnumerable<{the_type}>, IDisposable")?;
    indented!(w, r"{{")?;
    w.indent();
    indented!(w, r"public int Count => (int) _len;")?;
    w.newline()?;

    //////
    indented!(w, r"public unsafe ReadOnlySpan<{the_type}> ReadOnlySpan")?;
    indented!(w, r"{{")?;
    w.indent();
    i.inline_hint(w, 0)?;
    indented!(w, r"get => new(_data.ToPointer(), (int)_len);")?;
    w.unindent();
    indented!(w, r"}}")?;
    w.newline()?;

    ///////////
    indented!(w, r"public unsafe {the_type} this[int i]")?;
    indented!(w, r"{{")?;
    w.indent();
    i.inline_hint(w, 0)?;
    indented!(w, r"get")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"if (i >= Count) throw new IndexOutOfRangeException();")?;
    indented!(w, [()], r"return Unsafe.Read<{the_type}>((void*)IntPtr.Add(_data, i * Unsafe.SizeOf<{the_type}>()));")?;
    indented!(w, r"}}")?;
    w.newline()?;
    if kind == SliceKind::SliceMut {
        i.inline_hint(w, 0)?;
        indented!(w, r"set")?;
        indented!(w, r"{{")?;
        indented!(w, [()], r"if (i >= Count) throw new IndexOutOfRangeException();")?;
        indented!(w, [()], r"Unsafe.Write<{the_type}>((void*)IntPtr.Add(_data, i * Unsafe.SizeOf<{the_type}>()), value);")?;
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
    indented!(w, r"public static {name} From({the_type}[] managed)")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"var rval = new {name}();")?;
    indented!(w, [()], r"rval._handle = GCHandle.Alloc(managed, GCHandleType.Pinned);")?;
    indented!(w, [()], r"rval._data = rval._handle.AddrOfPinnedObject();")?;
    indented!(w, [()], r"rval._len = (ulong) managed.Length;")?;
    indented!(w, [()], r"return rval;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    i.inline_hint(w, 0)?;
    indented!(w, r"public IEnumerator<{the_type}> GetEnumerator()")?;
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
    i.inline_hint(w, 1)?;
    indented!(w, [()], r"internal {name} ToManaged()")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"return {name}.From(_data, _len);")?;
    indented!(w, [()], r"}}")?;
    indented!(w, r"}}")?;
    w.newline()?;
    write_common_marshaller(i, w, name, MoveSemantics::Copy)?;
    w.unindent();
    indented!(w, r"}}")?;
    w.newline()?;

    indented!(w, [()], r"public static class {name}Extensions")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"public static {name} {method}(this {the_type}[] s) {{ return {name}.From(s); }}")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;

    Ok(())
}

pub fn write_pattern_marshalling_slice(i: &Interop, w: &mut IndentWriter, slice: &SliceType) -> Result<(), Error> {
    i.debug(w, "write_pattern_marshalling_slice")?;

    let name = slice.rust_name();
    let the_type = slice_t(slice);
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
    indented!(w, r"public unsafe {the_type} this[int i]")?;
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
    indented!(w, r"public static unsafe {name} From({the_type}[] managed)")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"var rval = new {name}();")?;
    indented!(w, [()], r"var size = sizeof({marshaller_type}.Unmanaged);")?;
    indented!(w, [()], r"rval._data  = Marshal.AllocHGlobal(size * managed.Length);")?;
    indented!(w, [()], r"rval._len = (ulong) managed.Length;")?;
    indented!(w, [()], r"for (var i = 0; i < managed.Length; ++i)")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"var unmanaged = managed[i].AsUnmanaged();")?;
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
    write_common_marshaller(i, w, name, MoveSemantics::Copy)?;
    w.unindent();
    indented!(w, r"}}")?;
    w.newline()?;
    indented!(w, [()], r"public static class {name}Extensions")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"public static {name} Slice(this {the_type}[] s) {{ return {name}.From(s); }}")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
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
