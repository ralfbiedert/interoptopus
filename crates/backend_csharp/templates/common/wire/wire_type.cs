{%- if has_managed_class %}
{%- if field_decls %}
public partial class {{ inner_type }}
{
{%- for field in field_decls %}
    {{ field }}
{%- endfor %}
}

{% endif -%}
public partial class {{ inner_type }}
{
    public {{ wire_name }} Wire()
    {
        return {{ wire_name }}.From(this);
    }
}

{% endif -%}
{{ _types_docs_owned }}
[NativeMarshalling(typeof(MarshallerMeta))]
public partial class {{ wire_name }} : IDisposable
{
    internal WireBuffer Buffer;

    {{ _fns_decorators_all | indent }}
    public static {{ wire_name }} From({{ inner_type }} value)
    {
        var size = CalculateSize(value);
        var wire = new {{ wire_name }} { Buffer = WireBuffer.Allocate(size) };

        try
        {
            using var writer = wire.Buffer.Writer();
            {{ serialize_body | indent(prefix = "            ") }}
            return wire;
        }
        catch
        {
            wire.Dispose();
            throw;
        }
    }

    {{ _fns_decorators_all | indent }}
    public {{ inner_type }} Unwire()
    {
        using var reader = Buffer.Reader();
        {{ deserialize_body | indent(prefix = "        ") }}
    }

    {{ _fns_decorators_all | indent }}
    static int CalculateSize({{ inner_type }} value)
    {
        {{ size_body | indent(prefix = "        ") }}
    }

    public void Dispose()
    {
        Buffer.Dispose();
    }

    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal Unmanaged IntoUnmanaged()
    {
        var rval = new Unmanaged { Buffer = Buffer };
        Buffer = default;
        return rval;
    }
    
    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal Unmanaged AsUnmanaged()
    {
        var rval = new Unmanaged { Buffer = Buffer };
        return rval;
    }
    
    [CustomMarshaller(typeof({{ wire_name }}), MarshalMode.Default, typeof(Marshaller))]
    private struct MarshallerMeta { }

    [StructLayout(LayoutKind.Sequential)]
    internal struct Unmanaged
    {
        public WireBuffer Buffer;

        {{ _fns_decorators_all | indent(prefix="        ") }}
        {{ _fns_decorators_internal | indent(prefix="        ") }}
        internal {{ wire_name }} IntoManaged()
        {
            return new {{ wire_name }} { Buffer = Buffer };
        }
    }

    internal ref struct Marshaller
    {
        private {{ wire_name }} _managed;
        private Unmanaged _unmanaged;

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public Marshaller({{ wire_name }} managed) { _managed = managed; }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public Marshaller(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public void FromManaged({{ wire_name }} managed) { _managed = managed; }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public Unmanaged ToUnmanaged() { return _managed.IntoUnmanaged(); }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public {{ wire_name }} ToManaged() { return _unmanaged.IntoManaged(); }

        {{ _fns_decorators_all | indent(prefix="        ") }}
        public void Free() {}
    }
}
