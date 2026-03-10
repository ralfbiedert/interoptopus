public partial class {{ name }} : IDisposable
{
    private IntPtr _context;

    private {{ name }}() {}

    // TODO: Render ctors

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public void Dispose()
    {
        Interop.{{ dtor }}(_context);
        _context = IntPtr.Zero;
    }

    public IntPtr Context => _context;
}
