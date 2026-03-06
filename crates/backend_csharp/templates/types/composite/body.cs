[NativeMarshalling(typeof(MarshallerMeta))]
public partial {{ struct_or_class }} {{ name }}{% if is_disposable %} : IDisposable{% endif %}
{
    public {{name}}() { }

    {{ unmanaged | indent }}

    {{ to_unmanaged | indent }}

    {{ as_unmanaged | indent }}

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public override string ToString()
    {
        return "{{name}} { ... }";
    }

    [CustomMarshaller(typeof({{ name }}), MarshalMode.Default, typeof(Marshaller))]
    private struct MarshallerMeta { }
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
