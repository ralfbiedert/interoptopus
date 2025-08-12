{#- This is only size computation itself, for method body see wire/calculate_size_body.cs -#}
{%- if field.kind == "string" %}
8 + System.Text.Encoding.UTF8.GetByteCount(this.{{field.name}} ?? ""); /* {{field.kind}} */
{%- elif field.kind == "vec" %}
WireInterop.CalculateVecSize(this.{{field.name}}); /* {{field.kind}} */
{%- elif field.kind == "optional" %}
1 + (this.{{field.name}} != null ? Marshal.SizeOf<{{field.inner_type}}>() : 0); /* {{field.kind}} */
{%- elif field.kind == "enum" %}
8; /* {{field.kind}} */
{%- elif field.kind == "primitive" %}
{{field.primitive_size}}; /* {{field.kind}} */
{%- else %}
this.{{field.name}}.CalculateSize(); /* {{field.kind}} */
{%- endif %}
