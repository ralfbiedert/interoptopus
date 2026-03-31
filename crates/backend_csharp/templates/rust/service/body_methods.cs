{%- if docs %}
{{ docs }}
{%- endif %}
{{ _fns_decorators_all }}
{{ visibility }} {{ rval }} {{ method_name }}({% for arg in args %}{{arg.ty}} {{arg.name}}{% if not loop.last %}, {% endif %}{% endfor %})
{
    {% if not is_void %}return {% endif %}Interop.{{ interop_name }}({{ self_arg }}{% for arg in args %}, {% if arg.is_ref %}ref {% endif %}{{arg.name}}{% endfor %}){% if as_ok %}.AsOk(){% endif %};
}