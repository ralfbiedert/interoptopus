{{ _fns_decorators_all }}
public static {% if has_wraps %}unsafe {% endif %}{{rval}} {{name}}({% for arg in args %}{{arg.ty}} {{arg.name}}{% if not loop.last %}, {% endif %}{% endfor %})
{
{% for arg in args %}{% if arg.is_wrap == "true" %}    var {{arg.name}}_wrapped = new {{arg.wrapper_type}}({{arg.name}});
{% endif %}{% endfor %}{% if has_wraps %}    try
    {
        {% if is_async %}var (_cb, _cs) = Interop.{{ trampoline_field }}.NewCall();
        {{ name }}({% for arg in native_args %}{{arg.name}}, {% endfor %}_cb){% if native_rval_is_result %}.AsOk(){% endif %};
        return _cs;{% else %}{% if not is_void %}return {% endif %}{{name}}({% for arg in native_args %}{{arg.name}}{% if not loop.last %}, {% endif %}{% endfor %});{% endif %}

    }
    finally
    {
{% for arg in args %}{% if arg.is_wrap == "true" %}        {{arg.name}}_wrapped.Dispose();
{% endif %}{% endfor %}    }
{% else %}{% if is_async %}    var (_cb, _cs) = Interop.{{ trampoline_field }}.NewCall();
    {{ name }}({% for arg in native_args %}{{arg.name}}, {% endfor %}_cb){% if native_rval_is_result %}.AsOk(){% endif %};
    return _cs;
{% else %}    {% if not is_void %}return {% endif %}{{name}}({% for arg in native_args %}{{arg.name}}{% if not loop.last %}, {% endif %}{% endfor %});
{% endif %}{% endif %}}