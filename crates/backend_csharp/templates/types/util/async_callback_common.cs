public delegate void AsyncCallbackCommon(IntPtr data, IntPtr callback_data);

[StructLayout(LayoutKind.Sequential)]
public partial struct AsyncCallbackCommonNative
{
    internal IntPtr _ptr;
    internal IntPtr _ts;
}
