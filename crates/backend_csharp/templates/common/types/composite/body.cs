[NativeMarshalling(typeof(MarshallerMeta))]
{{ visibility }} partial {{ struct_or_class }} {{ name }}{% if is_disposable %} : IDisposable{% endif %}
{
    public {{name}}() { }

    {{ unmanaged | indent }}

    {{ to_unmanaged | indent }}

    {{ as_unmanaged | indent }}
{% if is_disposable %}
    public void Dispose()
    {
        {%- for field in disposable_fields %}
        {{ field.name }}?.Dispose();
        {%- endfor %}
    }
{% endif %}
    {{ _fns_decorators_all | indent }}
    public override string ToString()
    {
        return "{{name}} { ... }";
    }

    [CustomMarshaller(typeof({{ name }}), MarshalMode.Default, typeof(Marshaller))]
    private struct MarshallerMeta { }
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
        public Unmanaged ToUnmanaged() { return _managed.{{ marshaller_to_unmanaged }}(); }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public {{ name }} ToManaged() { return _unmanaged.{{ marshaller_to_managed }}(); }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public void Free() {}
    }

}
