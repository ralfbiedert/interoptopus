[StructLayout(LayoutKind.Sequential)]
public unsafe struct Unmanaged
{
    {%- for field in fields %}
    {%- if field.is_fixed_array %}
    internal fixed {{ field.element_type }} {{ field.name }}[{{ field.len }}];
    {%- else %}
    internal {{ field.type }} {{ field.name }};
    {%- endif %}
    {%- endfor %}

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    internal {{ name }} {{ to_managed_method }}()
    {
        var _managed = new {{ name }}();
        {%- for field in fields %}
        {%- if field.custom_to_managed %}
        {{ field.custom_to_managed | indent(prefix = "        ") }}
        {%- else %}
        _managed.{{ field.name }} = {{ field.name }}{{ field.to_managed }};
        {%- endif %}
        {%- endfor %}
        return _managed;
    }
}
