[StructLayout(LayoutKind.Explicit)]
public unsafe struct Unmanaged
{
    [FieldOffset(0)]
    internal uint _variant;

    // TODO ... for each variant ...
    [FieldOffset(0)]
    internal Unmanaged{{ variant }} _{{variant}};

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    internal EnumPayload ToManaged()
    {
        var _managed = new EnumPayload();
        _managed._variant = _variant;
        // TODO ... for each variant ...
        if (_variant == {{ variant_id }}) _managed._{{variant}} = _{{variant}}._{{variant}}{{to_managed}};
        return _managed;
    }
}
