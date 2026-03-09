[StructLayout(LayoutKind.Sequential)]
internal unsafe struct Unmanaged{{ variant }}
{
    internal uint _variant;
    internal {{ unmanaged_name }} _{{ variant }};
}
