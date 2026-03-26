    [UnmanagedCallersOnly(EntryPoint = "{{ ffi_name }}")]
    public static void {{ ffi_name }}({{ args }})
    {
        try
        {
            var handle = GCHandle.FromIntPtr(self);
            var obj = (I{{ type_name }}<{{ type_name }}>)handle.Target!;
            _ = obj.{{ method_name }}({{ forward }}).{{ continuation }};
        }
        catch (Exception e)
        {
            Trampoline.UncaughtException(e.ToString());
        }
    }
