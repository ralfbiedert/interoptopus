public partial class {{ name }} : IDisposable
{
    private IntPtr _context;

    private {{ name }}() {}

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public void Dispose()
    {
        Interop.{{ dtor }}(_context).AsOk();
        _context = IntPtr.Zero;
    }

    public IntPtr Context => _context;
}
