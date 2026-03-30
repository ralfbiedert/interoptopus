    [UnmanagedCallersOnly(EntryPoint = "{{ ffi_name }}")]
    internal static void {{ ffi_name }}({{ args }})
    {
        try
        {
            var handle = GCHandle.FromIntPtr({{ self_expr }});
            handle.Free();
        }
        catch (Exception e)
        {
            Trampoline.UncaughtException(e.ToString());
        }
    }
