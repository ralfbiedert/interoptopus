[StructLayout(LayoutKind.Sequential)]
internal unsafe struct Unmanaged{{ variant }}
{
    internal int _variant;
    internal {{ unmanaged_name }} _{{ variant }};
}
