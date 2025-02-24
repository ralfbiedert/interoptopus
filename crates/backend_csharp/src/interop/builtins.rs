use crate::Interop;
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error};

pub fn write_builtins(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    if i.write_types.write_interoptopus_globals() && i.has_ffi_error(i.inventory.functions()) {
        let error_text = &i.error_text;

        indented!(w, r"public class InteropException<T> : Exception")?;
        indented!(w, r"{{")?;
        indented!(w, [()], r"public T Error {{ get; private set; }}")?;
        w.newline()?;
        indented!(w, [()], r#"public InteropException(T error): base($"{error_text}")"#)?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"Error = error;")?;
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

        // Emit main struct
        indented!(w, r"[NativeMarshalling(typeof(MarshallerMeta))]")?;
        indented!(w, r"public struct AsyncHelper : IDisposable")?;
        indented!(w, r"{{")?;

        // Fields
        indented!(w, [()], r"private AsyncHelperDelegate _callbackUser;")?;
        indented!(w, [()], r"private IntPtr _callbackNative;")?;
        w.newline()?;

        // Constructors
        indented!(w, [()], r"public AsyncHelper() {{ }}")?;
        w.newline()?;
        indented!(w, [()], r"public AsyncHelper(AsyncHelperDelegate callbackUser)")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"_callbackUser = callbackUser;")?;
        indented!(w, [()()], r"_callbackNative = Marshal.GetFunctionPointerForDelegate(new AsyncHelperNative(Call));")?;
        indented!(w, [()], r"}}")?;
        w.newline()?;

        // Methods
        indented!(w, [()], r"public void Call(IntPtr data, IntPtr callback_data)")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"_callbackUser(data);")?;
        indented!(w, [()], r"}}")?;
        w.newline()?;

        indented!(w, [()], r"public void Dispose()")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"if (_callbackNative == IntPtr.Zero) return;")?;
        indented!(w, [()()], r"Marshal.FreeHGlobal(_callbackNative);")?;
        indented!(w, [()()], r"_callbackNative = IntPtr.Zero;")?;
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
        indented!(w, [()()], r"private AsyncHelper managed;")?;
        indented!(w, [()()], r"private Unmanaged native;")?;
        indented!(w, [()()], r"private Unmanaged sourceNative;")?;
        indented!(w, [()()], r"private GCHandle? pinned;")?;
        w.newline()?;

        indented!(w, [()()], r"public void FromManaged(AsyncHelper managed)")?;
        indented!(w, [()()], r"{{")?;
        indented!(w, [()()()], r"this.managed = managed;")?;
        indented!(w, [()()], r"}}")?;
        w.newline()?;

        indented!(w, [()()], r"public Unmanaged ToUnmanaged()")?;
        indented!(w, [()()], r"{{")?;
        indented!(w, [()()()], r"return new Unmanaged")?;
        indented!(w, [()()()], r"{{")?;
        indented!(w, [()()()()], r"Callback = managed._callbackNative,")?;
        indented!(w, [()()()()], r"Data = IntPtr.Zero")?;
        indented!(w, [()()()], r"}};")?;
        indented!(w, [()()], r"}}")?;
        w.newline()?;

        indented!(w, [()()], r"public void FromUnmanaged(Unmanaged unmanaged)")?;
        indented!(w, [()()], r"{{")?;
        indented!(w, [()()()], r"sourceNative = unmanaged;")?;
        indented!(w, [()()], r"}}")?;
        w.newline()?;

        indented!(w, [()()], r"public AsyncHelper ToManaged()")?;
        indented!(w, [()()], r"{{")?;
        indented!(w, [()()()], r"return new AsyncHelper")?;
        indented!(w, [()()()], r"{{")?;
        indented!(w, [()()()()], r"_callbackNative = sourceNative.Callback,")?;
        indented!(w, [()()()], r"}};")?;
        indented!(w, [()()], r"}}")?;
        w.newline()?;

        indented!(w, [()()], r"public void Free() {{ }}")?;

        indented!(w, [()], r"}}")?;
        indented!(w, r"}}")?;
    }

    Ok(())
}
