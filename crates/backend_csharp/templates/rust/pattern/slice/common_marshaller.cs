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
        public {{ name }} ToManaged()
        {
            // A `ref` parameter runs both directions through this one marshaller. When the
            // callee left the pointer untouched, hand back the very same wrapper so that
            // whatever it owns survives the round-trip and can still be released. Anything
            // else came from native code, so it becomes a non-owning view.
            if (_managed is not null)
            {
                var original = _managed.ToUnmanaged();
                if (original._data == _unmanaged._data && original._len == _unmanaged._len) { return _managed; }
            }
            return _unmanaged.ToManaged();
        }

        {{ _fns_decorators_all | indent(width = 8) }}
        public void Free() {}
    }
}