{%- if docs %}
{{ docs }}
{%- endif %}
{%- if is_disposable %}
{{ _types_docs_owned }}
{%- endif %}
public partial {{ struct_or_class }} {{ name }}
{
    int _variant;
    {%- for variant in variants %}
    {{ variant.type }} _{{ variant.name }};
    {%- endfor %}
}
