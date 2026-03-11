[MethodImpl(MethodImplOptions.AggressiveOptimization)]
public {{ rval }} {{ method_name }}({% for arg in args %}{{arg.ty}} {{arg.name}}{% if not loop.last %}, {% endif %}{% endfor %})
{
    {% for arg in args %}{% if arg.is_wrap %}
    var {{arg.name}}_wrapped = new {{arg.wrapper_type}}({{arg.name}});
    {% endif %}{% endfor %}
    try
    {
        {% if not is_void %}return {% endif %}Interop.{{ interop_name }}(_context{% for arg in args %}, {% if arg.is_wrap %}{{arg.name}}_wrapped{% elif arg.is_ref %}ref {{arg.name}}{% else %}{{arg.name}}{% endif %}{% endfor %});
    }
    finally
    {
        {% for arg in args %}{% if arg.is_wrap %}
        {{arg.name}}_wrapped.Dispose();
        {% endif %}{% endfor %}
    }
}