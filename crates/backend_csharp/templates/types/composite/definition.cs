public partial {{ struct_or_class }} {{ name }}
{
    {%- for field in fields %}
    public {{ field.type }} {{ field.name }};
    {%- endfor %}
}
