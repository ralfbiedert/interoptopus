{%- if docs %}
{{ docs }}
{%- endif %}
[LibraryImport(NativeLib, EntryPoint = "{{symbol}}")]
{%- if rval_decorator %}
[{{ rval_decorator }}]
{%- endif %}
{{ _fns_decorators_all }}
{{ visibility }} static partial {{rval}} {{name}}({% for arg in args %}{{arg.ty}} {{arg.name}}{% if not loop.last %}, {% endif %}{% endfor %});
