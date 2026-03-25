    [UnmanagedCallersOnly]
    public static {{ rval_type }} {{ ffi_name }}({{ args }})
    {
        try
        {
            var handle = GCHandle.FromIntPtr(self);
            var obj = (I{{ type_name }}<{{ type_name }}>)handle.Target!;
            return obj.{{ method_name }}({{ forward }}).IntoUnmanaged();
        }
        catch (Exception e)
        {
            Trampoline.UncaughtException(e.ToString());
            return default;
        }
    }
