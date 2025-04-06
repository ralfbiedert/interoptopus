use crate::Interop;
use interoptopus::backend::IndentWriter;
use interoptopus::{Error, indented};

pub fn write_async_helper(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
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
