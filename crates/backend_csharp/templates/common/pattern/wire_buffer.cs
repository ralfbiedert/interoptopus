///FFI buffer for Wire data transfer
internal partial struct WireBuffer
{
    public IntPtr data;
    public int len;
    public int capacity;
}

[NativeMarshalling(typeof(MarshallerMeta))]
internal partial struct WireBuffer
{
    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    public WireBuffer() { }

    /// Allocate a Rust-owned buffer of the given size.
    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal static unsafe WireBuffer Allocate(int size)
    {
{% if plugin_mode %}
        var ptr = Trampoline.WireCreate(size, out var outLen, out var outCapacity);
{% else %}
        var ptr = WireInterop.interoptopus_wire_create(size, out var outLen, out var outCapacity);
{% endif %}
        return new WireBuffer { data = ptr, len = outLen, capacity = outCapacity };
    }

    /// Get a BinaryWriter over this buffer.
    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal unsafe BinaryWriter Writer()
    {
        return new BinaryWriter(new UnmanagedMemoryStream((byte*)data, len, len, FileAccess.Write));
    }

    /// Get a BinaryReader over this buffer.
    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal unsafe BinaryReader Reader()
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

    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal unsafe Unmanaged IntoUnmanaged()
    {
        var _unmanaged = new Unmanaged();
        _unmanaged.data = data;
        _unmanaged.len = len;
        _unmanaged.capacity = capacity;
        return _unmanaged;
    }

    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
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

        {{ _fns_decorators_all | indent(prefix="        ") }}
        {{ _fns_decorators_internal | indent(prefix="        ") }}
        internal unsafe WireBuffer IntoManaged()
        {
            var _managed = new WireBuffer();
            _managed.data = data;
            _managed.len = len;
            _managed.capacity = capacity;
            return _managed;
        }
    }


    {{ _fns_decorators_all | indent }}
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

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public Marshaller(WireBuffer managed) { _managed = managed; }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public Marshaller(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public void FromManaged(WireBuffer managed) { _managed = managed; }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public Unmanaged ToUnmanaged() { return _managed.IntoUnmanaged(); }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public WireBuffer ToManaged() { return _unmanaged.IntoManaged(); }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public void Free() {}
    }
}

{% if not plugin_mode %}
internal partial class WireInterop {
    [LibraryImport(Interop.NativeLib, EntryPoint = "{{ create_entry_point }}")]
    {{ _fns_decorators_all | indent }}
    public static unsafe partial IntPtr interoptopus_wire_create(int size, out int out_len, out int out_capacity);

    [LibraryImport(Interop.NativeLib, EntryPoint = "{{ destroy_entry_point }}")]
    {{ _fns_decorators_all | indent }}
    public static partial void interoptopus_wire_destroy(IntPtr data, int len, int capacity);
}
{% endif %}
