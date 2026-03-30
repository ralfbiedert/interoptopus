public partial class {{ name }} : IDisposable
{
    [StructLayout(LayoutKind.Sequential)]
    internal struct Unmanaged
    {
        internal IntPtr _handle;

        {{ _fns_decorators_all | indent(prefix="        ") }}
        {{ _fns_decorators_internal | indent(prefix="        ") }}
        internal {{ name }} IntoManaged()
        {
            var h = GCHandle.FromIntPtr(_handle);
            var obj = ({{ name }})h.Target!;
            h.Free();
            return obj;
        }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        {{ _fns_decorators_internal | indent(prefix="        ") }}
        internal {{ name }} AsManaged()
        {
            return ({{ name }})GCHandle.FromIntPtr(_handle).Target!;
        }
    }

    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal Unmanaged IntoUnmanaged()
    {
        var h = GCHandle.Alloc(this);
        return new Unmanaged { _handle = GCHandle.ToIntPtr(h) };
    }

    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal Unmanaged AsUnmanaged()
    {
        var h = GCHandle.Alloc(this);
        return new Unmanaged { _handle = GCHandle.ToIntPtr(h) };
    }

    public void Dispose() { }
}
