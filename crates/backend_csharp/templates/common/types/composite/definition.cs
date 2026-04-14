{%- if docs %}
{{ docs }}
{%- endif %}
{%- if is_disposable %}
{{ _types_docs_owned }}
{%- endif %}
{{ visibility }} partial {{ struct_or_class }} {{ name }}
{
    {%- for field in fields %}
    {%- if field.docs %}
    {{ field.docs | indent }}
    {%- endif %}
    public required {{ field.managed_name }} {{ field.name }};
    {%- endfor %}
}
