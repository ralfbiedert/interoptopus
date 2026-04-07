[UnmanagedFunctionPointer(CallingConvention.Cdecl)]
internal delegate {{ rval_unmanaged_name }} {{ name }}Native({% for arg in args %}{{ arg.unmanaged_name }} {{ arg.name }}, {% endfor %}IntPtr callback_data);
/// Managed delegate signature for <see cref="{{ name }}"/>.
{{ visibility }} delegate {{ rval_managed }} {{ name }}Delegate({% for arg in args %}{{ arg.managed_type }} {{ arg.name }}{% if not loop.last %}, {% endif %}{% endfor %});

[UnmanagedFunctionPointer(CallingConvention.Cdecl)]
delegate void {{ name }}Destructor(IntPtr data);

{{ visibility }} partial class {{ name }}
{
    private {{ name }}Delegate _managed;
    private {{ name }}Native _native;
    private IntPtr _ptr;
    private IntPtr _data;
    private IntPtr _destructor;
    private Exception _exception;
}

/// A named callback that bridges a managed <see cref="{{ name }}Delegate"/> and an
/// unmanaged function pointer. When created from a managed delegate, exceptions
/// thrown inside the callback are captured and re-thrown on <see cref="Dispose"/>.
///
/// When received from Rust, use <see cref="Dispose"/> after the last use to allow
/// Rust to free any associated data. When passed to Rust, hold onto this class until done,
/// otherwise your function trampoline might get deallocated.
{{ _types_docs_owned }}
[NativeMarshalling(typeof(MarshallerMeta))]
{{ visibility }} partial class {{ name }} : IDisposable
{
    // Static destructor delegate and function pointer. These release the GCHandle
    // that pins `this` when a managed callback is moved to unmanaged via IntoUnmanaged().
    // Must be static so they are rooted for the lifetime of the process.
    private static void ReleaseGCHandle(IntPtr data) => GCHandle.FromIntPtr(data).Free();
    private static readonly {{ name }}Destructor _prevent_gc_release = ReleaseGCHandle;
    private static readonly IntPtr _prevent_gc_release_ptr = Marshal.GetFunctionPointerForDelegate(_prevent_gc_release);

    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal {{ name }}() { }

    /// Wraps a managed delegate so it can be passed to Rust as a callback.
    {{ _fns_decorators_all | indent }}
    public {{ name }}({{ name }}Delegate managed)
    {
        _managed = managed;
        _native = CallTrampoline;
        _ptr = Marshal.GetFunctionPointerForDelegate(_native);
    }

    {{ _fns_decorators_all | indent }}
    private {{ rval_unmanaged_name }} CallTrampoline({% for arg in args %}{{ arg.unmanaged_name }} {{ arg.name }}, {% endfor %}IntPtr callback_data)
    {
        try
        {
            {% if not is_void %}
            return _managed({% for arg in args %}{{ arg.name }}{{ arg.to_managed }}{% if not loop.last %}, {% endif %}{% endfor %}){{ rval_to_unmanaged }};
            {% else %}
            _managed({% for arg in args %}{{ arg.name }}{{ arg.to_managed }}{% if not loop.last %}, {% endif %}{% endfor %});
            {% endif %}
        }
        catch (Exception e)
        {
            _exception = e;
            {% if not is_void %}
            return default;
            {% else %}
            return;
            {% endif %}
        }
    }

    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal {{ rval_managed }} CallRaw({% for arg in args %}{{ arg.managed_type }} {{ arg.name }}{% if not loop.last %}, {% endif %}{% endfor %})
    {
        var __target = Marshal.GetDelegateForFunctionPointer<{{ name }}Native>(_ptr);
        {% if not is_void %}
        return __target({% for arg in args %}{{ arg.name }}{{ arg.to_unmanaged }}, {% endfor %}_data){{ rval_to_managed }};
        {% else %}
        __target({% for arg in args %}{{ arg.name }}{{ arg.to_unmanaged }}, {% endfor %}_data);
        {% endif %}
    }

    /// Invokes the callback. When created from a managed delegate, calls it directly.
    /// When received from Rust, marshals arguments and calls through the function pointer.
    {{ _fns_decorators_all | indent }}
    public {{ rval_managed }} Call({% for arg in args %}{{ arg.managed_type }} {{ arg.name }}{% if not loop.last %}, {% endif %}{% endfor %})
    {
        if (_ptr == IntPtr.Zero) throw new ObjectDisposedException(nameof({{ name }}));
        if (_managed != null)
        {
            {% if not is_void %}return {% endif %}_managed({% for arg in args %}{{ arg.name }}{% if not loop.last %}, {% endif %}{% endfor %});
        }
        {% if not is_void %}return {% endif %}CallRaw({% for arg in args %}{{ arg.name }}{% if not loop.last %}, {% endif %}{% endfor %});
    }

    /// Disposes the callback. If the managed delegate threw an exception during a
    /// previous invocation, that exception is re-thrown here. If the callback was
    /// received from Rust, this tells Rust to free the associated data pointer.
    {{ _fns_decorators_all | indent }}
    public void Dispose()
    {
        if (_exception != null) throw _exception;
        if (_destructor != IntPtr.Zero)
        {
            Marshal.GetDelegateForFunctionPointer<{{ name }}Destructor>(_destructor)(_data);
            _destructor = IntPtr.Zero;
        }
        _ptr = IntPtr.Zero;
    }

    /// Converts this managed callback to its unmanaged representation for passing to Rust.
    /// Pins `this` via GCHandle so the native trampoline delegate cannot be garbage collected
    /// while Rust holds the function pointer. The GCHandle is freed when Rust drops the
    /// callback and invokes the destructor.
    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal Unmanaged IntoUnmanaged()
    {
        var rval = new Unmanaged();
        rval._callback = _ptr;
        if (_managed != null)
        {
            var handle = GCHandle.Alloc(this);
            rval._data = GCHandle.ToIntPtr(handle);
            rval._destructor = _prevent_gc_release_ptr;
        }
        else
        {
            rval._data = _data;
            rval._destructor = _destructor;
        }
        return rval;
    }

    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal Unmanaged AsUnmanaged()
    {
        var rval = new Unmanaged();
        rval._callback = _ptr;
        rval._data = _data;
        rval._destructor = _destructor;
        return rval;
    }

    [CustomMarshaller(typeof({{ name }}), MarshalMode.Default, typeof(Marshaller))]
    private struct MarshallerMeta {  }

    [StructLayout(LayoutKind.Sequential)]
    internal struct Unmanaged
    {
        internal IntPtr _callback;
        internal IntPtr _data;
        internal IntPtr _destructor;

        {{ _fns_decorators_all | indent(prefix="        ") }}
        {{ _fns_decorators_internal | indent(prefix="        ") }}
        internal {{ name }} IntoManaged()
        {
            var rval = new {{ name }}();
            rval._ptr = _callback;
            rval._data = _data;
            rval._destructor = _destructor;
            return rval;
        }

    }

    internal ref struct Marshaller
    {
        private {{ name }} _managed;
        private Unmanaged _unmanaged;

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public Marshaller({{ name }} managed) { _managed = managed; }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public Marshaller(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public void FromManaged({{ name }} managed) { _managed = managed; }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public Unmanaged ToUnmanaged() { return _managed.IntoUnmanaged(); }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public {{ name }} ToManaged() { return _unmanaged.IntoManaged(); }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public void Free() {}
    }
}
