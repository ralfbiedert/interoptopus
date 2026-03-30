[UnmanagedFunctionPointer(CallingConvention.Cdecl)]
internal delegate void AsyncCallbackCommon(IntPtr data, IntPtr callback_data);

[StructLayout(LayoutKind.Sequential)]
public partial struct AsyncCallbackCommonNative
{
    internal IntPtr _ptr;
    internal IntPtr _ts;
}

public partial struct AsyncCallbackCommonNative
{
    /// Signals completion with no return value (corresponds to <c>AsyncCallback&lt;()&gt;</c> on the Rust side).
    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal unsafe void UnsafeComplete()
    {
        if (_ptr == IntPtr.Zero) return;
        var fn = (delegate* unmanaged[Cdecl]<void*, IntPtr, void>)_ptr;
        // &() in Rust is a valid zero-sized reference; pass a non-null dummy so Rust never sees null.
        byte dummy = 0;
        fn(&dummy, _ts);
    }

    /// Signals completion with a value (corresponds to <c>AsyncCallback&lt;T&gt;</c> on the Rust side).
    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal unsafe void UnsafeComplete<T>(T t) where T : unmanaged
    {
        if (_ptr == IntPtr.Zero) return;
        var fn = (delegate* unmanaged[Cdecl]<T*, IntPtr, void>)_ptr;
        fn(&t, _ts);
    }
}

