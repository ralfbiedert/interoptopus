public partial class {{ name }}
{
    private readonly IntPtr _data;
    private readonly ulong _len;
    private readonly Lifetime? _lifetime;

    internal sealed class Lifetime
    {
        private object? _owner;
        private bool _disposed;

        internal Lifetime(object owner)
        {
            _owner = owner;
        }

        internal bool IsDisposed => _disposed;

        internal void Invalidate()
        {
            _disposed = true;
            _owner = null;
        }

        internal void ThrowIfDisposed()
        {
            if (_disposed)
            {
                throw new ObjectDisposedException(nameof({{ name }}Lease));
            }
        }
    }
}

/// A read-only view into a contiguous region of <c>{{ element_type }}</c> elements,
/// with marshalling support for non-blittable element types.
///
/// Elements are marshalled from their unmanaged representation on each access.
/// Slices borrow data and never own or release the underlying memory. A slice received
/// from Rust is only valid for the duration documented by the native API. Use
/// <see cref="{{ name }}Lease"/> to marshal a managed array into owned native memory.
[NativeMarshalling(typeof(MarshallerMeta))]
public partial class {{ name }}
{
    /// The number of elements in this slice.
    public int Count => _lifetime is { IsDisposed: true } ? 0 : (int) _len;

{% if has_indexer %}
    /// Gets the element at the given index, marshalling from its unmanaged form.
    public unsafe {{ element_type }} this[int i]
    {
        {{ _fns_decorators_all | indent(width = 8) }}
        get
        {
            ThrowIfDisposed();
            if (i < 0 || (ulong)i >= _len) throw new IndexOutOfRangeException();
            if (_data == IntPtr.Zero) { throw new NullReferenceException(); }
            var size = Marshal.SizeOf<{{ unmanaged_element_type }}>();
            var ptr = IntPtr.Add(_data, i * size);
            var unmanaged = Marshal.PtrToStructure<{{ unmanaged_element_type }}>(ptr);
            return unmanaged.{{ element_to_managed }}();
        }
    }
{% endif %}

    {{ _fns_decorators_all | indent }}
    internal {{ name }}(IntPtr data, ulong len, Lifetime? lifetime = null)
    {
        _data = data;
        _len = len;
        _lifetime = lifetime;
    }

    /// Marshals a managed array into a native allocation owned by the returned lease.
    {{ _fns_decorators_all | indent }}
    public static {{ name }}Lease From({{ element_type }}[] managed)
    {
        return {{ name }}Lease.From(managed);
    }

    internal bool Matches(Unmanaged unmanaged)
    {
        ThrowIfDisposed();
        return _data == unmanaged._data && _len == unmanaged._len;
    }

    internal {{ name }} Borrow(Unmanaged unmanaged)
    {
        ThrowIfDisposed();
        return new {{ name }}(unmanaged._data, unmanaged._len, _lifetime);
    }

    private void ThrowIfDisposed()
    {
        _lifetime?.ThrowIfDisposed();
    }

    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal Unmanaged ToUnmanaged()
    {
        ThrowIfDisposed();
        var unmanaged = new Unmanaged();
        unmanaged._data = _data;
        unmanaged._len = _len;
        return unmanaged;
    }

    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal Unmanaged AsUnmanaged() => ToUnmanaged();

    [CustomMarshaller(typeof({{ name }}), MarshalMode.Default, typeof(Marshaller))]
    private struct MarshallerMeta { }

    [StructLayout(LayoutKind.Sequential)]
    internal struct Unmanaged
    {
        public IntPtr _data;
        public ulong _len;

        {{ _fns_decorators_all | indent(width = 8) }}
        {{ _fns_decorators_internal | indent(width = 8) }}
        internal {{ name }} ToManaged()
        {
            return new {{ name }}(_data, _len);
        }
    }
}

{%- include "rust/pattern/slice/common_marshaller.cs" %}

/// Owns the native allocation backing a <see cref="{{ name }}"/> view.
public sealed class {{ name }}Lease : IDisposable
{
    private sealed class Allocation : SafeHandle
    {
        internal Allocation(int bytes)
            : base(IntPtr.Zero, true)
        {
            SetHandle(Marshal.AllocHGlobal(bytes));
        }

        public override bool IsInvalid => handle == IntPtr.Zero;

        protected override bool ReleaseHandle()
        {
            Marshal.FreeHGlobal(handle);
            return true;
        }
    }

    private readonly Allocation _allocation;
    private readonly {{ element_type }}[] _managed;
    private readonly {{ name }}.Lifetime _lifetime;

    private {{ name }}Lease(Allocation allocation, {{ element_type }}[] managed)
    {
        _allocation = allocation;
        _managed = managed;
        _lifetime = new {{ name }}.Lifetime(this);
        Slice = new {{ name }}(allocation.DangerousGetHandle(), (ulong)managed.Length, _lifetime);
    }

    internal static unsafe {{ name }}Lease From({{ element_type }}[] managed)
    {
        var size = Marshal.SizeOf<{{ unmanaged_element_type }}>();
        var allocation = new Allocation(checked(size * managed.Length));
        try
        {
            for (var i = 0; i < managed.Length; ++i)
            {
                var unmanaged = managed[i].AsUnmanaged();
                var dst = IntPtr.Add(allocation.DangerousGetHandle(), i * size);
                Marshal.StructureToPtr(unmanaged, dst, false);
            }
            return new {{ name }}Lease(allocation, managed);
        }
        catch
        {
            allocation.Dispose();
            throw;
        }
    }

    /// The borrowed slice backed by this lease.
    public {{ name }} Slice { get; }

    /// Returns the borrowed slice backed by this lease.
    public static implicit operator {{ name }}({{ name }}Lease lease)
    {
        ArgumentNullException.ThrowIfNull(lease);
        return lease.Slice;
    }

    /// Frees the native allocation and invalidates the borrowed slice.
    public void Dispose()
    {
        _lifetime.Invalidate();
        _allocation.Dispose();
        GC.KeepAlive(_managed);
    }
}

/// Convenience extension to marshal a <c>{{ element_type }}[]</c> array for use as a slice.
public static class {{ name }}Extensions
{
    /// Owns the native allocation until the returned lease is disposed.
    public static {{ name }}Lease {{ method }}(this {{ element_type }}[] s) { return {{ name }}.From(s); }
}
