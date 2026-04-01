    [UnmanagedCallersOnly(EntryPoint = "{{ ffi_name }}")]
    internal static unsafe TaskHandle {{ ffi_name }}({{ args }})
    {
        var cts = new CancellationTokenSource();
        try
        {
            var handle = GCHandle.FromIntPtr(self);
            var obj = (I{{ type_name }}<{{ type_name }}>)handle.Target!;
{%- if result_wrap_type %}
            _ = {{ result_wrap_type }}.FromCallAsync(() => obj.{{ method_name }}({{ forward }}{% if forward %}, {% endif %}cts.Token)).{{ continuation }};
{%- else %}
            _ = obj.{{ method_name }}({{ forward }}{% if forward %}, {% endif %}cts.Token).{{ continuation }};
{%- endif %}
        }
        catch (Exception e)
        {
            Trampoline.UncaughtException(e.ToString());
        }
        return TaskHandle.FromCancellationTokenSource(cts);
    }
