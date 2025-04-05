use crate::Interop;
use interoptopus::backend::IndentWriter;
use interoptopus::{Error, indented};

pub fn write_builtins(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    if i.write_types.write_interoptopus_globals() {
        indented!(w, r"public class InteropException: Exception")?;
        indented!(w, r"{{")?;
        w.newline()?;
        indented!(w, [()], r"public InteropException(): base()")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()], r"}}")?;
        indented!(w, r"}}")?;
        w.newline()?;
    }

    if i.write_types.write_interoptopus_globals() {
        // Emit delegates
        indented!(w, r"[UnmanagedFunctionPointer(CallingConvention.Cdecl)]")?;
        indented!(w, r"public delegate void AsyncHelperNative(IntPtr data, IntPtr callback_data);")?;
        indented!(w, r"public delegate void AsyncHelperDelegate(IntPtr data);")?;
        w.newline()?;

        indented!(w, r"public partial struct AsyncHelper")?;
        indented!(w, r"{{")?;
        indented!(w, [()], r"private AsyncHelperDelegate _managed;")?;
        indented!(w, [()], r"private AsyncHelperNative _native;")?;
        indented!(w, [()], r"private IntPtr _ptr;")?;
        indented!(w, r"}}")?;
        w.newline()?;

        // --------------------------------

        // Emit main struct
        indented!(w, r"[NativeMarshalling(typeof(MarshallerMeta))]")?;
        indented!(w, r"public partial struct AsyncHelper : IDisposable")?;
        indented!(w, r"{{")?;

        // Constructors
        indented!(w, [()], r"public AsyncHelper() {{ }}")?;
        w.newline()?;
        i.inline_hint(w, 1)?;
        indented!(w, [()], r"public AsyncHelper(AsyncHelperDelegate managed)")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"_managed = managed;")?;
        indented!(w, [()()], r"_native = Call;")?;
        indented!(w, [()()], r"_ptr = Marshal.GetFunctionPointerForDelegate(_native);")?;
        indented!(w, [()], r"}}")?;
        w.newline()?;

        // Methods
        i.inline_hint(w, 1)?;
        indented!(w, [()], r"void Call(IntPtr data, IntPtr _)")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"_managed(data);")?;
        indented!(w, [()], r"}}")?;
        w.newline()?;

        i.inline_hint(w, 1)?;
        indented!(w, [()], r"public void Dispose()")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"if (_ptr == IntPtr.Zero) return;")?;
        indented!(w, [()()], r"Marshal.FreeHGlobal(_ptr);")?;
        indented!(w, [()()], r"_ptr = IntPtr.Zero;")?;
        indented!(w, [()], r"}}")?;
        w.newline()?;

        // Marshaller metadata
        indented!(w, [()], r"[CustomMarshaller(typeof(AsyncHelper), MarshalMode.Default, typeof(Marshaller))]")?;
        indented!(w, [()], r"private struct MarshallerMeta {{ }}")?;
        w.newline()?;

        // Unmanaged struct
        indented!(w, [()], r"[StructLayout(LayoutKind.Sequential)]")?;
        indented!(w, [()], r"public struct Unmanaged")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"internal IntPtr Callback;")?;
        indented!(w, [()()], r"internal IntPtr Data;")?;
        indented!(w, [()], r"}}")?;
        w.newline()?;

        // Marshaller struct
        indented!(w, [()], r"public ref struct Marshaller")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"private AsyncHelper _managed;")?;
        indented!(w, [()()], r"private Unmanaged _unmanaged;")?;
        w.newline()?;

        i.inline_hint(w, 2)?;
        indented!(w, [()()], r"public void FromManaged(AsyncHelper managed) {{ _managed = managed; }}")?;
        i.inline_hint(w, 2)?;
        indented!(w, [()()], r"public void FromUnmanaged(Unmanaged unmanaged) {{ _unmanaged = unmanaged; }}")?;
        w.newline()?;

        i.inline_hint(w, 2)?;
        indented!(w, [()()], r"public Unmanaged ToUnmanaged()")?;
        indented!(w, [()()], r"{{")?;
        indented!(w, [()()()], r"_unmanaged = new Unmanaged();")?;
        indented!(w, [()()()], r"_unmanaged.Callback = _managed._ptr;")?;
        indented!(w, [()()()], r"_unmanaged.Data = IntPtr.Zero;")?;
        indented!(w, [()()()], r"return _unmanaged;")?;
        indented!(w, [()()], r"}}")?;
        w.newline()?;

        i.inline_hint(w, 2)?;
        indented!(w, [()()], r"public AsyncHelper ToManaged()")?;
        indented!(w, [()()], r"{{")?;
        indented!(w, [()()()], r"_managed = new AsyncHelper();")?;
        indented!(w, [()()()], r"_managed._ptr = _unmanaged.Callback;")?;
        indented!(w, [()()()], r"return _managed;")?;
        indented!(w, [()()], r"}}")?;
        w.newline()?;

        i.inline_hint(w, 2)?;
        indented!(w, [()()], r"public void Free() {{ }}")?;

        indented!(w, [()], r"}}")?;
        indented!(w, r"}}")?;

        // --------------------------------

        indented!(w, r"public partial class Utf8String")?;
        indented!(w, r"{{")?;
        indented!(w, [()], r"IntPtr _ptr;")?;
        indented!(w, [()], r"ulong _len;")?;
        indented!(w, [()], r"ulong _capacity;")?;
        indented!(w, r"}}")?;
        w.newline()?;
        indented!(w, r"[NativeMarshalling(typeof(MarshallerMeta))]")?;
        indented!(w, r"public partial class Utf8String: IDisposable")?;
        indented!(w, r"{{")?;
        indented!(w, [()], r"private Utf8String() {{ }}")?;
        w.newline()?;
        i.inline_hint(w, 1)?;
        indented!(w, [()], r"public static unsafe Utf8String From(string s)")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"var rval = new Utf8String();")?;
        indented!(w, [()()], r"var source = s.AsSpan();")?;
        indented!(w, [()()], r"Span<byte> utf8Bytes = stackalloc byte[Encoding.UTF8.GetByteCount(source)];")?;
        indented!(w, [()()], r"var len = Encoding.UTF8.GetBytes(source, utf8Bytes);")?;
        w.newline()?;
        indented!(w, [()()], r"fixed (byte* p = utf8Bytes)")?;
        indented!(w, [()()], r"{{")?;
        indented!(w, [()()()], r"InteropHelper.interoptopus_string_create((IntPtr) p, (ulong)len, out var native);")?;
        indented!(w, [()()()], r"rval._ptr = native._ptr;")?;
        indented!(w, [()()()], r"rval._len = native._len;")?;
        indented!(w, [()()()], r"rval._capacity = native._capacity;")?;
        indented!(w, [()()], r"}}")?;
        w.newline()?;
        indented!(w, [()()], r"return rval;")?;
        indented!(w, [()], r"}}")?;
        w.newline()?;
        indented!(w, [()], r"public unsafe string String")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"get")?;
        indented!(w, [()()], r"{{")?;
        indented!(w, [()()()], r"var span = new ReadOnlySpan<byte>((byte*) _ptr, (int)_len);")?;
        indented!(w, [()()()], r"var s = Encoding.UTF8.GetString(span);")?;
        indented!(w, [()()()], r"return s;")?;
        indented!(w, [()()], r"}}")?;
        indented!(w, [()], r"}}")?;
        w.newline()?;
        i.inline_hint(w, 1)?;
        indented!(w, [()], r"public void Dispose()")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"var _unmanaged = new Unmanaged();")?;
        indented!(w, [()()], r"_unmanaged._ptr = _ptr;")?;
        indented!(w, [()()], r"_unmanaged._len = _len;")?;
        indented!(w, [()()], r"_unmanaged._capacity = _capacity;")?;
        indented!(w, [()()], r"InteropHelper.interoptopus_string_destroy(_unmanaged);")?;
        indented!(w, [()], r"}}")?;
        w.newline()?;
        i.inline_hint(w, 1)?;
        indented!(w, [()], r"public Unmanaged IntoUnmanaged()")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"var _unmanaged = new Unmanaged();")?;
        indented!(w, [()()], r"_unmanaged._ptr = _ptr;")?;
        indented!(w, [()()], r"_unmanaged._len = _len;")?;
        indented!(w, [()()], r"_unmanaged._capacity = _capacity;")?;
        indented!(w, [()()], r"_ptr = IntPtr.Zero;")?;
        indented!(w, [()()], r"return _unmanaged;")?;
        indented!(w, [()], r"}}")?;
        w.newline()?;
        i.inline_hint(w, 1)?;
        indented!(w, [()], r"public Unmanaged AsUnmanaged()")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"var _unmanaged = new Unmanaged();")?;
        indented!(w, [()()], r"_unmanaged._ptr = _ptr;")?;
        indented!(w, [()()], r"_unmanaged._len = _len;")?;
        indented!(w, [()()], r"_unmanaged._capacity = _capacity;")?;
        indented!(w, [()()], r"return _unmanaged;")?;
        indented!(w, [()], r"}}")?;
        w.newline()?;
        indented!(w, [()], r"/// A highly dangerous 'use once type' that has ownership semantics!")?;
        indented!(w, [()], r"/// Once passed over an FFI boundary 'the other side' is meant to own")?;
        indented!(w, [()], r"/// (and free) it. Rust handles that fine, but if in C# you put this")?;
        indented!(w, [()], r"/// in a struct and then call Rust multiple times with that struct")?;
        indented!(w, [()], r"/// you'll free the same pointer multiple times, and get UB!")?;
        indented!(w, [()], r"[StructLayout(LayoutKind.Sequential)]")?;
        indented!(w, [()], r"public unsafe struct Unmanaged")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"public IntPtr _ptr;")?;
        indented!(w, [()()], r"public ulong _len;")?;
        indented!(w, [()()], r"public ulong _capacity;")?;
        w.newline()?;
        i.inline_hint(w, 2)?;
        indented!(w, [()()], r"public Utf8String IntoManaged()")?;
        indented!(w, [()()], r"{{")?;
        indented!(w, [()()], r"var _managed = new Utf8String();")?;
        indented!(w, [()()], r"_managed._ptr = _ptr;")?;
        indented!(w, [()()], r"_managed._len = _len;")?;
        indented!(w, [()()], r"_managed._capacity = _capacity;")?;
        indented!(w, [()()], r"return _managed;")?;
        indented!(w, [()()], r"}}")?;
        w.newline()?;
        indented!(w, [()], r"}}")?;
        w.newline()?;

        indented!(w, [()], r"public partial class InteropHelper")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r#"[LibraryImport(Interop.NativeLib, EntryPoint = "interoptopus_string_create")]"#)?;
        i.inline_hint(w, 2)?;
        indented!(w, [()()], r"public static partial long interoptopus_string_create(IntPtr utf8, ulong len, out Unmanaged rval);")?;
        w.newline()?;
        indented!(w, [()()], r#"[LibraryImport(Interop.NativeLib, EntryPoint = "interoptopus_string_destroy")]"#)?;
        i.inline_hint(w, 2)?;
        indented!(w, [()()], r"public static partial long interoptopus_string_destroy(Unmanaged utf8);")?;
        indented!(w, [()], r"}}")?;
        w.newline()?;

        indented!(w, [()], r"[CustomMarshaller(typeof(Utf8String), MarshalMode.Default, typeof(Marshaller))]")?;
        indented!(w, [()], r"private struct MarshallerMeta {{ }}")?;
        w.newline()?;

        indented!(w, [()], r"public ref struct Marshaller")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"private Utf8String _managed; // Used when converting managed -> unmanaged")?;
        indented!(w, [()()], r"private Unmanaged _unmanaged; // Used when converting unmanaged -> managed")?;
        w.newline()?;

        i.inline_hint(w, 2)?;
        indented!(w, [()()], r"public Marshaller(Utf8String managed) {{ _managed = managed; }}")?;
        i.inline_hint(w, 2)?;
        indented!(w, [()()], r"public Marshaller(Unmanaged unmanaged) {{ _unmanaged = unmanaged; }}")?;
        w.newline()?;

        i.inline_hint(w, 2)?;
        indented!(w, [()()], r"public void FromManaged(Utf8String managed) {{ _managed = managed; }}")?;
        i.inline_hint(w, 2)?;
        indented!(w, [()()], r"public void FromUnmanaged(Unmanaged unmanaged) {{ _unmanaged = unmanaged; }}")?;
        w.newline()?;

        i.inline_hint(w, 2)?;
        indented!(w, [()()], r"public unsafe Unmanaged ToUnmanaged()")?;
        indented!(w, [()()], r"{{")?;
        indented!(w, [()()()], r"return _managed.IntoUnmanaged();")?;
        indented!(w, [()()], r"}}")?;
        w.newline()?;

        i.inline_hint(w, 2)?;
        indented!(w, [()()], r"public unsafe Utf8String ToManaged()")?;
        indented!(w, [()()], r"{{")?;
        indented!(w, [()()()], r"return _unmanaged.IntoManaged();")?;
        indented!(w, [()()], r"}}")?;
        w.newline()?;

        i.inline_hint(w, 2)?;
        indented!(w, [()()], r"public void Free() {{ }}")?;
        indented!(w, [()], r"}}")?;
        indented!(w, r"}}")?;
        w.newline()?;

        // --------------------------------

        indented!(w, [()], r"public static class StringExtensions")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"public static Utf8String Utf8(this string s) {{ return Utf8String.From(s); }}")?;
        indented!(w, [()], r"}}")?;
        w.newline()?;

        // --------------------------------

        indented!(w, [()], r"public delegate void AsyncCallbackCommon(IntPtr data, IntPtr callback_data);")?;
        w.newline()?;
        indented!(w, [()], r"[StructLayout(LayoutKind.Sequential)]")?;
        indented!(w, [()], r"public partial struct AsyncCallbackCommonNative")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"internal IntPtr _ptr;")?;
        indented!(w, [()()], r"internal IntPtr _ts;")?;
        indented!(w, [()], r"}}")?;
    }

    Ok(())
}
