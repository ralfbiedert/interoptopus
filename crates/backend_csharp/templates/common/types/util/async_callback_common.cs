[UnmanagedFunctionPointer(CallingConvention.Cdecl)]
internal delegate void AsyncCallbackCommon(IntPtr data, IntPtr callback_data);

/// Wire-level discriminant for <c>AsyncOutcome&lt;T&gt;</c> on the Rust side
/// (see <c>interoptopus::pattern::asynk::AsyncOutcome</c>). Layout is
/// <c>#[repr(C, u8)]</c>: a one-byte tag at offset 0, then natural padding,
/// then the payload (only valid for <c>Ok</c>).
internal static class AsyncOutcomeTag
{
    public const byte Ok = 0;
    public const byte Cancelled = 1;
}

/// Mirror of Rust's <c>AsyncOutcome&lt;T&gt;</c>. Used both to read incoming
/// callback payloads and to construct outgoing ones from .NET plugins.
[StructLayout(LayoutKind.Sequential)]
internal struct AsyncOutcomeOf<T> where T : unmanaged
{
    public byte Tag;
    public T Value;
}

[StructLayout(LayoutKind.Sequential)]
{{ visibility }} partial struct AsyncCallbackCommonNative
{
    internal IntPtr _ptr;
    internal IntPtr _ts;
}

{{ visibility }} partial struct AsyncCallbackCommonNative
{
    /// Signals normal completion with no return value (corresponds to
    /// <c>AsyncCallback&lt;()&gt;</c> on the Rust side). Sends an
    /// <c>AsyncOutcome::Ok(())</c> tag (single zero byte).
    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal unsafe void UnsafeComplete()
    {
        if (_ptr == IntPtr.Zero) return;
        var fn = (delegate* unmanaged[Cdecl]<void*, IntPtr, void>)_ptr;
        byte tag = AsyncOutcomeTag.Ok;
        fn(&tag, _ts);
    }

    /// Signals normal completion with a value (corresponds to
    /// <c>AsyncCallback&lt;T&gt;</c> on the Rust side). Wraps <paramref name="t"/>
    /// in <c>AsyncOutcome::Ok</c> on the wire.
    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal unsafe void UnsafeComplete<T>(T t) where T : unmanaged
    {
        if (_ptr == IntPtr.Zero) return;
        var fn = (delegate* unmanaged[Cdecl]<AsyncOutcomeOf<T>*, IntPtr, void>)_ptr;
        var outcome = new AsyncOutcomeOf<T> { Tag = AsyncOutcomeTag.Ok, Value = t };
        fn(&outcome, _ts);
    }

    /// Signals cancellation. The Rust side will resolve the future to
    /// <c>Err(AsyncCancelled)</c>. Sends an <c>AsyncOutcome::Cancelled</c>
    /// tag (single byte equal to <c>1</c>); no payload follows.
    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal unsafe void UnsafeCompleteCancelled()
    {
        if (_ptr == IntPtr.Zero) return;
        var fn = (delegate* unmanaged[Cdecl]<void*, IntPtr, void>)_ptr;
        byte tag = AsyncOutcomeTag.Cancelled;
        fn(&tag, _ts);
    }
}

