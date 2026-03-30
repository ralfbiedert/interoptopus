typedef enum {{ tag_name }}
{
{%- for v in variants %}
    {{ v.name }} = {{ v.tag }},
{%- endfor %}
} {{ tag_name }};

typedef struct {{ name }}
{
    {{ tag_name }} tag;
    union
    {
{%- for v in variants %}
{%- if v.data_type %}
        {{ v.data_type }} {{ v.field_name }};
{%- endif %}
{%- endfor %}
    };
} {{ name }};
