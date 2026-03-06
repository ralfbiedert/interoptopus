[StructLayout(LayoutKind.Sequential)]
internal unsafe struct Unmanaged{{ variant }}
{
    internal uint _variant;
    internal {{ variant_type }} _{{ variant }};
}
