typedef enum {{ name }}
{
{%- for v in variants %}
    {{ v.name }} = {{ v.value }},
{%- endfor %}
} {{ name }};
