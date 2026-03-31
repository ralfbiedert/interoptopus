[StructLayout(LayoutKind.Explicit)]
internal unsafe struct Unmanaged
{
    [FieldOffset(0)]
    internal {{ discriminant_type }} _variant;
    {%- for v in variants %}

    [FieldOffset(0)]
    internal Unmanaged{{ v.name }} _{{ v.name }};
    {%- endfor %}

    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
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
