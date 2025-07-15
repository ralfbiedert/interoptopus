{% if field.deser_type == "bool" %}
{{field.name}} = reader.ReadByte() != 0,
{% elif field.deser_type == "sbyte" %}
{{field.name}} = reader.ReadSByte(),
{% elif field.deser_type == "byte" %}
{{field.name}} = reader.ReadByte(),
{% elif field.deser_type == "short" %}
{{field.name}} = reader.ReadInt16(),
{% elif field.deser_type == "ushort" %}
{{field.name}} = reader.ReadUInt16(),
{% elif field.deser_type == "int" %}
{{field.name}} = reader.ReadInt32(),
{% elif field.deser_type == "uint" %}
{{field.name}} = reader.ReadUInt32(),
{% elif field.deser_type == "long" %}
{{field.name}} = reader.ReadInt64(),
{% elif field.deser_type == "ulong" %}
{{field.name}} = reader.ReadUInt64(),
{% elif field.deser_type == "float" %}
{{field.name}} = reader.ReadSingle(),
{% elif field.deser_type == "double" %}
{{field.name}} = reader.ReadDouble(),
{% else %}
{{field.name}} = reader.ReadBytes(1)[0],
{% endif %}
