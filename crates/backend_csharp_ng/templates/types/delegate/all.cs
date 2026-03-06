[UnmanagedFunctionPointer(CallingConvention.Cdecl)]
public delegate void SumDelegateReturn2Native(int x, int y, IntPtr callback_data); // 'True' native callback signature
public delegate void SumDelegateReturn2Delegate(int x, int y); // Our C# signature

public partial class SumDelegateReturn2
{
    private SumDelegateReturn2Delegate _managed; // C# callback
    private SumDelegateReturn2Native _native; // Native callback
    private IntPtr _ptr; // Raw function pointer of native callback
    private Exception _exception; // Set if the callback encountered an Exception
}

[NativeMarshalling(typeof(MarshallerMeta))]
public partial class SumDelegateReturn2 : IDisposable
{

    internal SumDelegateReturn2() { }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public SumDelegateReturn2(SumDelegateReturn2Delegate managed)
    {
        _managed = managed;
        _native = CallTrampoline;
        _ptr = Marshal.GetFunctionPointerForDelegate(_native);
    }

    // Helper to invoke managed code from the native invocation.
    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    private void CallTrampoline(int x, int y, IntPtr callback_data)
    {
        // We ignore the last parameter, a generic callback pointer, as it's not needed in C#.
        try
        {
            _managed(x, y);
        }
        catch (Exception e)
        {
            _exception = e;
            return;
        }
    }

    // Invokes the callback.
    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    internal void Call(int x, int y)
    {
        var __target = Marshal.GetDelegateForFunctionPointer<SumDelegateReturn2Native>(_ptr);
        // TODO
        // __target(x, y);
        return;
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

    [CustomMarshaller(typeof(SumDelegateReturn2), MarshalMode.Default, typeof(Marshaller))]
    private struct MarshallerMeta {  }

    [StructLayout(LayoutKind.Sequential)]
    public struct Unmanaged
    {
        internal IntPtr _callback;
        internal IntPtr _data;

        public SumDelegateReturn2 ToManaged()
        {
            var rval = new SumDelegateReturn2();
            rval._ptr = _callback;
            return rval;
        }

    }

    public ref struct Marshaller
    {
        private SumDelegateReturn2 _managed;
        private Unmanaged _unmanaged;

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Marshaller(SumDelegateReturn2 managed) { _managed = managed; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Marshaller(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void FromManaged(SumDelegateReturn2 managed) { _managed = managed; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Unmanaged ToUnmanaged() { return _managed.ToUnmanaged(); }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public SumDelegateReturn2 ToManaged() { return _unmanaged.ToManaged(); }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void Free() {}
    }
}
