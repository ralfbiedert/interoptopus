// TODO: Sample method:
[MethodImpl(MethodImplOptions.AggressiveOptimization)]
public uint ReturnDefaultValue(uint x)
{
    return Interop.service_on_panic_return_default_value(_context, x); // TODO: note how we always add a fixed _context
}
