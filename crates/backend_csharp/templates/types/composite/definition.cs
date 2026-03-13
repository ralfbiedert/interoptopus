public partial {{ struct_or_class }} {{ name }}
{
    {%- for field in fields %}
    public {{ field.managed_name }} {{ field.name }};
    {%- endfor %}
}
