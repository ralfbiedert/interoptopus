public enum {{name}}
{
{% for variant in variants %}
    {{variant.tag}} = {{variant.value}},
{% endfor %}
}

public static class Wire{{name}}Extensions
{
    [MethodImpl(MethodImplOptions.AggressiveOptimization)]
    public static void Serialize(this {{name}} value, BinaryWriter writer) {
        writer.Write((int)value); /* only works for unit enum variants */
    }
}
