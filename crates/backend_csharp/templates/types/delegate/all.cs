[UnmanagedFunctionPointer(CallingConvention.Cdecl)]
public delegate {{ rval_unmanaged_name }} {{ name }}Native({% for arg in args %}{{ arg.unmanaged_name }} {{ arg.name }}, {% endfor %}IntPtr callback_data);
public delegate {{ rval_managed }} {{ name }}Delegate({% for arg in args %}{{ arg.managed_type }} {{ arg.name }}{% if not loop.last %}, {% endif %}{% endfor %});

public partial class {{ name }}
{
    private {{ name }}Delegate _managed;
    private {{ name }}Native _native;
    private IntPtr _ptr;
    private Exception _exception;
}

[NativeMarshalling(typeof(MarshallerMeta))]
public partial class {{ name }} : IDisposable
{

    internal {{ name }}() { }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public {{ name }}({{ name }}Delegate managed)
    {
        _managed = managed;
        _native = CallTrampoline;
        _ptr = Marshal.GetFunctionPointerForDelegate(_native);
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
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

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    internal {{ rval_managed }} Call({% for arg in args %}{{ arg.managed_type }} {{ arg.name }}{% if not loop.last %}, {% endif %}{% endfor %})
    {
        var __target = Marshal.GetDelegateForFunctionPointer<{{ name }}Native>(_ptr);
        {% if not is_void %}
        return __target({% for arg in args %}{{ arg.name }}{{ arg.to_unmanaged }}, {% endfor %}IntPtr.Zero){{ rval_to_managed }};
        {% else %}
        __target({% for arg in args %}{{ arg.name }}{{ arg.to_unmanaged }}, {% endfor %}IntPtr.Zero);
        {% endif %}
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public void Dispose()
    {
        if (_exception != null) throw _exception;
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    internal Unmanaged ToUnmanaged()
    {
        var rval = new Unmanaged();
        rval._callback = _ptr;
        rval._data = IntPtr.Zero;
        return rval;
    }

    [CustomMarshaller(typeof({{ name }}), MarshalMode.Default, typeof(Marshaller))]
    private struct MarshallerMeta {  }

    [StructLayout(LayoutKind.Sequential)]
    public struct Unmanaged
    {
        internal IntPtr _callback;
        internal IntPtr _data;

        public {{ name }} ToManaged()
        {
            var rval = new {{ name }}();
            rval._ptr = _callback;
            return rval;
        }

    }

    public ref struct Marshaller
    {
        private {{ name }} _managed;
        private Unmanaged _unmanaged;

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Marshaller({{ name }} managed) { _managed = managed; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Marshaller(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void FromManaged({{ name }} managed) { _managed = managed; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Unmanaged ToUnmanaged() { return _managed.ToUnmanaged(); }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public {{ name }} ToManaged() { return _unmanaged.ToManaged(); }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void Free() {}
    }
}