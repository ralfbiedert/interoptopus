public partial class {{ name }} : IDisposable
{
    private IntPtr _context;

    private {{ name }}() {}

    {% for ctor in ctors %}
    {{ ctor | indent(width = 4) }}
    {% endfor %}

    {% for method in methods %}
    {{ method | indent(width = 4) }}
    {% endfor %}

    {{ _fns_decorators_all | indent }}
    public void Dispose()
    {
        Interop.{{ dtor }}(_context);
        _context = IntPtr.Zero;
    }

    internal IntPtr Context => _context;
}
