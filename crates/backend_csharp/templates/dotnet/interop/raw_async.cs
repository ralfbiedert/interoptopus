    [UnmanagedCallersOnly(EntryPoint = "{{ ffi_name }}")]
    internal static void {{ ffi_name }}({{ args }})
    {
        try
        {
{%- if result_wrap_type %}
            _ = {{ result_wrap_type }}.FromCallAsync(() => Plugin.{{ pascal_name }}({{ forward }})).{{ continuation }};
{%- else %}
            _ = Plugin.{{ pascal_name }}({{ forward }}).{{ continuation }};
{%- endif %}
        }
        catch (Exception e)
        {
            Trampoline.UncaughtException(e.ToString());
        }
    }
