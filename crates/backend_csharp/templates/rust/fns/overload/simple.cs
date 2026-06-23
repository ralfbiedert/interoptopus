{%- if docs %}
{{ docs }}
{%- endif %}
{{ _fns_decorators_all }}
#if NET7_0_OR_GREATER
[LibraryImport(NativeLib, EntryPoint = "{{symbol}}")]
partial
#else
[DllImport(NativeLib, EntryPoint = "{{symbol}}", CallingConvention = CallingConvention.Cdecl)]
extern
#endif
{{ visibility }} static {{rval}} {{name}}({% for arg in args %}{{arg.ty}} {{arg.name}}{% if not loop.last %}, {% endif %}{% endfor %});
