public partial struct EnumPayload
{
    uint _variant;
    {% for ... each variant %}
}

[NativeMarshalling(typeof(MarshallerMeta))]
public partial struct EnumPayload
{

    {% for ... unmanaged structs %}

    [StructLayout(LayoutKind.Explicit)]
    public unsafe struct Unmanaged
    {
        [FieldOffset(0)]
        internal uint _variant;

        {% for ... unmanaged fields w. offsets %}

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        internal EnumPayload ToManaged()
        {
            var _managed = new EnumPayload();
            _managed._variant = _variant;

            {% for ... variant ... convert %}

            return _managed;
        }
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    internal Unmanaged ToUnmanaged()
    {
        var _unmanaged = new Unmanaged();
        _unmanaged._variant = _variant;

        {% for ... variant ... convert %}

        return _unmanaged;
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    internal Unmanaged AsUnmanaged()
    {
        var _unmanaged = new Unmanaged();
        _unmanaged._variant = _variant;

        {% for ... variant ... convert %}

        return _unmanaged;
    }

    {% for ... variant ... ctor %}
    {% for ... variant ... IsX %}
    {% for ... variant ... AsX %}

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public override string ToString()
    {
        {% for ... variant ... return string %}
        throw new InteropException("Illegal enum state detected. This is a severe error and should never happen.");
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public Exception ExceptionForVariant()
    {
        {% for ... variant ... new Exception %}
        throw new InteropException("Illegal enum state detected. This is a severe error and should never happen.");
    }

    [CustomMarshaller(typeof(EnumPayload), MarshalMode.Default, typeof(Marshaller))]
    private struct MarshallerMeta { }

    public ref struct Marshaller
    {
        private EnumPayload _managed;
        private Unmanaged _unmanaged;

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Marshaller(EnumPayload managed) { _managed = managed; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Marshaller(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void FromManaged(EnumPayload managed) { _managed = managed; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Unmanaged ToUnmanaged() { return _managed.ToUnmanaged(); }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public EnumPayload ToManaged() { return _unmanaged.ToManaged(); }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void Free() {}
    }
}
