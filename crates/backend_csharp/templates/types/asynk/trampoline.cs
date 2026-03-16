public class {{ trampoline_name }}
{
    private static ulong Id = 0;
    private static Dictionary<ulong, TaskCompletionSource<{% if is_task_void %}bool{% else %}{{ task_inner_ty }}{% endif %}>> InFlight = new(1024);
    private AsyncCallbackCommon _delegate;
    private IntPtr _callback_ptr;

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    internal {{ trampoline_name }}()
    {
        _delegate = Call;
        _callback_ptr = Marshal.GetFunctionPointerForDelegate(_delegate);
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    private static void Call(IntPtr data, IntPtr csPtr)
    {
        TaskCompletionSource<{% if is_task_void %}bool{% else %}{{ task_inner_ty }}{% endif %}> tcs;

        lock (InFlight) { InFlight.Remove((ulong) csPtr, out tcs); }

        {% if has_unmanaged %}var unmanaged = Marshal.PtrToStructure<{{ unmanaged_result_ty }}>(data);
        var managed = unmanaged.{{ result_to_managed }}();{% else %}var managed = Marshal.PtrToStructure<{{ result_ty_name }}>(data);{% endif %}
        if (managed.IsOk) { tcs.SetResult({% if is_task_void %}true{% else %}managed.AsOk(){% endif %}); }
        else { tcs.SetException(managed.ExceptionForVariant()); }
    }

    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    internal (AsyncCallbackCommonNative, Task{% if not is_task_void %}<{{ task_inner_ty }}>{% endif %}) NewCall()
    {
        var tcs = new TaskCompletionSource<{% if is_task_void %}bool{% else %}{{ task_inner_ty }}{% endif %}>();
        var id = Interlocked.Increment(ref Id);

        lock (InFlight) { InFlight.TryAdd(id, tcs); }

        var ac = new AsyncCallbackCommonNative {
            _ptr = _callback_ptr,
            _ts = (IntPtr) id,
        };

        return (ac, tcs.Task);
    }
}