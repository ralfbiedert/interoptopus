[MethodImpl(MethodImplOptions.AggressiveOptimization)]
internal Unmanaged {{ to_unmanaged }}()
{
    var _unmanaged = new Unmanaged();
    {%- for field in fields %}
    _unmanaged.{{ field.name }} = {{ field.name }}{{ field.to_unmanaged }};
    {%- endfor %}
    return _unmanaged;
}
