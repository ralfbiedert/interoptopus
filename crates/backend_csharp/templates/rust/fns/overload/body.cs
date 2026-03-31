{%- if docs %}
{{ docs }}
{%- endif %}
{{ _fns_decorators_all }}
{{ visibility }} static {% if is_async %}async {% endif %}{% if has_wraps %}unsafe {% endif %}{{rval}} {{name}}({% for arg in args %}{{arg.ty}} {{arg.name}}{% if arg.has_default == "true" %} = {{arg.default_value}}{% endif %}{% if not loop.last %}, {% endif %}{% endfor %})
{
{% for arg in args %}{% if arg.is_wrap == "true" %}    var {{arg.name}}_wrapped = new {{arg.wrapper_type}}({{arg.name}});
{% endif %}{% endfor %}{% if has_wraps %}    try
    {
        {% if is_async %}var (_cb, _cs) = Interop.{{ trampoline_field }}.NewCall();
        var _th = {{ name }}({% for arg in native_args %}{{arg.name}}, {% endfor %}_cb);
        using var _cr = _ct.Register(() => { unsafe { _th.Abort(); } });
        try { {% if is_task_void %}await _cs;{% else %}return await _cs;{% endif %} }
        finally { unsafe { _th.Dispose(); } }{% else %}{% if not is_void %}return {% endif %}{{name}}({% for arg in native_args %}{{arg.name}}{% if not loop.last %}, {% endif %}{% endfor %});{% endif %}

    }
    finally
    {
{% for arg in args %}{% if arg.is_wrap == "true" %}        {{arg.name}}_wrapped.Dispose();
{% endif %}{% endfor %}    }
{% else %}{% if is_async %}    var (_cb, _cs) = Interop.{{ trampoline_field }}.NewCall();
    var _th = {{ name }}({% for arg in native_args %}{{arg.name}}, {% endfor %}_cb);
    using var _cr = _ct.Register(() => { unsafe { _th.Abort(); } });
    try { {% if is_task_void %}await _cs;{% else %}return await _cs;{% endif %} }
    finally { unsafe { _th.Dispose(); } }
{% else %}    {% if not is_void %}return {% endif %}{{name}}({% for arg in native_args %}{{arg.name}}{% if not loop.last %}, {% endif %}{% endfor %});
{% endif %}{% endif %}}