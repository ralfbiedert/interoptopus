{%- if docs %}
{{ docs }}
{%- endif %}
{%- if is_disposable %}
{{ _types_docs_owned }}
{%- endif %}
{{ visibility }} partial {{ struct_or_class }} {{ name }}
{
    {{ discriminant_type }} _variant;
    {%- for variant in variants %}
    {{ variant.type }} _{{ variant.name }};
    {%- endfor %}
}
