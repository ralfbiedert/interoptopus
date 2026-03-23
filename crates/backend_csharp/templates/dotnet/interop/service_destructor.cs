    [UnmanagedCallersOnly]
    public static void {{ ffi_name }}(nint self)
    {
        try
        {
            var handle = GCHandle.FromIntPtr(self);
            handle.Free();
        }
        catch (Exception e)
        {
            Trampoline.UncaughtException(e.ToString());
        }
    }
