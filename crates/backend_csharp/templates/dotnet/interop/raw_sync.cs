    [UnmanagedCallersOnly(EntryPoint = "{{ ffi_name }}")]
    internal static {{ rval_type }} {{ ffi_name }}({{ args }})
    {
        try
        {
{%- if is_void %}
            Plugin.{{ pascal_name }}({{ forward }}){{ rval_suffix }};
{%- elif result_wrap_type %}
            return {{ result_wrap_type }}.FromCall(() => Plugin.{{ pascal_name }}({{ forward }})){{ rval_suffix }};
{%- else %}
            return Plugin.{{ pascal_name }}({{ forward }}){{ rval_suffix }};
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
