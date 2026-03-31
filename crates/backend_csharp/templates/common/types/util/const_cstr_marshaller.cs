[CustomMarshaller(typeof(string), MarshalMode.ManagedToUnmanagedOut, typeof(ConstCStrMarshaller))]
{{ visibility }} static class ConstCStrMarshaller
{
    public static unsafe string? ConvertToManaged(byte* unmanaged)
        => unmanaged == null ? null : Marshal.PtrToStringAnsi((nint)unmanaged);
}
