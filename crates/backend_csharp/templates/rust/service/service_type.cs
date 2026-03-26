public partial class {{ name }}
{
    [StructLayout(LayoutKind.Sequential)]
    public struct Unmanaged
    {
        internal IntPtr _handle;

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public {{ name }} IntoManaged()
        {
            var h = GCHandle.FromIntPtr(_handle);
            var obj = ({{ name }})h.Target!;
            h.Free();
            return obj;
        }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public {{ name }} AsManaged()
        {
            return ({{ name }})GCHandle.FromIntPtr(_handle).Target!;
        }
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public Unmanaged IntoUnmanaged()
    {
        var h = GCHandle.Alloc(this);
        return new Unmanaged { _handle = GCHandle.ToIntPtr(h) };
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public Unmanaged AsUnmanaged()
    {
        var h = GCHandle.Alloc(this);
        return new Unmanaged { _handle = GCHandle.ToIntPtr(h) };
    }
}
