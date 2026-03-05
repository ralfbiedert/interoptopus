public partial {{ struct_or_class }} {{ name }}
{
    uint _variant;
    {% for variant in variants %}
    {{ variant.type }} {{ variant.name }};
    {% endfor %}
}
