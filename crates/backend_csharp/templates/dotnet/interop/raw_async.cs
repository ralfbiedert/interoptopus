    [UnmanagedCallersOnly]
    public static void {{ ffi_name }}({{ args }})
    {
        try
        {
            _ = Plugin.{{ pascal_name }}({{ forward }}).{{ continuation }};
        }
        catch (Exception e)
        {
            Trampoline.UncaughtException(e.ToString());
        }
    }
