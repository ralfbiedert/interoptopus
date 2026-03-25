    [UnmanagedCallersOnly]
    public static void {{ ffi_name }}({{ args }})
    {
        try
        {
            _ = {{ type_name }}.{{ method_name }}({{ forward }}).{{ continuation }};
        }
        catch (Exception e)
        {
            Trampoline.UncaughtException(e.ToString());
        }
    }
