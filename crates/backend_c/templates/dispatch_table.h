typedef struct {{ api_name }}
{
{%- for fn in functions %}
{%- if fn.separator %}
    /* internal helpers */
{%- endif %}
    {{ fn.rval }} (*{{ fn.name }})({{ fn.param_types }});
{%- endfor %}
} {{ api_name }};
