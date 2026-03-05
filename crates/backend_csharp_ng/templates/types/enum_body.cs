public partial {{ struct_or_class }} {{ name }}{% if struct_or_class == "class" %} : IDisposable{% endif %}
{
    {%- for item in unmanaged_variants %}
    {{ item | indent }}
    {% endfor %}

    {{ unmanaged }}
}
