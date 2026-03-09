public partial {{ struct_or_class }} {{ name }}
{
    {%- for field in fields %}
    public {{ field.unmanaged_name }} {{ field.name }};
    {%- endfor %}
}
