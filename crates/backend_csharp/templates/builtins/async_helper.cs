[UnmanagedFunctionPointer(CallingConvention.Cdecl)]
public delegate void AsyncHelperNative(IntPtr data, IntPtr callback_data);
public delegate void AsyncHelperDelegate(IntPtr data);

public partial struct AsyncHelper
{
    private AsyncHelperDelegate _managed;
    private AsyncHelperNative _native;
    private IntPtr _ptr;
}

[NativeMarshalling(typeof(MarshallerMeta))]
public partial struct AsyncHelper : IDisposable
{
    public AsyncHelper() { }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public AsyncHelper(AsyncHelperDelegate managed)
    {
        _managed = managed;
        _native = Call;
        _ptr = Marshal.GetFunctionPointerForDelegate(_native);
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    void Call(IntPtr data, IntPtr _)
    {
        _managed(data);
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public void Dispose()
    {
        if (_ptr == IntPtr.Zero) return;
        Marshal.FreeHGlobal(_ptr);
        _ptr = IntPtr.Zero;
    }

    [CustomMarshaller(typeof(AsyncHelper), MarshalMode.Default, typeof(Marshaller))]
    private struct MarshallerMeta { }

    [StructLayout(LayoutKind.Sequential)]
    public struct Unmanaged
    {
        internal IntPtr Callback;
        internal IntPtr Data;
    }

    public ref struct Marshaller
    {
        private AsyncHelper _managed;
        private Unmanaged _unmanaged;

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void FromManaged(AsyncHelper managed) { _managed = managed; }
        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Unmanaged ToUnmanaged()
        {
            _unmanaged = new Unmanaged();
            _unmanaged.Callback = _managed._ptr;
            _unmanaged.Data = IntPtr.Zero;
            return _unmanaged;
        }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public AsyncHelper ToManaged()
        {
            _managed = new AsyncHelper();
            _managed._ptr = _unmanaged.Callback;
            return _managed;
        }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void Free() { }
    }
}

public delegate void AsyncCallbackCommon(IntPtr data, IntPtr callback_data);

[StructLayout(LayoutKind.Sequential)]
public partial struct AsyncCallbackCommonNative
{
    internal IntPtr _ptr;
    internal IntPtr _ts;
}
