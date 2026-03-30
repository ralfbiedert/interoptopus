{%- if docs %}
{{ docs }}
{%- endif %}
public partial {{ struct_or_class }} {{ name }}
{
    {%- for field in fields %}
    {%- if field.docs %}
    {{ field.docs | indent }}
    {%- endif %}
    public {{ field.managed_name }} {{ field.name }};
    {%- endfor %}
}
