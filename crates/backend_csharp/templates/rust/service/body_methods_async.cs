{{ _fns_decorators_all }}
public {{ task_rval }} {{ method_name }}({% for arg in args %}{{arg.ty}} {{arg.name}}{% if not loop.last %}, {% endif %}{% endfor %})
{
    return Interop.{{ interop_name }}(_context{% for arg in args %}, {% if arg.is_ref %}ref {% endif %}{{arg.name}}{% endfor %});
}