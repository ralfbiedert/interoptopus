public partial class {{ name }}
{
    {{ _fns_decorators_all | indent }}
    private {{ name }}() { }

    /// Creates an empty Rust-owned vector.
    {{ _fns_decorators_all | indent }}
    public static unsafe {{ name }} Empty()
    {
        InteropHelper.interoptopus_vec_create(IntPtr.Zero, 0, out var _out);
        return _out.IntoManaged();
    }

    /// The number of elements in this vector.
    public int Count
    {
        {{ _fns_decorators_all | indent(prefix="        ") }}
        get { if (_ptr == IntPtr.Zero) { throw new NullReferenceException(); } else { return (int) _len; } }
    }

    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal Unmanaged IntoUnmanaged()
    {
        if (_ptr == IntPtr.Zero) throw new NullReferenceException();
        var rval = new Unmanaged();
        rval._len = _len;
        rval._capacity = _capacity;
        rval._ptr = _ptr;
        _ptr = IntPtr.Zero;
        return rval;
    }

    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal Unmanaged AsUnmanaged()
    {
        if (_ptr == IntPtr.Zero) throw new NullReferenceException();
        var rval = new Unmanaged();
        rval._len = _len;
        rval._capacity = _capacity;
        rval._ptr = _ptr;
        return rval;
    }

    /// Frees the underlying Rust allocation. Safe to call multiple times.
    {{ _fns_decorators_all | indent }}
    public void Dispose()
    {
        if (_ptr == IntPtr.Zero) return;
        var _unmanaged = new Unmanaged();
        _unmanaged._ptr = _ptr;
        _unmanaged._len = _len;
        _unmanaged._capacity = _capacity;
        InteropHelper.interoptopus_vec_destroy(_unmanaged);
        _ptr = IntPtr.Zero;
        _len = 0;
        _capacity = 0;
    }

    {{ _fns_decorators_all | indent }}
    public override string ToString()
    {
        return "{{ name }} { ... }";
    }

    internal partial class InteropHelper
    {
        [LibraryImport(Interop.NativeLib, EntryPoint = "{{ create_entry_point }}")]
        internal static partial long interoptopus_vec_create(IntPtr vec, ulong len, out Unmanaged rval);
        [LibraryImport(Interop.NativeLib, EntryPoint = "{{ destroy_entry_point }}")]
        internal static partial long interoptopus_vec_destroy(Unmanaged vec);
    }

    [CustomMarshaller(typeof({{ name }}), MarshalMode.Default, typeof(Marshaller))]
    private struct MarshallerMeta { }

    [StructLayout(LayoutKind.Sequential)]
    internal struct Unmanaged
    {
        internal IntPtr _ptr;
        internal ulong _len;
        internal ulong _capacity;

        {{ _fns_decorators_all | indent(prefix="        ") }}
        {{ _fns_decorators_internal | indent(prefix="        ") }}
        internal {{ name }} IntoManaged()
        {
            var rval = new {{ name }}();
            rval._len = _len;
            rval._capacity = _capacity;
            rval._ptr = _ptr;
            return rval;
        }
    }

    internal ref struct Marshaller
    {
        private {{ name }} _managed;
        private Unmanaged _unmanaged;

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public Marshaller({{ name }} managed) { _managed = managed; }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public Marshaller(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public void FromManaged({{ name }} managed) { _managed = managed; }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public Unmanaged ToUnmanaged() { return _managed.IntoUnmanaged(); }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public {{ name }} ToManaged() { return _unmanaged.IntoManaged(); }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public void Free() {}
    }
}
