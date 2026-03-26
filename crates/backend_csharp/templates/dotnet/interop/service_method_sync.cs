    [UnmanagedCallersOnly(EntryPoint = "{{ ffi_name }}")]
    public static {{ rval_type }} {{ ffi_name }}({{ args }})
    {
        try
        {
            var handle = GCHandle.FromIntPtr(self);
            var obj = (I{{ type_name }}<{{ type_name }}>)handle.Target!;
{%- if is_void %}
            obj.{{ method_name }}({{ forward }});
{%- else %}
            return obj.{{ method_name }}({{ forward }}){{ rval_suffix }};
{%- endif %}
        }
        catch (Exception e)
        {
            Trampoline.UncaughtException(e.ToString());
{%- if not is_void %}
            return default;
{%- endif %}
        }
    }
