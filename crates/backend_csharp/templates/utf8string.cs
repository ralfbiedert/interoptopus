public partial class Utf8String
{
    IntPtr _ptr;
    ulong _len;
    ulong _capacity;
}

[NativeMarshalling(typeof(MarshallerMeta))]
public partial class Utf8String : IDisposable
{
    private Utf8String() { }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static unsafe Utf8String From(string s)
    {
        var rval = new Utf8String();
        var source = s.AsSpan();
        Span<byte> utf8Bytes = stackalloc byte[Encoding.UTF8.GetByteCount(source)];
        var len = Encoding.UTF8.GetBytes(source, utf8Bytes);

        fixed (byte* p = utf8Bytes)
        {
            InteropHelper.interoptopus_string_create((IntPtr)p, (ulong)len, out var native);
            rval._ptr = native._ptr;
            rval._len = native._len;
            rval._capacity = native._capacity;
        }

        return rval;
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static unsafe Utf8String Empty()
    {
        InteropHelper.interoptopus_string_create(IntPtr.Zero, 0, out var _out);
        return _out.IntoManaged();
    }


    public unsafe string String
    {
        get
        {
            var span = new ReadOnlySpan<byte>((byte*)_ptr, (int)_len);
            var s = Encoding.UTF8.GetString(span);
            return s;
        }
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public string IntoString()
    {
        var rval = String;
        Dispose();
        return rval;
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public void Dispose()
    {
        if (_ptr == IntPtr.Zero) return;
        var _unmanaged = new Unmanaged();
        _unmanaged._ptr = _ptr;
        _unmanaged._len = _len;
        _unmanaged._capacity = _capacity;
        InteropHelper.interoptopus_string_destroy(_unmanaged);
        _ptr = IntPtr.Zero;
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public Utf8String Clone()
    {
        var _new = new Unmanaged();
        var _this = AsUnmanaged();
        InteropHelper.interoptopus_string_clone(ref _this, ref _new);
        return _new.IntoManaged();
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public Unmanaged IntoUnmanaged()
    {
        if (_ptr == IntPtr.Zero) { throw new Exception(); }
        var _unmanaged = new Unmanaged();
        _unmanaged._ptr = _ptr;
        _unmanaged._len = _len;
        _unmanaged._capacity = _capacity;
        _ptr = IntPtr.Zero;
        return _unmanaged;
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public Unmanaged AsUnmanaged()
    {
        var _unmanaged = new Unmanaged();
        _unmanaged._ptr = _ptr;
        _unmanaged._len = _len;
        _unmanaged._capacity = _capacity;
        return _unmanaged;
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct Unmanaged
    {
        public IntPtr _ptr;
        public ulong _len;
        public ulong _capacity;

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Utf8String IntoManaged()
        {
            var _managed = new Utf8String();
            _managed._ptr = _ptr;
            _managed._len = _len;
            _managed._capacity = _capacity;
            return _managed;
        }

    }

    public partial class InteropHelper
    {
        [LibraryImport(Interop.NativeLib, EntryPoint = "interoptopus_string_create")]
        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public static partial long interoptopus_string_create(IntPtr utf8, ulong len, out Unmanaged rval);

        [LibraryImport(Interop.NativeLib, EntryPoint = "interoptopus_string_destroy")]
        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public static partial long interoptopus_string_destroy(Unmanaged utf8);

        [LibraryImport(Interop.NativeLib, EntryPoint = "interoptopus_string_clone")]
        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public static partial long interoptopus_string_clone(ref Unmanaged orig, ref Unmanaged cloned);
    }

    [CustomMarshaller(typeof(Utf8String), MarshalMode.Default, typeof(Marshaller))]
    private struct MarshallerMeta { }

    public ref struct Marshaller
    {
        private Utf8String _managed; // Used when converting managed -> unmanaged
        private Unmanaged _unmanaged; // Used when converting unmanaged -> managed

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Marshaller(Utf8String managed) { _managed = managed; }
        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Marshaller(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void FromManaged(Utf8String managed) { _managed = managed; }
        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public unsafe Unmanaged ToUnmanaged()
        {
            return _managed.IntoUnmanaged();
        }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public unsafe Utf8String ToManaged()
        {
            return _unmanaged.IntoManaged();
        }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void Free() { }
    }
}

public static class StringExtensions
{
    public static Utf8String Utf8(this string s) { return Utf8String.From(s); }
}
