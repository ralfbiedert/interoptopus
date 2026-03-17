{%- if has_managed_class %}
public partial class {{ inner_type }}
{
{%- for field in field_decls %}
    {{ field }}
{%- endfor %}

    public {{ wire_name }} Wire()
    {
        return {{ wire_name }}.From(this);
    }
}

{% endif -%}
[StructLayout(LayoutKind.Sequential)]
public unsafe partial struct {{ wire_name }}
{
    byte* Data;
    int Length;
    int Capacity;

    public static {{ wire_name }} From({{ inner_type }} value)
    {
        var size = CalculateSize(value);
        var buffer = Marshal.AllocHGlobal(size);
        var wire = new {{ wire_name }}
        {
            Data = (byte*)buffer,
            Length = size,
            Capacity = -size
        };

        try
        {
            using var stream = new UnmanagedMemoryStream(wire.Data, wire.Length, wire.Length, FileAccess.Write);
            using var writer = new BinaryWriter(stream);
            {{ serialize_body | indent(prefix = "            ") }}
            return wire;
        }
        catch
        {
            Marshal.FreeHGlobal(buffer);
            throw;
        }
    }

    public {{ inner_type }} Unwire()
    {
        using var stream = new UnmanagedMemoryStream(Data, Length);
        using var reader = new BinaryReader(stream);
        {{ deserialize_body | indent(prefix = "        ") }}
    }

    static int CalculateSize({{ inner_type }} value)
    {
        {{ size_body | indent(prefix = "        ") }}
    }

    public void Dispose()
    {
        if (Data != null)
        {
            if (Capacity != 0)
            {
                if (Capacity > 0)
                    WireInterop.interoptopus_wire_destroy((IntPtr)Data, Length, Capacity);
                else
                    Marshal.FreeHGlobal((IntPtr)Data);
            }
            Data = null;
            Length = 0;
            Capacity = 0;
        }
    }
}
