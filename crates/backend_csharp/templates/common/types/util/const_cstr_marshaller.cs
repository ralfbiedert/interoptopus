#if NET7_0_OR_GREATER
[CustomMarshaller(typeof(string), MarshalMode.ManagedToUnmanagedOut, typeof(ConstCStrMarshaller))]
#endif
{{ visibility }} static class ConstCStrMarshaller
{
    public static unsafe string? ConvertToManaged(byte* unmanaged)
        => unmanaged == null ? null : Marshal.PtrToStringAnsi((nint)unmanaged);
}
