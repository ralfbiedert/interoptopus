    [UnmanagedCallersOnly(EntryPoint = "{{ ffi_name }}")]
    public static void {{ ffi_name }}({{ args }})
    {
        try
        {
            var handle = GCHandle.FromIntPtr(self);
            var obj = (I{{ type_name }}<{{ type_name }}>)handle.Target!;
{%- if result_wrap_type %}
            _ = {{ result_wrap_type }}.FromCallAsync(() => obj.{{ method_name }}({{ forward }})).{{ continuation }};
{%- else %}
            _ = obj.{{ method_name }}({{ forward }}).{{ continuation }};
{%- endif %}
        }
        catch (Exception e)
        {
            Trampoline.UncaughtException(e.ToString());
        }
    }
