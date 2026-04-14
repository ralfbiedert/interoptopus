public partial class {{ name }}
{
    internal ref struct Marshaller
    {
        private {{ name }} _managed;
        private Unmanaged _unmanaged;

        {{ _fns_decorators_all | indent(width = 8) }}
        public Marshaller({{ name }} managed) { _managed = managed; }

        {{ _fns_decorators_all | indent(width = 8) }}
        public Marshaller(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        {{ _fns_decorators_all | indent(width = 8) }}
        public void FromManaged({{ name }} managed) { _managed = managed; }

        {{ _fns_decorators_all | indent(width = 8) }}
        public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        {{ _fns_decorators_all | indent(width = 8) }}
        public Unmanaged ToUnmanaged() { return _managed.ToUnmanaged(); }

        {{ _fns_decorators_all | indent(width = 8) }}
        public {{ name }} ToManaged() { return _unmanaged.ToManaged(); }

        {{ _fns_decorators_all | indent(width = 8) }}
        public void Free() {}
    }
}