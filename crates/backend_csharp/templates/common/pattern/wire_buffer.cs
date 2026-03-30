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

    /// Allocate a Rust-owned buffer of the given size.
    public static unsafe WireBuffer Allocate(int size)
    {
{% if plugin_mode %}
        var ptr = Trampoline.WireCreate(size, out var outLen, out var outCapacity);
{% else %}
        var ptr = WireInterop.interoptopus_wire_create(size, out var outLen, out var outCapacity);
{% endif %}
        return new WireBuffer { data = ptr, len = outLen, capacity = outCapacity };
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

    /// Free the buffer. Rust-allocated buffers (capacity > 0) are freed via the
    /// destroy helper. Borrowed or empty buffers (capacity == 0) are no-ops.
    /// Do NOT call this after passing a wire into a Rust function — Rust owns it then.
    public void Dispose()
    {
        if (data != IntPtr.Zero)
        {
{% if plugin_mode %}
            if (capacity > 0)
                Trampoline.WireDestroy(data, len, capacity);
{% else %}
            if (capacity > 0)
                WireInterop.interoptopus_wire_destroy(data, len, capacity);
{% endif %}
            data = IntPtr.Zero;
            len = 0;
            capacity = 0;
        }
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    internal unsafe Unmanaged IntoUnmanaged()
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
    internal unsafe struct Unmanaged
    {
        public IntPtr data;
        public int len;
        public int capacity;

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        internal unsafe WireBuffer IntoManaged()
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
    internal ref struct Marshaller
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
        public Unmanaged ToUnmanaged() { return _managed.IntoUnmanaged(); }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public WireBuffer ToManaged() { return _unmanaged.IntoManaged(); }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void Free() {}
    }
}

{% if not plugin_mode %}
internal partial class WireInterop {
    [LibraryImport(Interop.NativeLib, EntryPoint = "{{ create_entry_point }}")]
    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static unsafe partial IntPtr interoptopus_wire_create(int size, out int out_len, out int out_capacity);

    [LibraryImport(Interop.NativeLib, EntryPoint = "{{ destroy_entry_point }}")]
    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static partial void interoptopus_wire_destroy(IntPtr data, int len, int capacity);
}
{% endif %}
