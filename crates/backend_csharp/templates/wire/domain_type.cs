/// <summary>
/// Domain type that is sent across FFI boundary in a Wire.
/// </summary>
{%- if docs %}
///
{%- for line in docs %}
/// {{line}}
{%- endfor %}
///
{%- endif %}
{{visibility}} {{self_kind}} {{type}}
{
{%- for field in fields %}
    /// {{field.docs}}
    {{field.visibility}}{{field.type_name}} {{field.name}};
{%- endfor %}

    /// <summary>Empty constructor</summary>
    public {{type}}() { }

{%- if fields %}

    /// <summary>Member-wise initializing constructor</summary>
    public {{type}}({% for field in fields %}{{field.type_name}} {{field.name}}{% if not loop.last %}, {% endif %}{% endfor %})
    {
{%- for field in fields %}
        this.{{field.name}} = {{field.name}};
{%- endfor %}
    }
{%- endif %}

    public override string ToString()
    {
        return "{{type}} { {% for field in fields %}{{field.name}} = " + {{field.name}}{% if not loop.last %} + ", {% endif %}{% endfor %} + " }";
    }

    // TODO: add serialize/deserialize here
    /// <summary>
    /// Deserialize the wire data back to a managed {{type}} object
    /// </summary>
    public static {{type}} Deserialize(BinaryReader reader)
    {
{%- if fields %}
{{deserialization_code}}
{%- else -%}
        return new {{type}}();
{%- endif %}
    }

    /// <summary>
    /// Serialize a {{type}} object into this wire's buffer
    /// </summary>
    public void Serialize(BinaryWriter writer)
    {
{%- if fields %}
{{serialization_code}}
{%- endif %}
    }

    /// <summary>
    /// Calculate the size needed to serialize a {{type}} object
    /// </summary>
    public int CalculateSize()
    {
{% if fields %}
{{size_calculation}}
{%- else -%}
        return 0;
{%- endif %}
    }
}

/// <summary>
/// Extension methods for {{type}} to Serialize/Deserialize instances
/// </summary>
public static class Serde{{type}}Extensions
{
    public static void Serialize{{type}}(BinaryWriter writer, {{type}} item)
    {
        item.Serialize(writer); {# This is used as a callback Func #}
    }

    public static {{type}} Deserialize{{type}}(BinaryReader reader)
    {
        return {{type}}.Deserialize(reader); {# This is used as a callback Func #}
    }

    public static int CalculateSize({{type}} value)
    {
        return value.CalculateSize(); {# This is used as a callback Func #}
    }
}
