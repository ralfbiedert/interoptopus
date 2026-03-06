{% set known_types = ["sbyte", "byte", "short", "ushort", "int", "uint", "long", "ulong", "float", "double"] -%}
{% if field.inner_type == "bool" %}
writer.Write((byte)(this.{{field.name}} ? 1 : 0));
{% elif known_types is containing(field.inner_type) %}
writer.Write(({{field.inner_type}})this.{{field.name}});
{% else %}
writer.Write(this.{{field.name}});
{% endif %}
