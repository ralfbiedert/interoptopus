    [UnmanagedCallersOnly(EntryPoint = "{{ ffi_name }}")]
    public static {{ rval_type }} {{ ffi_name }}({{ args }})
    {
        try
        {
            return {{ type_name }}.{{ method_name }}({{ forward }}){{ rval_suffix }};
        }
        catch (Exception e)
        {
            Trampoline.UncaughtException(e.ToString());
            return default;
        }
    }
