{%- include "rust/pattern/vec/common_fields.cs" %}

{%- include "rust/pattern/vec/common_body.cs" %}

[NativeMarshalling(typeof(MarshallerMeta))]
public partial class {{ name }} : IDisposable
{

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static unsafe {{ name }} From(Span<{{ element_type }}> _data)
    {
        var rval = new {{ name }}();
        fixed (void* _data_ptr = _data)
        {
            InteropHelper.interoptopus_vec_create((IntPtr) _data_ptr, (ulong)_data.Length, out var _out);
            rval._len = _out._len;
            rval._capacity = _out._capacity;
            rval._ptr = _out._ptr;
        }
        return rval;
    }

    public unsafe {{ element_type }} this[int i]
    {
        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        get
        {
            if (i >= Count) throw new IndexOutOfRangeException();
            if (_ptr == IntPtr.Zero) throw new NullReferenceException();
            return Marshal.PtrToStructure<{{ element_type }}>(new IntPtr(_ptr.ToInt64() + i * sizeof({{ element_type }})));
        }
    }
}

public static class {{ name }}Extensions
{
    public static {{ name }} Vec(this {{ element_type }}[] s) { return {{ name }}.From(s); }
}