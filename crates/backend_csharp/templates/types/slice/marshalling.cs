public partial class {{ name }}
{
    IntPtr _data;
    ulong _len;
}

[NativeMarshalling(typeof(MarshallerMeta))]
public partial class {{ name }} : IDisposable
{
    public int Count => (int) _len;

    public unsafe {{ element_type }} this[int i]
    {
        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        get
        {
            if (i >= (int) _len) throw new IndexOutOfRangeException();
            if (_data == IntPtr.Zero) { throw new NullReferenceException(); }
            var size = sizeof({{ unmanaged_element_type }});
            var ptr = IntPtr.Add(_data, i * size);
            var unmanaged = Marshal.PtrToStructure<{{ unmanaged_element_type }}>(ptr);
            return unmanaged.ToManaged();
        }
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    {{ name }}() { }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static unsafe {{ name }} From({{ element_type }}[] managed)
    {
        var rval = new {{ name }}();
        var size = sizeof({{ unmanaged_element_type }});
        rval._data = Marshal.AllocHGlobal(size * managed.Length);
        rval._len = (ulong) managed.Length;
        for (var i = 0; i < managed.Length; ++i)
        {
            var unmanaged = managed[i].AsUnmanaged();
            var dst = IntPtr.Add(rval._data, i * size);
            Marshal.StructureToPtr(unmanaged, dst, false);
        }
        return rval;
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public void Dispose()
    {
        if (_data == IntPtr.Zero) return;
        Marshal.FreeHGlobal(_data);
        _data = IntPtr.Zero;
    }

    internal Unmanaged ToUnmanaged()
    {
        var unmanaged = new Unmanaged();
        unmanaged._data = _data;
        unmanaged._len = _len;
        return unmanaged;
    }

    [CustomMarshaller(typeof({{ name }}), MarshalMode.Default, typeof(Marshaller))]
    private struct MarshallerMeta { }

    [StructLayout(LayoutKind.Sequential)]
    public struct Unmanaged
    {
        public IntPtr _data;
        public ulong _len;

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        internal {{ name }} ToManaged()
        {
            var _managed = new {{ name }}();
            _managed._data = _data;
            _managed._len = _len;
            return _managed;
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

    public static class {{ name }}Extensions
    {
        public static {{ name }} {{ method }}(this {{ element_type }}[] s) { return {{ name }}.From(s); }
    }
