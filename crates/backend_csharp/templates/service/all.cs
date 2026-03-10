public partial class {{ name }} : IDisposable
{
    private IntPtr _context;

    private {{ name }}() {}

    {% for ctor in ctors %}
    {{ ctor | indent(prefix="    ") }}
    {% endfor %}

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public void Dispose()
    {
        Interop.{{ dtor }}(_context);
        _context = IntPtr.Zero;
    }

    public IntPtr Context => _context;
}
