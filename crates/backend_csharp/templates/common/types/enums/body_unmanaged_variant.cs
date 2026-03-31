[StructLayout(LayoutKind.Sequential)]
internal unsafe struct Unmanaged{{ variant }}
{
    internal {{ discriminant_type }} _variant;
    internal {{ unmanaged_name }} _{{ variant }};
}
