[StructLayout(LayoutKind.Sequential)]
public unsafe struct Unmanaged
{
    {%- for field in fields %}
    internal {{ field.type }} {{ field.name }};
    {%- endfor %}

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    internal {{ name }} ToManaged()
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
