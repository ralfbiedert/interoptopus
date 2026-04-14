#ifdef {{ guard }}
static int {{ load_fn }}({{ api_name }}* api)
{
{%- for fn in functions %}
{%- if fn.separator %}
    /* internal helpers */
{%- endif %}
    api->{{ fn.field_name }} = {{ fn.symbol }};
{%- endfor %}
    return 0;
}
#endif /* {{ guard }} */
