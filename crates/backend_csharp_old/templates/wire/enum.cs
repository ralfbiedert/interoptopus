public enum {{name}}
{
{% for variant in variants %}
    {{variant.tag}} = {{variant.value}},
{% endfor %}
}

public static class Wire{{name}}Extensions
{
    {# this makes code slower, do NOT enable [MethodImpl(MethodImplOptions.AggressiveOptimization)] -#}
    public static void Serialize(this {{name}} value, BinaryWriter writer) {
        writer.Write((int)value); /* only works for unit enum variants */
    }
}
