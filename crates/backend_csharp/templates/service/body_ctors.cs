[MethodImpl(MethodImplOptions.AggressiveOptimization)]
public static {{ name }} {{ method_name }}({{ params }})
{
    var self = new {{ name }}();
    self._context = Interop.{{ interop_name }}({{ args }}).AsOk();
    return self;
}