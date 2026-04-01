    [UnmanagedCallersOnly(EntryPoint = "{{ ffi_name }}")]
    internal static unsafe TaskHandle {{ ffi_name }}({{ args }})
    {
        var cts = new CancellationTokenSource();
        try
        {
{%- if result_wrap_type %}
            _ = {{ result_wrap_type }}.FromCallAsync(() => Plugin.{{ pascal_name }}({{ forward }}{% if forward %}, {% endif %}cts.Token)).{{ continuation }};
{%- else %}
            _ = Plugin.{{ pascal_name }}({{ forward }}{% if forward %}, {% endif %}cts.Token).{{ continuation }};
{%- endif %}
        }
        catch (Exception e)
        {
            Trampoline.UncaughtException(e.ToString());
        }
        return TaskHandle.FromCancellationTokenSource(cts);
    }
