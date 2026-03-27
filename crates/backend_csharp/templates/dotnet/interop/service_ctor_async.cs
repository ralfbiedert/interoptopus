    [UnmanagedCallersOnly(EntryPoint = "{{ ffi_name }}")]
    public static void {{ ffi_name }}({{ args }})
    {
        try
        {
{%- if result_wrap_type %}
            _ = {{ result_wrap_type }}.FromCallAsync(() => {{ type_name }}.{{ method_name }}({{ forward }})).{{ continuation }};
{%- else %}
            _ = {{ type_name }}.{{ method_name }}({{ forward }}).{{ continuation }};
{%- endif %}
        }
        catch (Exception e)
        {
            Trampoline.UncaughtException(e.ToString());
        }
    }
