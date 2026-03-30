{%- include "rust/pattern/vec/common_fields.cs" %}

{%- include "rust/pattern/vec/common_body.cs" %}

[NativeMarshalling(typeof(MarshallerMeta))]
public partial class {{ name }} : IDisposable
{

    {{ _fns_decorators_all | indent }}
    public static unsafe {{ name }} From(Span<{{ element_type }}> _data)
    {
        var _temp = new {{ unmanaged_element_type }}[_data.Length];
        for (var i = 0; i < _data.Length; ++i)
        {
            _temp[i] = _data[i].IntoUnmanaged();
        }
        fixed (void* _data_ptr = _temp)
        {
            InteropHelper.interoptopus_vec_create((IntPtr) _data_ptr, (ulong)_data.Length, out var _out);
            return _out.IntoManaged();
        }
    }

    public unsafe {{ element_type }} this[int i]
    {
        {{ _fns_decorators_all | indent(prefix="        ") }}
        get
        {
            if (i >= Count) throw new IndexOutOfRangeException();
            if (_ptr == IntPtr.Zero) throw new NullReferenceException();
            var _element = Marshal.PtrToStructure<{{ unmanaged_element_type }}>(new IntPtr(_ptr.ToInt64() + i * sizeof({{ unmanaged_element_type }})));
            return _element.IntoManaged();
        }
    }
}

public static class {{ name }}Extensions
{
    {{ _fns_decorators_all | indent }}
    public static {{ name }} IntoVec(this {{ element_type }}[] s) { return {{ name }}.From(s); }
}