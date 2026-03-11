[MethodImpl(MethodImplOptions.AggressiveOptimization)]
public static unsafe {{rval}} {{name}}({% for arg in args %}{{arg.ty}} {{arg.name}}{% if not loop.last %}, {% endif %}{% endfor %})
{
    {% for arg in args %}{% if arg.is_wrap == "true" %}
    var {{arg.name}}_wrapped = new {{arg.wrapper_type}}({{arg.name}});
    {% endif %}{% endfor %}
    try
    {
        {% if not is_void %}return {% endif %}{{name}}({% for arg in args %}{% if arg.is_wrap == "true" %}{{arg.name}}_wrapped{% elif arg.is_ref == "true" %}ref {{arg.name}}{% else %}{{arg.name}}{% endif %}{% if not loop.last %}, {% endif %}{% endfor %});
    }
    finally
    {
        {% for arg in args %}{% if arg.is_wrap == "true" %}
        {{arg.name}}_wrapped.Dispose();
        {% endif %}{% endfor %}
    }
}