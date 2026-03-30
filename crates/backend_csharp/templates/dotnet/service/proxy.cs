public partial class {{ name }} : IDisposable
{
    [StructLayout(LayoutKind.Sequential)]
    internal struct Unmanaged
    {
        internal IntPtr _handle;

        internal {{ name }} IntoManaged()
        {
            var h = GCHandle.FromIntPtr(_handle);
            var obj = ({{ name }})h.Target!;
            h.Free();
            return obj;
        }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        internal {{ name }} AsManaged()
        {
            return ({{ name }})GCHandle.FromIntPtr(_handle).Target!;
        }
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    internal Unmanaged IntoUnmanaged()
    {
        var h = GCHandle.Alloc(this);
        return new Unmanaged { _handle = GCHandle.ToIntPtr(h) };
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    internal Unmanaged AsUnmanaged()
    {
        var h = GCHandle.Alloc(this);
        return new Unmanaged { _handle = GCHandle.ToIntPtr(h) };
    }

    public void Dispose() { }
}
