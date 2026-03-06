[MethodImpl(MethodImplOptions.AggressiveOptimization)]
internal Unmanaged {{ to_unmanaged }}()
{
    var _unmanaged = new Unmanaged();
    _unmanaged._variant = _variant;
    {%- for v in variants %}
    if (_variant == {{ v.id }}) _unmanaged._{{ v.name }}._{{ v.name }} = _{{ v.name }}{{ v.to_unmanaged }};
    {%- endfor %}
    return _unmanaged;
}
