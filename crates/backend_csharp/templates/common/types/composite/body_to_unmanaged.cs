{{ _fns_decorators_all }}
{{ _fns_decorators_internal }}
internal Unmanaged {{ to_unmanaged }}()
{
    var _unmanaged = new Unmanaged();
    {%- for field in fields %}
    {%- if field.custom_to_unmanaged %}
    {{ field.custom_to_unmanaged | indent }}
    {%- else %}
    _unmanaged.{{ field.name }} = {{ field.name }}{{ field.to_unmanaged }};
    {%- endif %}
    {%- endfor %}
    return _unmanaged;
}
