typedef {{ tag_c_type }} {{ name }};
{%- for v in variants %}
#define {{ v.name }} (({{ name }}){{ v.value }})
{%- endfor %}
