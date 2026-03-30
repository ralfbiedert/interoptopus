public partial class Utf8String
{
    IntPtr _ptr;
    ulong _len;
    ulong _capacity;
}

/// A Rust-allocated UTF-8 string.
///
/// This type wraps a native Rust <c>String</c> and provides zero-copy read access
/// via the <see cref="String"/> property.
{{ _types_docs_owned }}
[NativeMarshalling(typeof(MarshallerMeta))]
public partial class Utf8String : IDisposable
{
    private Utf8String() { }

    /// Creates a new Rust-owned <see cref="Utf8String"/> from a managed string.
    {{ _fns_decorators_all | indent }}
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

    /// Creates an empty Rust-owned <see cref="Utf8String"/>.
    {{ _fns_decorators_all | indent }}
    public static unsafe Utf8String Empty()
    {
        InteropHelper.interoptopus_string_create(IntPtr.Zero, 0, out var _out);
        return _out.IntoManaged();
    }


    /// Converts the native UTF-8 buffer to a managed string, leaving the native buffer alive.
    /// The caller must still call <see cref="Dispose"/> to free the native memory.
    public unsafe string String
    {
        get
        {
            var span = new ReadOnlySpan<byte>((byte*)_ptr, (int)_len);
            var s = Encoding.UTF8.GetString(span);
            return s;
        }
    }

    /// Converts the native UTF-8 buffer to a managed string and disposes the native buffer.
    /// After this call the <see cref="Utf8String"/> instance is consumed and must not be used again.
    {{ _fns_decorators_all | indent }}
    public string IntoString()
    {
        var rval = String;
        Dispose();
        return rval;
    }

    /// Frees the native Rust memory. Safe to call multiple times.
    {{ _fns_decorators_all | indent }}
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

    /// Creates an independent copy of this string, backed by a new Rust allocation.
    {{ _fns_decorators_all | indent }}
    public Utf8String Clone()
    {
        var _new = new Unmanaged();
        var _this = AsUnmanaged();
        InteropHelper.interoptopus_string_clone(ref _this, ref _new);
        return _new.IntoManaged();
    }

    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal Unmanaged IntoUnmanaged()
    {
        if (_ptr == IntPtr.Zero) { throw new Exception(); }
        var _unmanaged = new Unmanaged();
        _unmanaged._ptr = _ptr;
        _unmanaged._len = _len;
        _unmanaged._capacity = _capacity;
        _ptr = IntPtr.Zero;
        return _unmanaged;
    }

    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal Unmanaged AsUnmanaged()
    {
        var _unmanaged = new Unmanaged();
        _unmanaged._ptr = _ptr;
        _unmanaged._len = _len;
        _unmanaged._capacity = _capacity;
        return _unmanaged;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe struct Unmanaged
    {
        public IntPtr _ptr;
        public ulong _len;
        public ulong _capacity;

        {{ _fns_decorators_all | indent(prefix="        ") }}
        {{ _fns_decorators_internal | indent(prefix="        ") }}
        internal Utf8String IntoManaged()
        {
            var _managed = new Utf8String();
            _managed._ptr = _ptr;
            _managed._len = _len;
            _managed._capacity = _capacity;
            return _managed;
        }

    }

    internal partial class InteropHelper
    {
        [LibraryImport(Interop.NativeLib, EntryPoint = "{{ create_entry_point }}")]
        {{ _fns_decorators_all | indent(prefix="        ") }}

        public static partial long interoptopus_string_create(IntPtr utf8, ulong len, out Unmanaged rval);

        [LibraryImport(Interop.NativeLib, EntryPoint = "{{ destroy_entry_point }}")]
        {{ _fns_decorators_all | indent(prefix="        ") }}

        public static partial long interoptopus_string_destroy(Unmanaged utf8);

        [LibraryImport(Interop.NativeLib, EntryPoint = "{{ clone_entry_point }}")]
        {{ _fns_decorators_all | indent(prefix="        ") }}

        public static partial long interoptopus_string_clone(ref Unmanaged orig, ref Unmanaged cloned);
    }

    [CustomMarshaller(typeof(Utf8String), MarshalMode.Default, typeof(Marshaller))]
    private struct MarshallerMeta { }

    internal ref struct Marshaller
    {
        private Utf8String _managed; // Used when converting managed -> unmanaged
        private Unmanaged _unmanaged; // Used when converting unmanaged -> managed

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public Marshaller(Utf8String managed) { _managed = managed; }
        {{ _fns_decorators_all | indent(prefix="        ") }}
        public Marshaller(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public void FromManaged(Utf8String managed) { _managed = managed; }
        {{ _fns_decorators_all | indent(prefix="        ") }}
        public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public unsafe Unmanaged ToUnmanaged()
        {
            return _managed.IntoUnmanaged();
        }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public unsafe Utf8String ToManaged()
        {
            return _unmanaged.IntoManaged();
        }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public void Free() { }
    }
}


/// Convenience extension to convert a <see cref="string"/> to a <see cref="Utf8String"/>.
public static class StringExtensions
{
    /// Converts this string to a Rust-owned <see cref="Utf8String"/>.
    /// Call <see cref="Utf8String.Dispose"/> if the value is not passed back to Rust.
    public static Utf8String Utf8(this string s) { return Utf8String.From(s); }
}
