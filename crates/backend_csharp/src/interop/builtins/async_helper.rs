use crate::Interop;
use interoptopus::backend::IndentWriter;
use interoptopus::{Error, indented};

pub fn write_async_helper(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    if i.write_types.write_interoptopus_globals() {
        indented!(w, r"file sealed class RecyclableTaskCompletionSource<T> : IValueTaskSource<T>")?;
        indented!(w, r"{{")?;
        indented!(w, [()], r"private static readonly ConcurrentDictionary<int, RecyclableTaskCompletionSource<T>> Registered = new();")?;
        indented!(w, [()], r"// ReSharper disable once StaticMemberInGenericType")?;
        indented!(w, [()], r"private static int _nextId;")?;
        w.newline()?;
        indented!(w, [()], r"private static readonly ConcurrentQueue<RecyclableTaskCompletionSource<T>> Pool = new();")?;
        w.newline()?;
        indented!(w, [()], r"[MethodImpl(MethodImplOptions.AggressiveInlining)]")?;
        indented!(w, [()], r"public static RecyclableTaskCompletionSource<T> GetById(int id) => Registered[id];")?;
        w.newline()?;
        indented!(w, [()], r"public int Id {{ [MethodImpl(MethodImplOptions.AggressiveInlining)] get; }}")?;
        w.newline()?;
        indented!(w, [()], r"[MethodImpl(MethodImplOptions.AggressiveOptimization)]")?;
        indented!(w, [()], r"public static RecyclableTaskCompletionSource<T> Get()")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()()], r"if (Pool.TryDequeue(out var recyclableTaskCompletionSource))")?;
        indented!(w, [()()()()], r"return recyclableTaskCompletionSource;")?;
        indented!(w, [()()()], r"return new RecyclableTaskCompletionSource<T>();")?;
        indented!(w, [()], r"}}")?;
        w.newline()?;
        indented!(w, [()], r"private RecyclableTaskCompletionSource()")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"// By keeping Id sequential, we limit the amount")?;
        indented!(w, [()()], r"// of mod division we need to do.")?;
        indented!(w, [()()], r"// Speeding up the lookup of TCS.")?;
        indented!(w, [()()], r"Id = Interlocked.Increment(ref _nextId) - 1;")?;
        indented!(w, [()()], r"Registered[Id] = this;")?;
        indented!(w, [()], r"}}")?;
        w.newline()?;
        indented!(w, [()], r"private ManualResetValueTaskSourceCore<T> _core;")?;
        w.newline()?;
        indented!(w, [()], r"internal ValueTask<T> GetTask() => new(this, _core.Version);")?;
        w.newline()?;
        indented!(w, [()], r"T IValueTaskSource<T>.GetResult(short token) => GetResult(token);")?;
        w.newline()?;
        indented!(w, [()], r"private T GetResult(short token)")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"try")?;
        indented!(w, [()()], r"{{")?;
        indented!(w, [()()()], r"return _core.GetResult(token);")?;
        indented!(w, [()()], r"}}")?;
        indented!(w, [()()], r"finally")?;
        indented!(w, [()()], r"{{")?;
        indented!(w, [()()()], r"_core.Reset();")?;
        indented!(w, [()()()], r"Pool.Enqueue(this);")?;
        indented!(w, [()()], r"}}")?;
        indented!(w, [()], r"}}")?;
        w.newline()?;
        indented!(w, [()], r"ValueTaskSourceStatus IValueTaskSource<T>.GetStatus(short token) => _core.GetStatus(token);")?;
        w.newline()?;
        indented!(w, [()], r"void IValueTaskSource<T>.OnCompleted(Action<object> continuation, object state, short token, ValueTaskSourceOnCompletedFlags flags) => ")?;
        indented!(w, [()()], r"_core.OnCompleted(continuation, state, token, flags);")?;
        w.newline()?;
        indented!(w, [()], r"[MethodImpl(MethodImplOptions.AggressiveInlining)]")?;
        indented!(w, [()], r"public void SetResult(T result) => _core.SetResult(result);")?;
        w.newline()?;
        indented!(w, [()], r"[MethodImpl(MethodImplOptions.AggressiveInlining)]")?;
        indented!(w, [()], r"public void SetException(Exception result) => _core.SetException(result);")?;
        indented!(w, r"}}")?;
        w.newline()?;


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

        indented!(w, r"public delegate void AsyncCallbackCommon(IntPtr data, IntPtr callback_data);")?;
        w.newline()?;
        indented!(w, r"[StructLayout(LayoutKind.Sequential)]")?;
        indented!(w, r"public partial struct AsyncCallbackCommonNative")?;
        indented!(w, r"{{")?;
        indented!(w, [()], r"internal IntPtr _ptr;")?;
        indented!(w, [()], r"internal IntPtr _ts;")?;
        indented!(w, r"}}")?;
    }
    Ok(())
}
