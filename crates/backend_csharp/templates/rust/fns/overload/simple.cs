{%- if docs %}
{{ docs }}
{%- endif %}
[LibraryImport(NativeLib, EntryPoint = "{{symbol}}")]
{{ _fns_decorators_all }}
public static partial {{rval}} {{name}}({% for arg in args %}{{arg.ty}} {{arg.name}}{% if not loop.last %}, {% endif %}{% endfor %});
