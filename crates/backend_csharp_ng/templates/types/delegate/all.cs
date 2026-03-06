[UnmanagedFunctionPointer(CallingConvention.Cdecl)]
public delegate void {{ name }}Native(TODO: FOR_EACH_ARG_UNMANAGED, IntPtr callback_data); // 'True' native callback signature
public delegate void {{ name }}Delegate(TODO: FOR_EACH_ARG); // Our C# signature

public partial class {{ name }}
{
    private {{ name }}Delegate _managed; // C# callback
    private {{ name }}Native _native; // Native callback
    private IntPtr _ptr; // Raw function pointer of native callback
    private Exception _exception; // Set if the callback encountered an Exception
}

[NativeMarshalling(typeof(MarshallerMeta))]
public partial class {{ name }} : IDisposable
{

    internal {{ name }}() { }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public {{ name }}({{ name }}Delegate managed)
    {
        _managed = managed;
        _native = CallTrampoline;
        _ptr = Marshal.GetFunctionPointerForDelegate(_native);
    }

    // Helper to invoke managed code from the native invocation.
    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    private TODO: RVAL_UNMANAGED CallTrampoline(TODO: FOR_EACH_ARG_UNMANAGED, IntPtr callback_data)
    {
        // We ignore the last parameter, a generic callback pointer, as it's not needed in C#.
        try
        {
            _managed(TODO: FOR_EACH_ARG_CONVERT_TO_MANAGED) TODO_TO_UNMANAGED;
        }
        catch (Exception e)
        {
            _exception = e;
            return;
        }
    }

    // Invokes the callback.
    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    internal TODO:RVAL Call(TODO: FOR_EACH_ARG)
    {
        var __target = Marshal.GetDelegateForFunctionPointer<{{ name }}Native>(_ptr);
        // TODO - let's do this later
        // __target(x, y);
        return TODO:NOTHING_OR_default;
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public void Dispose()
    {
        // This means when the callback was invoked from Rust C# had an exception which
        // we caught (otherwise C# might not re-enter Rust, and we leak memory). Now is
        // the time to rethrow it.
        if (_exception != null) throw _exception;
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    internal Unmanaged ToUnmanaged()
    {
        var rval = new Unmanaged();
        rval._callback = _ptr;
        rval._data = IntPtr.Zero;
        return rval;
    }

    [CustomMarshaller(typeof({{ name }}), MarshalMode.Default, typeof(Marshaller))]
    private struct MarshallerMeta {  }

    [StructLayout(LayoutKind.Sequential)]
    public struct Unmanaged
    {
        internal IntPtr _callback;
        internal IntPtr _data;

        public {{ name }} ToManaged()
        {
            var rval = new {{ name }}();
            rval._ptr = _callback;
            return rval;
        }

    }

    public ref struct Marshaller
    {
        private {{ name }} _managed;
        private Unmanaged _unmanaged;

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Marshaller({{ name }} managed) { _managed = managed; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Marshaller(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void FromManaged({{ name }} managed) { _managed = managed; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Unmanaged ToUnmanaged() { return _managed.ToUnmanaged(); }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public {{ name }} ToManaged() { return _unmanaged.ToManaged(); }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void Free() {}
    }
}
