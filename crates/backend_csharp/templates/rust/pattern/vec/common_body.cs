public partial class {{ name }}
{
    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    private {{ name }}() { }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static unsafe {{ name }} Empty()
    {
        InteropHelper.interoptopus_vec_create(IntPtr.Zero, 0, out var _out);
        return _out.IntoManaged();
    }

    public int Count
    {
        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        get { if (_ptr == IntPtr.Zero) { throw new NullReferenceException(); } else { return (int) _len; } }
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
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

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    internal Unmanaged AsUnmanaged()
    {
        if (_ptr == IntPtr.Zero) throw new NullReferenceException();
        var rval = new Unmanaged();
        rval._len = _len;
        rval._capacity = _capacity;
        rval._ptr = _ptr;
        return rval;
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
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

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
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

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
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

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Marshaller({{ name }} managed) { _managed = managed; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Marshaller(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void FromManaged({{ name }} managed) { _managed = managed; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Unmanaged ToUnmanaged() { return _managed.IntoUnmanaged(); }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public {{ name }} ToManaged() { return _unmanaged.IntoManaged(); }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void Free() {}
    }
}