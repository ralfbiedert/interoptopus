    [UnmanagedCallersOnly(EntryPoint = "{{ ffi_name }}")]
    internal static nint {{ ffi_name }}({{ args }})
    {
        try
        {
            var obj = {{ type_name }}.{{ method_name }}({{ forward }});
            var h = GCHandle.Alloc(obj);
            return GCHandle.ToIntPtr(h);
        }
        catch (Exception e)
        {
            Trampoline.UncaughtException(e.ToString());
            return default;
        }
    }
