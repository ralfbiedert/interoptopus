[MethodImpl(MethodImplOptions.AggressiveOptimization)]
public static {{ task_rval }} {{ name }}({% for arg in args %}{{arg.ty}} {{arg.name}}{% if not loop.last %}, {% endif %}{% endfor %})
{
{% for arg in args %}{% if arg.is_wrap == "true" %}    var {{arg.name}}_wrapped = new {{arg.wrapper_type}}({{arg.name}});
{% endif %}{% endfor %}{% if has_wraps %}    try
    {
        var (_cb, _cs) = Interop.{{ trampoline_field }}.NewCall();
        {{ name }}({% for arg in native_args %}{{arg.name}}, {% endfor %}_cb){% if native_rval_is_result %}.AsOk(){% endif %};
        return _cs;
    }
    finally
    {
{% for arg in args %}{% if arg.is_wrap == "true" %}        {{arg.name}}_wrapped.Dispose();
{% endif %}{% endfor %}    }
{% else %}    var (_cb, _cs) = Interop.{{ trampoline_field }}.NewCall();
    {{ name }}({% for arg in native_args %}{{arg.name}}, {% endfor %}_cb){% if native_rval_is_result %}.AsOk(){% endif %};
    return _cs;
{% endif %}}