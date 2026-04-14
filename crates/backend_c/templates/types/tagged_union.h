typedef {{ tag_c_type }} {{ tag_name }};
{%- for v in variants %}
#define {{ v.name }} (({{ tag_name }}){{ v.tag }})
{%- endfor %}

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
