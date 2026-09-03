public partial class {{ name }}
{
    private readonly IntPtr _data;
    private readonly ulong _len;
    private readonly Lifetime? _lifetime;
    private readonly ReadOnlyMemory<{{ element_type }}>? _managedMemory;

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


/// A {% if is_mut %}read/write{% else %}read-only{% endif %} view into a contiguous region
/// of <c>{{ element_type }}</c> elements.
///
/// Slices borrow data and never own or release the underlying memory. A slice received
/// from Rust is only valid for the duration documented by the native API. Use
/// <see cref="{{ name }}Lease"/> to pin a managed array for use as a slice.
[NativeMarshalling(typeof(MarshallerMeta))]
public partial class {{ name }} : IEnumerable<{{ element_type }}>
{
    /// The number of elements in this slice.
    public int Count => _lifetime is { IsDisposed: true } ? 0 : (int) _len;

    /// Returns a <see cref="ReadOnlySpan{T}"/> over the underlying data without copying.
    public unsafe ReadOnlySpan<{{ element_type }}> ReadOnlySpan
    {
        {{ _fns_decorators_all | indent(width = 8) }}
        get
        {
            ThrowIfDisposed();
            if (_managedMemory is { } managedMemory)
            {
                return managedMemory.Span;
            }
            return new(_data.ToPointer(), (int)_len);
        }
    }

    /// Gets {% if is_mut %}or sets {% endif %}the element at the given index.
    public unsafe {{ element_type }} this[int i]
    {
        {{ _fns_decorators_all | indent(width = 8) }}
        get
        {
            ThrowIfDisposed();
            if (i < 0 || (ulong)i >= _len) throw new IndexOutOfRangeException();
            return Unsafe.Read<{{ element_type }}>((void*)IntPtr.Add(_data, i * Unsafe.SizeOf<{{ element_type }}>()));
        }
{% if is_mut %}
        {{ _fns_decorators_all | indent(width = 8) }}
        set
        {
            ThrowIfDisposed();
            if (i < 0 || (ulong)i >= _len) throw new IndexOutOfRangeException();
            Unsafe.Write<{{ element_type }}>((void*)IntPtr.Add(_data, i * Unsafe.SizeOf<{{ element_type }}>()), value);
        }
{% endif %}
    }

    {{ _fns_decorators_all | indent }}
    internal {{ name }}(
        IntPtr data,
        ulong len,
        Lifetime? lifetime = null,
        ReadOnlyMemory<{{ element_type }}>? managedMemory = null)
    {
        _data = data;
        _len = len;
        _lifetime = lifetime;
        _managedMemory = managedMemory;
    }

    /// Creates a slice from a raw pointer and length. The caller must ensure the
    /// memory remains valid for the lifetime of this slice.
    public static {{ name }} From(IntPtr data, ulong len)
    {
        return new {{ name }}(data, len);
    }

    internal bool Matches(Unmanaged unmanaged)
    {
        ThrowIfDisposed();
        return _data == unmanaged._data && _len == unmanaged._len;
    }

    internal {{ name }} Borrow(Unmanaged unmanaged)
    {
        ThrowIfDisposed();
        return new {{ name }}(unmanaged._data, unmanaged._len, _lifetime, BorrowManagedMemory(unmanaged));
    }

    private unsafe ReadOnlyMemory<{{ element_type }}>? BorrowManagedMemory(Unmanaged unmanaged)
    {
        if (_managedMemory is not { } managedMemory)
        {
            return null;
        }

        var elementSize = (nuint)Unsafe.SizeOf<{{ element_type }}>();
        var start = (nuint)_data.ToPointer();
        var data = (nuint)unmanaged._data.ToPointer();
        if (data < start)
        {
            return null;
        }

        var offsetBytes = data - start;
        if (offsetBytes % elementSize != 0)
        {
            return null;
        }

        var offset = offsetBytes / elementSize;
        if (offset > (nuint)managedMemory.Length)
        {
            return null;
        }

        var remaining = (ulong)(managedMemory.Length - (int)offset);
        if (unmanaged._len > remaining)
        {
            return null;
        }

        return managedMemory.Slice((int)offset, (int)unmanaged._len);
    }

    private void ThrowIfDisposed()
    {
        _lifetime?.ThrowIfDisposed();
    }

    /// Pins a managed array and returns a lease whose view can be passed as a slice.
    {{ _fns_decorators_all | indent }}
    public static {{ name }}Lease From({{ element_type }}[] managed)
    {
        return new {{ name }}Lease(managed);
    }

    {{ _fns_decorators_all | indent }}
    public IEnumerator<{{ element_type }}> GetEnumerator()
    {
        for (var i = 0; i < Count; ++i) { yield return this[i]; }
    }

    {{ _fns_decorators_all | indent }}
    IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();

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
            return {{ name }}.From(_data, _len);
        }
    }
}

{%- include "rust/pattern/slice/common_marshaller.cs" %}

/// Owns the pinned managed array backing a <see cref="{{ name }}"/> view.
public sealed class {{ name }}Lease : IDisposable
{
    private GCHandle _handle;
    private readonly {{ name }}.Lifetime _lifetime;

    internal {{ name }}Lease({{ element_type }}[] managed)
    {
        _handle = GCHandle.Alloc(managed, GCHandleType.Pinned);
        _lifetime = new {{ name }}.Lifetime(this);
        Slice = new {{ name }}(_handle.AddrOfPinnedObject(), (ulong)managed.Length, _lifetime, managed);
    }

    /// The borrowed slice backed by this lease.
    public {{ name }} Slice { get; }

    /// Returns the borrowed slice backed by this lease.
    public static implicit operator {{ name }}({{ name }}Lease lease)
    {
        ArgumentNullException.ThrowIfNull(lease);
        return lease.Slice;
    }

    /// Unpins the managed array and invalidates the borrowed slice.
    public void Dispose()
    {
        _lifetime.Invalidate();
        if (_handle is { IsAllocated: true }) { _handle.Free(); }
        GC.SuppressFinalize(this);
    }

    ~{{ name }}Lease()
    {
        _lifetime.Invalidate();
        if (_handle is { IsAllocated: true }) { _handle.Free(); }
    }
}

/// Convenience extension to pin a <c>{{ element_type }}[]</c> array for use as a slice.
public static class {{ name }}Extensions
{
    /// Pins the array until the returned lease is disposed.
    public static {{ name }}Lease {{ method }}(this {{ element_type }}[] s) { return {{ name }}.From(s); }
}
