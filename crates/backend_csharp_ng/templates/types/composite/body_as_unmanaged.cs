[MethodImpl(MethodImplOptions.AggressiveOptimization)]
internal Unmanaged AsUnmanaged()
{
    var _unmanaged = new Unmanaged();
    {%- for field in fields %}
    _unmanaged.{{ field.name }} = {{ field.name }}{{ field.as_unmanaged }};
    {%- endfor %}
    return _unmanaged;
}
