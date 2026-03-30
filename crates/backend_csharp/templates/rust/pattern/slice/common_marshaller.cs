public partial class {{ name }}
{
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
        public Unmanaged ToUnmanaged() { return _managed.ToUnmanaged(); }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public {{ name }} ToManaged() { return _unmanaged.ToManaged(); }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public void Free() {}
    }
}