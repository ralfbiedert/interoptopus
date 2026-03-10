[MethodImpl(MethodImplOptions.AggressiveOptimization)]
public {{ rval }} {{ method_name }}({% for arg in args %}{{arg.ty}} {{arg.name}}{% if not loop.last %}, {% endif %}{% endfor %})
{
    return Interop.{{ interop_name }}(_context{% for arg in args %}, {{arg.name}}{% endfor %});
}