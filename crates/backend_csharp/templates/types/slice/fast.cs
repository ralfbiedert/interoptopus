public partial class {{ name }}
{
    GCHandle _handle;
    IntPtr _data;
    ulong _len;
}

[NativeMarshalling(typeof(MarshallerMeta))]
public partial class {{ name }} : IEnumerable<{{ element_type }}>, IDisposable
{
    public int Count => (int) _len;

    public unsafe ReadOnlySpan<{{ element_type }}> ReadOnlySpan
    {
        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        get => new(_data.ToPointer(), (int)_len);
    }

    public unsafe {{ element_type }} this[int i]
    {
        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        get
        {
            if (i >= Count) throw new IndexOutOfRangeException();
            return Unsafe.Read<{{ element_type }}>((void*)IntPtr.Add(_data, i * Unsafe.SizeOf<{{ element_type }}>()));
        }
{% if is_mut %}
        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        set
        {
            if (i >= Count) throw new IndexOutOfRangeException();
            Unsafe.Write<{{ element_type }}>((void*)IntPtr.Add(_data, i * Unsafe.SizeOf<{{ element_type }}>()), value);
        }
{% endif %}
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    {{ name }}() { }

    public static {{ name }} From(IntPtr data, ulong len)
    {
        var rval = new {{ name }}();
        rval._data = data;
        rval._len = len;
        return rval;
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static {{ name }} From({{ element_type }}[] managed)
    {
        var rval = new {{ name }}();
        rval._handle = GCHandle.Alloc(managed, GCHandleType.Pinned);
        rval._data = rval._handle.AddrOfPinnedObject();
        rval._len = (ulong) managed.Length;
        return rval;
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public IEnumerator<{{ element_type }}> GetEnumerator()
    {
        for (var i = 0; i < Count; ++i) { yield return this[i]; }
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public void Dispose()
    {
        if (_handle is { IsAllocated: true }) { _handle.Free(); }
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
            return {{ name }}.From(_data, _len);
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
