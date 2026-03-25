    [UnmanagedCallersOnly]
    public static nint {{ ffi_name }}({{ args }})
    {
        try
        {
            var handle = GCHandle.FromIntPtr(self);
            var obj = (I{{ type_name }}<{{ type_name }}>)handle.Target!;
            var result = obj.{{ method_name }}({{ forward }});
            var resultHandle = GCHandle.Alloc(result);
            return GCHandle.ToIntPtr(resultHandle);
        }
        catch (Exception e)
        {
            Trampoline.UncaughtException(e.ToString());
            return default;
        }
    }
