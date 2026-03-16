[StructLayout(LayoutKind.Explicit)]
public unsafe struct Unmanaged
{
    [FieldOffset(0)]
    internal uint _variant;
    {%- for v in variants %}

    [FieldOffset(0)]
    internal Unmanaged{{ v.name }} _{{ v.name }};
    {%- endfor %}

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    internal {{ name }} {{ to_managed_method }}()
    {
        var _managed = new {{ name }}();
        _managed._variant = _variant;
        {%- for v in variants %}
        if (_variant == {{ v.id }}) _managed._{{ v.name }} = _{{ v.name }}._{{ v.name }}{{ v.to_managed }};
        {%- endfor %}
        return _managed;
    }
}
