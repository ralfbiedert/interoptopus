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
}
