///FFI buffer for Wire data transfer
public partial struct WireBuffer
{
    public IntPtr data;
    public int len;
    public int capacity;
}

[NativeMarshalling(typeof(MarshallerMeta))]
public partial struct WireBuffer
{
    public WireBuffer() { }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    internal unsafe Unmanaged ToUnmanaged()
    {
        var _unmanaged = new Unmanaged();
        _unmanaged.data = data;
        _unmanaged.len = len;
        _unmanaged.capacity = capacity;
        return _unmanaged;
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    internal unsafe Unmanaged AsUnmanaged()
    {
        var _unmanaged = new Unmanaged();
        _unmanaged.data = data;
        _unmanaged.len = len;
        _unmanaged.capacity = capacity;
        return _unmanaged;
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct Unmanaged
    {
        public IntPtr data;
        public int len;
        public int capacity;

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        internal unsafe WireBuffer ToManaged()
        {
            var _managed = new WireBuffer();
            _managed.data = data;
            _managed.len = len;
            _managed.capacity = capacity;
            return _managed;
        }
    }


    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public override string ToString()
    {
        return "WireBuffer { ... }";
    }

    [CustomMarshaller(typeof(WireBuffer), MarshalMode.Default, typeof(Marshaller))]
    private struct MarshallerMeta { }
    public ref struct Marshaller
    {
        private WireBuffer _managed;
        private Unmanaged _unmanaged;

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Marshaller(WireBuffer managed) { _managed = managed; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Marshaller(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void FromManaged(WireBuffer managed) { _managed = managed; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Unmanaged ToUnmanaged() { return _managed.ToUnmanaged(); }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public WireBuffer ToManaged() { return _unmanaged.ToManaged(); }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void Free() {}
    }
}

public partial class WireInterop {
    [LibraryImport(Interop.NativeLib, EntryPoint = "{{ destroy_entry_point }}")]
    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static partial void interoptopus_wire_destroy(IntPtr data, int len, int capacity);
}
