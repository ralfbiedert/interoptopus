{%- if docs %}
{{ docs }}
{%- endif %}
{{ _fns_decorators_all }}
{{ visibility }} {{ task_rval }} {{ method_name }}({% for arg in args %}{{arg.ty}} {{arg.name}}{% if arg.has_default == "true" %} = {{arg.default_value}}{% endif %}{% if not loop.last %}, {% endif %}{% endfor %})
{
    return Interop.{{ interop_name }}(this{% for arg in args %}, {% if arg.is_ref %}ref {% endif %}{{arg.name}}{% endfor %});
}