[StructLayout(LayoutKind.Sequential)]
public unsafe struct Unmanaged
{
    {%- for field in fields %}
    internal {{ field.type }} {{ field.name }};
    {%- endfor %}

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    internal {{ name }} ToManaged()
    {
        var _managed = new {{ name }}();
        {%- for field in fields %}
        _managed.{{ field.name }} = {{ field.name }}{{ field.to_managed }};
        {%- endfor %}
        return _managed;
    }
}
