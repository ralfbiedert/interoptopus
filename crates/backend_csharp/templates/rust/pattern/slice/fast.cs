public partial class {{ name }}
{
    GCHandle _handle;
    IntPtr _data;
    ulong _len;
}


{{ _types_docs_owned }}
[NativeMarshalling(typeof(MarshallerMeta))]
public partial class {{ name }} : IEnumerable<{{ element_type }}>, IDisposable
{
    public int Count => (int) _len;

    public unsafe ReadOnlySpan<{{ element_type }}> ReadOnlySpan
    {
        {{ _fns_decorators_all | indent(prefix="        ") }}
        get => new(_data.ToPointer(), (int)_len);
    }

    public unsafe {{ element_type }} this[int i]
    {
        {{ _fns_decorators_all | indent(prefix="        ") }}
        get
        {
            if (i >= Count) throw new IndexOutOfRangeException();
            return Unsafe.Read<{{ element_type }}>((void*)IntPtr.Add(_data, i * Unsafe.SizeOf<{{ element_type }}>()));
        }
{% if is_mut %}
        {{ _fns_decorators_all | indent(prefix="        ") }}
        set
        {
            if (i >= Count) throw new IndexOutOfRangeException();
            Unsafe.Write<{{ element_type }}>((void*)IntPtr.Add(_data, i * Unsafe.SizeOf<{{ element_type }}>()), value);
        }
{% endif %}
    }

    {{ _fns_decorators_all | indent }}
    {{ name }}() { }

    public static {{ name }} From(IntPtr data, ulong len)
    {
        var rval = new {{ name }}();
        rval._data = data;
        rval._len = len;
        return rval;
    }

    {{ _fns_decorators_all | indent }}
    public static {{ name }} From({{ element_type }}[] managed)
    {
        var rval = new {{ name }}();
        rval._handle = GCHandle.Alloc(managed, GCHandleType.Pinned);
        rval._data = rval._handle.AddrOfPinnedObject();
        rval._len = (ulong) managed.Length;
        return rval;
    }

    {{ _fns_decorators_all | indent }}
    public IEnumerator<{{ element_type }}> GetEnumerator()
    {
        for (var i = 0; i < Count; ++i) { yield return this[i]; }
    }

    {{ _fns_decorators_all | indent }}
    IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();

    {{ _fns_decorators_all | indent }}
    public void Dispose()
    {
        if (_handle is { IsAllocated: true }) { _handle.Free(); }
        _data = IntPtr.Zero;
    }

    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
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
    internal struct Unmanaged
    {
        public IntPtr _data;
        public ulong _len;

        {{ _fns_decorators_all | indent(prefix="        ") }}
        {{ _fns_decorators_internal | indent(prefix="        ") }}
        internal {{ name }} ToManaged()
        {
            return {{ name }}.From(_data, _len);
        }
    }
}

{%- include "rust/pattern/slice/common_marshaller.cs" %}

public static class {{ name }}Extensions
{
    public static {{ name }} {{ method }}(this {{ element_type }}[] s) { return {{ name }}.From(s); }
}