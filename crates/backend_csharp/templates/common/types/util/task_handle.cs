[StructLayout(LayoutKind.Sequential)]
{{ visibility }} partial struct TaskHandle
{
    private IntPtr _data;
    private IntPtr _abort_fn;
    private IntPtr _drop_fn;
}

{{ visibility }} partial struct TaskHandle : IDisposable
{
    /// Aborts the associated Rust async task. The spawned future will be
    /// dropped at its next <c>.await</c> point and the completion callback
    /// will fire with a <c>Panic</c> result.
    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal unsafe void Abort()
    {
        if (_abort_fn != IntPtr.Zero && _data != IntPtr.Zero)
        {
            ((delegate* unmanaged[Cdecl]<IntPtr, void>)_abort_fn)(_data);
        }
    }

    /// Frees the native handle resources without aborting the task.
    /// Must be called exactly once when the handle is no longer needed.
    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    public unsafe void Dispose()
    {
        if (_drop_fn != IntPtr.Zero && _data != IntPtr.Zero)
        {
            ((delegate* unmanaged[Cdecl]<IntPtr, void>)_drop_fn)(_data);
            _data = IntPtr.Zero;
            _abort_fn = IntPtr.Zero;
            _drop_fn = IntPtr.Zero;
        }
    }

    /// Creates a <see cref="TaskHandle"/> backed by a <see cref="CancellationTokenSource"/>.
    /// Calling <c>Abort</c> on the returned handle triggers <see cref="CancellationTokenSource.Cancel"/>;
    /// calling <c>Dispose</c> frees the pinned GCHandle.
    {{ _fns_decorators_all | indent }}
    {{ _fns_decorators_internal | indent }}
    internal static unsafe TaskHandle FromCancellationTokenSource(CancellationTokenSource cts)
    {
        var gcHandle = GCHandle.Alloc(cts);
        return new TaskHandle
        {
            _data = GCHandle.ToIntPtr(gcHandle),
            _abort_fn = (IntPtr)(delegate* unmanaged[Cdecl]<IntPtr, void>)&TaskHandleAbort,
            _drop_fn = (IntPtr)(delegate* unmanaged[Cdecl]<IntPtr, void>)&TaskHandleDrop,
        };
    }

    [UnmanagedCallersOnly(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    private static void TaskHandleAbort(IntPtr data)
    {
        var handle = GCHandle.FromIntPtr(data);
        if (handle.Target is CancellationTokenSource cts)
        {
            cts.Cancel();
        }
    }

    [UnmanagedCallersOnly(CallConvs = new[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
    private static void TaskHandleDrop(IntPtr data)
    {
        var handle = GCHandle.FromIntPtr(data);
        if (handle.Target is CancellationTokenSource cts)
        {
            cts.Cancel();
            cts.Dispose();
        }
        handle.Free();
    }
}
