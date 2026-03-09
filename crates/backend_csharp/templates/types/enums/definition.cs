public partial {{ struct_or_class }} {{ name }}
{
    uint _variant;
    {%- for variant in variants %}
    {{ variant.unmanaged_name }} _{{ variant.name }};
    {%- endfor %}
}
