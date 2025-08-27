{% if field.inner_type == "bool" %}
{{field.name}} = reader.ReadByte() != 0,
{% elif field.inner_type == "sbyte" %}
{{field.name}} = reader.ReadSByte(),
{% elif field.inner_type == "byte" %}
{{field.name}} = reader.ReadByte(),
{% elif field.inner_type == "short" %}
{{field.name}} = reader.ReadInt16(),
{% elif field.inner_type == "ushort" %}
{{field.name}} = reader.ReadUInt16(),
{% elif field.inner_type == "int" %}
{{field.name}} = reader.ReadInt32(),
{% elif field.inner_type == "uint" %}
{{field.name}} = reader.ReadUInt32(),
{% elif field.inner_type == "long" %}
{{field.name}} = reader.ReadInt64(),
{% elif field.inner_type == "ulong" %}
{{field.name}} = reader.ReadUInt64(),
{% elif field.inner_type == "float" %}
{{field.name}} = reader.ReadSingle(),
{% elif field.inner_type == "double" %}
{{field.name}} = reader.ReadDouble(),
{% else %}
{{field.name}} = reader.ReadBytes(1)[0],
{% endif %}
