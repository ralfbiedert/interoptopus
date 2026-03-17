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

    /// Allocate a C#-owned buffer of the given size.
    public static unsafe WireBuffer Allocate(int size)
    {
        var buf = new WireBuffer();
        buf.data = Marshal.AllocHGlobal(size);
        buf.len = size;
        buf.capacity = -size;
        return buf;
    }

    /// Get a BinaryWriter over this buffer.
    public unsafe BinaryWriter Writer()
    {
        return new BinaryWriter(new UnmanagedMemoryStream((byte*)data, len, len, FileAccess.Write));
    }

    /// Get a BinaryReader over this buffer.
    public unsafe BinaryReader Reader()
    {
        return new BinaryReader(new UnmanagedMemoryStream((byte*)data, len));
    }

    /// Free the buffer. Rust-allocated buffers (capacity > 0) are freed via
    /// interoptopus_wire_destroy; C#-allocated buffers (capacity < 0) via
    /// Marshal.FreeHGlobal. Borrowed buffers (capacity == 0) are not freed.
    public void Dispose()
    {
        if (data != IntPtr.Zero)
        {
            if (capacity > 0)
                WireInterop.interoptopus_wire_destroy(data, len, capacity);
            else if (capacity < 0)
                Marshal.FreeHGlobal(data);
            data = IntPtr.Zero;
            len = 0;
            capacity = 0;
        }
    }

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
