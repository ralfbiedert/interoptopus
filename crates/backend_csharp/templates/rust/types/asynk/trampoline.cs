internal class {{ trampoline_name }}
{
    private static ulong Id = 0;
    private static Dictionary<ulong, TaskCompletionSource<{% if is_task_void %}bool{% else %}{{ task_inner_ty }}{% endif %}>> InFlight = new(1024);
    private AsyncCallbackCommon _delegate;
    private IntPtr _callback_ptr;

    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal {{ trampoline_name }}()
    {
        _delegate = Call;
        _callback_ptr = Marshal.GetFunctionPointerForDelegate(_delegate);
    }

    {{ _fns_decorators_all | indent }}
    private static unsafe void Call(IntPtr data, IntPtr csPtr)
    {
        TaskCompletionSource<{% if is_task_void %}bool{% else %}{{ task_inner_ty }}{% endif %}> tcs;

        lock (InFlight) { InFlight.Remove((ulong) csPtr, out tcs); }

        // Wire layout matches Rust's `#[repr(C, u8)]` AsyncOutcome<T>:
        // byte 0 is the discriminant, payload (if any) follows at T's natural alignment.
        // `Marshal.PtrToStructure<T>` rejects generic types, so we deref via raw
        // pointer instead (the surrounding method is `unsafe`).
        var tag = Marshal.ReadByte(data, 0);
        if (tag == AsyncOutcomeTag.Cancelled)
        {
            tcs.SetException(new TaskCanceledException("Async operation was cancelled by the Rust side."));
            return;
        }

        {% if shape == "BareVoid" -%}
        tcs.SetResult(true);
        {%- elif shape == "BareDirect" -%}
        var outcome = *({{ payload_full }}*)data;
        tcs.SetResult(outcome.Value);
        {%- elif shape == "BareUnmanaged" -%}
        var outcome = *({{ payload_full }}*)data;
        var managed = outcome.Value.{{ result_to_managed }}();
        tcs.SetResult(managed);
        {%- elif shape == "ResultDirect" -%}
        var outcome = *({{ payload_full }}*)data;
        var managed = outcome.Value;
        if (managed.IsOk) { tcs.SetResult({% if is_task_void %}true{% else %}managed.AsOk(){% endif %}); }
        else { tcs.SetException(managed.ExceptionForVariant()); }
        {%- elif shape == "ResultUnmanaged" -%}
        var outcome = *({{ payload_full }}*)data;
        var managed = outcome.Value.{{ result_to_managed }}();
        if (managed.IsOk) { tcs.SetResult({% if is_task_void %}true{% else %}managed.AsOk(){% endif %}); }
        else { tcs.SetException(managed.ExceptionForVariant()); }
        {%- endif %}
    }

    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal (AsyncCallbackCommonNative, Task{% if not is_task_void %}<{{ task_inner_ty }}>{% endif %}) NewCall()
    {
        var tcs = new TaskCompletionSource<{% if is_task_void %}bool{% else %}{{ task_inner_ty }}{% endif %}>(TaskCreationOptions.RunContinuationsAsynchronously);
        var id = Interlocked.Increment(ref Id);

        lock (InFlight) { InFlight.TryAdd(id, tcs); }

        var ac = new AsyncCallbackCommonNative {
            _ptr = _callback_ptr,
            _ts = (IntPtr) id,
        };

        return (ac, tcs.Task);
    }
}
