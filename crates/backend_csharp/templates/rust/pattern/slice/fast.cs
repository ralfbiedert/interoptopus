public partial class {{ name }}
{
    GCHandle _handle;
    IntPtr _data;
    ulong _len;
}


/// A {% if is_mut %}read/write{% else %}read-only{% endif %} view into a contiguous region
/// of <c>{{ element_type }}</c> elements.
///
/// Slices borrow data — they do not own it. If created from a managed array via
/// <see cref="From({{ element_type }}[])"/>, the array is pinned for the slice's
/// lifetime. Call <see cref="Dispose"/> when done to unpin the array.
/// If received from Rust, the slice points into Rust memory and is only valid
/// for the duration of the call.
{{ _types_docs_owned }}
[NativeMarshalling(typeof(MarshallerMeta))]
public partial class {{ name }} : IEnumerable<{{ element_type }}>, IDisposable
{
    /// The number of elements in this slice.
    public int Count => (int) _len;

    /// Returns a <see cref="ReadOnlySpan{T}"/> over the underlying data without copying.
    public unsafe ReadOnlySpan<{{ element_type }}> ReadOnlySpan
    {
        {{ _fns_decorators_all | indent(prefix="        ") }}
        get => new(_data.ToPointer(), (int)_len);
    }

    /// Gets {% if is_mut %}or sets {% endif %}the element at the given index.
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

    /// Creates a slice from a raw pointer and length. The caller must ensure the
    /// memory remains valid for the lifetime of this slice.
    public static {{ name }} From(IntPtr data, ulong len)
    {
        var rval = new {{ name }}();
        rval._data = data;
        rval._len = len;
        return rval;
    }

    /// Creates a slice by pinning a managed array. Call <see cref="Dispose"/> to unpin.
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

    /// Unpins the underlying managed array (if any) and invalidates this slice.
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

/// Convenience extension to convert a <c>{{ element_type }}[]</c> array to a <see cref="{{ name }}"/>.
public static class {{ name }}Extensions
{
    /// Pins the array and wraps it as a <see cref="{{ name }}"/>. Call <see cref="{{ name }}.Dispose"/> when done.
    public static {{ name }} {{ method }}(this {{ element_type }}[] s) { return {{ name }}.From(s); }
}
