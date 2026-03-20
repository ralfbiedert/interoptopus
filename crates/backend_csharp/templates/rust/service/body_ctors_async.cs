[MethodImpl(MethodImplOptions.AggressiveOptimization)]
public static async Task<{{ name }}> {{ method_name }}({% for arg in args %}{{arg.ty}} {{arg.name}}{% if not loop.last %}, {% endif %}{% endfor %})
{
    var self = new {{ name }}();
    self._context = await Interop.{{ interop_name }}({% for arg in args %}{% if arg.is_ref %}ref {% endif %}{{arg.name}}{% if not loop.last %}, {% endif %}{% endfor %});
    return self;
}
