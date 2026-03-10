// TODO SHOULD LOOK LIKE THIS
[MethodImpl(MethodImplOptions.AggressiveOptimization)]
public static ServiceBasic New()
{
    var self = new ServiceBasic();
    self._context = Interop.service_basic_new().AsOk();
    return self;
}
