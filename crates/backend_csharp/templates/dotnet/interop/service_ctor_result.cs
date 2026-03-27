    [UnmanagedCallersOnly(EntryPoint = "{{ ffi_name }}")]
    public static {{ rval_type }} {{ ffi_name }}({{ args }})
    {
        try
        {
{%- if result_wrap_type %}
            return {{ result_wrap_type }}.FromCall(() => {{ type_name }}.{{ method_name }}({{ forward }})){{ rval_suffix }};
{%- else %}
            return {{ type_name }}.{{ method_name }}({{ forward }}){{ rval_suffix }};
{%- endif %}
        }
        catch (Exception e)
        {
            Trampoline.UncaughtException(e.ToString());
            return default;
        }
    }
