[MethodImpl(MethodImplOptions.AggressiveOptimization)]
public static {{ name }} {{ method_name }}({% for arg in args %}{{arg.ty}} {{arg.name}}{% if not loop.last %}, {% endif %}{% endfor %})
{
    var self = new {{ name }}();
    self._context = Interop.{{ interop_name }}({% for arg in args %}{{arg.name}}{% if not loop.last %}, {% endif %}{% endfor %}).AsOk();
    return self;
}