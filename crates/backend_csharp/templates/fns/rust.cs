[LibraryImport(NativeLib, EntryPoint = "{{symbol}}")]
{%- if rval_decorator %}
[{{ rval_decorator }}]
{%- endif %}
[MethodImpl(MethodImplOptions.AggressiveOptimization)]
public static partial {{rval}} {{name}}({% for arg in args %}{{arg.ty}} {{arg.name}}{% if not loop.last %}, {% endif %}{% endfor %});
