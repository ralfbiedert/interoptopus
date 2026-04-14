{%- if has_managed_class %}
{%- if field_decls %}
public partial class {{ inner_type }}
{
{%- for field in field_decls %}
    {{ field }}
{%- endfor %}
}

{% endif -%}
{% endif -%}
/// Extension methods for converting <c>{{ inner_type }}</c> to <see cref="{{ wire_name }}"/>.
public static class {{ wire_name }}Extensions
{
    /// Serializes <paramref name="value"/> into a <see cref="{{ wire_name }}"/> for FFI transfer.
    /// Call <see cref="{{ wire_name }}.Dispose"/> on the result if it is not passed back to Rust.
    public static {{ wire_name }} Wire(this {{ inner_type }} value)
    {
        return {{ wire_name }}.From(value);
    }
}


/// Binary wire-format wrapper for <c>{{ inner_type }}</c>.
///
/// Wire types serialize complex managed objects into a flat byte buffer that
/// Rust can read. Create one with <see cref="From"/> before passing to a Rust
/// function; use <see cref="Unwire"/> to deserialize a buffer received from Rust.
{{ _types_docs_owned }}
[NativeMarshalling(typeof(MarshallerMeta))]
public partial class {{ wire_name }} : IDisposable
{
    internal WireBuffer Buffer;

    /// Serializes <paramref name="value"/> into a new wire buffer.
    {{ _fns_decorators_all | indent }}
    public static {{ wire_name }} From({{ inner_type }} value)
    {
        var size = CalculateSize(value);
        var wire = new {{ wire_name }} { Buffer = WireBuffer.Allocate(size) };

        try
        {
            using var writer = wire.Buffer.Writer();
            {{ serialize_body | indent(width = 12) }}
            return wire;
        }
        catch
        {
            wire.Dispose();
            throw;
        }
    }

    /// Deserializes the wire buffer back into a <c>{{ inner_type }}</c> instance.
    {{ _fns_decorators_all | indent }}
    public {{ inner_type }} Unwire()
    {
        using var reader = Buffer.Reader();
        {{ deserialize_body | indent(width = 8) }}
    }

    {{ _fns_decorators_all | indent }}
    static int CalculateSize({{ inner_type }} value)
    {
        {{ size_body | indent(width = 8) }}
    }

    /// Frees the underlying wire buffer.
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

        {{ _fns_decorators_all | indent(width = 8) }}
        {{ _fns_decorators_internal | indent(width = 8) }}
        internal {{ wire_name }} IntoManaged()
        {
            return new {{ wire_name }} { Buffer = Buffer };
        }
    }

    internal ref struct Marshaller
    {
        private {{ wire_name }} _managed;
        private Unmanaged _unmanaged;

        {{ _fns_decorators_all | indent(width = 8) }}
        public Marshaller({{ wire_name }} managed) { _managed = managed; }

        {{ _fns_decorators_all | indent(width = 8) }}
        public Marshaller(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        {{ _fns_decorators_all | indent(width = 8) }}
        public void FromManaged({{ wire_name }} managed) { _managed = managed; }

        {{ _fns_decorators_all | indent(width = 8) }}
        public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        {{ _fns_decorators_all | indent(width = 8) }}
        public Unmanaged ToUnmanaged() { return _managed.IntoUnmanaged(); }

        {{ _fns_decorators_all | indent(width = 8) }}
        public {{ wire_name }} ToManaged() { return _unmanaged.IntoManaged(); }

        {{ _fns_decorators_all | indent(width = 8) }}
        public void Free() {}
    }
}
