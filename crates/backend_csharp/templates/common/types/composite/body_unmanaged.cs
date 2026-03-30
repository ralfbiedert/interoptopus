{%- if is_packed %}
[StructLayout(LayoutKind.Sequential, Pack = 1)]
{%- else %}
[StructLayout(LayoutKind.Sequential)]
{%- endif %}
internal unsafe struct Unmanaged
{
    {%- for field in fields %}
    {%- if field.is_fixed_array %}
    internal fixed {{ field.element_type }} {{ field.name }}[{{ field.len }}];
    {%- else %}
    internal {{ field.type }} {{ field.name }};
    {%- endif %}
    {%- endfor %}

    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
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
