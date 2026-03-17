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
public partial struct {{ wire_name }}
{
    public WireBuffer Buffer;
}

public partial struct {{ wire_name }}
{
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

    public {{ inner_type }} Unwire()
    {
        using var reader = Buffer.Reader();
        {{ deserialize_body | indent(prefix = "        ") }}
    }

    static int CalculateSize({{ inner_type }} value)
    {
        {{ size_body | indent(prefix = "        ") }}
    }

    public void Dispose()
    {
        Buffer.Dispose();
    }
}
