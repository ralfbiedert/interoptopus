    [UnmanagedCallersOnly]
    public static nint {{ ffi_name }}({{ args }})
    {
        try
        {
            var obj = {{ type_name }}.{{ method_name }}({{ forward }});
            var handle = GCHandle.Alloc(obj);
            return GCHandle.ToIntPtr(handle);
        }
        catch (Exception e)
        {
            Trampoline.UncaughtException(e.ToString());
            return default;
        }
    }
