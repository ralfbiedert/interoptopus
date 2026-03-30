public partial class {{ name }}
{
    IntPtr _data;
    ulong _len;
}

/// A read-only view into a contiguous region of <c>{{ element_type }}</c> elements,
/// with marshalling support for non-blittable element types.
///
/// Elements are marshalled from their unmanaged representation on each access.
/// The slice allocates a temporary native copy via <c>Marshal.AllocHGlobal</c>;
/// call <see cref="Dispose"/> to free it.
{{ _types_docs_owned }}
[NativeMarshalling(typeof(MarshallerMeta))]
public partial class {{ name }} : IDisposable
{
    /// The number of elements in this slice.
    public int Count => (int) _len;

    /// Gets the element at the given index, marshalling from its unmanaged form.
    public unsafe {{ element_type }} this[int i]
    {
        {{ _fns_decorators_all | indent(prefix="        ") }}
        get
        {
            if (i >= (int) _len) throw new IndexOutOfRangeException();
            if (_data == IntPtr.Zero) { throw new NullReferenceException(); }
            var size = sizeof({{ unmanaged_element_type }});
            var ptr = IntPtr.Add(_data, i * size);
            var unmanaged = Marshal.PtrToStructure<{{ unmanaged_element_type }}>(ptr);
            return unmanaged.{{ element_to_managed }}();
        }
    }

    {{ _fns_decorators_all | indent }}
    {{ name }}() { }

    /// Creates a slice by marshalling each element from a managed array into a native copy.
    {{ _fns_decorators_all | indent }}
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

    /// Frees the native copy. Safe to call multiple times.
    {{ _fns_decorators_all | indent }}
    public void Dispose()
    {
        if (_data == IntPtr.Zero) return;
        Marshal.FreeHGlobal(_data);
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

    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal Unmanaged AsUnmanaged() => ToUnmanaged();

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
            var _managed = new {{ name }}();
            _managed._data = _data;
            _managed._len = _len;
            return _managed;
        }
    }
}

{%- include "rust/pattern/slice/common_marshaller.cs" %}

/// Convenience extension to convert a <c>{{ element_type }}[]</c> array to a <see cref="{{ name }}"/>.
public static class {{ name }}Extensions
{
    /// Marshals the array into a <see cref="{{ name }}"/>. Call <see cref="{{ name }}.Dispose"/> when done.
    public static {{ name }} {{ method }}(this {{ element_type }}[] s) { return {{ name }}.From(s); }
}
