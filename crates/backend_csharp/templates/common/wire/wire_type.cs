{%- if has_managed_class %}
public partial class {{ inner_type }}
{
{%- for field in field_decls %}
    {{ field }}
{%- endfor %}
}

public partial class {{ inner_type }}
{
    public {{ wire_name }} Wire()
    {
        return {{ wire_name }}.From(this);
    }
}

{% endif -%}
[NativeMarshalling(typeof(MarshallerMeta))]
public partial class {{ wire_name }} : IDisposable
{
    public WireBuffer Buffer;

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
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

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public {{ inner_type }} Unwire()
    {
        using var reader = Buffer.Reader();
        {{ deserialize_body | indent(prefix = "        ") }}
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    static int CalculateSize({{ inner_type }} value)
    {
        {{ size_body | indent(prefix = "        ") }}
    }

    public void Dispose()
    {
        Buffer.Dispose();
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    internal Unmanaged IntoUnmanaged()
    {
        var rval = new Unmanaged { Buffer = Buffer };
        Buffer = default;
        return rval;
    }

    [CustomMarshaller(typeof({{ wire_name }}), MarshalMode.Default, typeof(Marshaller))]
    private struct MarshallerMeta { }

    [StructLayout(LayoutKind.Sequential)]
    public struct Unmanaged
    {
        public WireBuffer Buffer;

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public {{ wire_name }} IntoManaged()
        {
            return new {{ wire_name }} { Buffer = Buffer };
        }
    }

    public ref struct Marshaller
    {
        private {{ wire_name }} _managed;
        private Unmanaged _unmanaged;

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Marshaller({{ wire_name }} managed) { _managed = managed; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Marshaller(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void FromManaged({{ wire_name }} managed) { _managed = managed; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Unmanaged ToUnmanaged() { return _managed.IntoUnmanaged(); }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public {{ wire_name }} ToManaged() { return _unmanaged.IntoManaged(); }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void Free() {}
    }
}
