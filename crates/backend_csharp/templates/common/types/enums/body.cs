{%- if not is_managed_only -%}
[NativeMarshalling(typeof(MarshallerMeta))]
{% endif -%}
public partial {{ struct_or_class }} {{ name }}{% if is_disposable %} : IDisposable{% endif %}
{
{%- if not is_managed_only %}
    {%- for item in unmanaged_variants %}
    {{ item | indent }}
    {% endfor %}

    {{ unmanaged | indent }}

    {{ to_unmanaged | indent }}

    {{ as_unmanaged | indent }}
{% endif %}
    {{ exception_for_variant | indent }}

    {{ ctors | indent }}
{% if from_call %}
    {{ from_call | indent }}
{% endif %}
    {{ to_string | indent }}
{% if is_disposable %}
    public void Dispose()
    {
        {%- for v in disposable_variants %}
        if (_variant == {{ v.tag }}) {{ v.name }}?.Dispose();
        {%- endfor %}
    }
{% endif -%}
{%- if not is_managed_only %}
    [CustomMarshaller(typeof({{ name }}), MarshalMode.Default, typeof(Marshaller))]
    private struct MarshallerMeta { }

    internal ref struct Marshaller
    {
        private {{ name }} _managed;
        private Unmanaged _unmanaged;

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Marshaller({{ name }} managed) { _managed = managed; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Marshaller(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void FromManaged({{ name }} managed) { _managed = managed; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void FromUnmanaged(Unmanaged unmanaged) { _unmanaged = unmanaged; }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public Unmanaged ToUnmanaged() { return _managed.{{ marshaller_to_unmanaged }}(); }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public {{ name }} ToManaged() { return _unmanaged.{{ marshaller_to_managed }}(); }

        [MethodImpl(MethodImplOptions.AggressiveOptimization)]
        public void Free() {}
    }

{% endif -%}
}
