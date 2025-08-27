{#- This is only size computation itself, for entire method body see wire/calculate_size_body.cs -#}
{%- if field.kind == "string" %}
Marshal.SizeOf<ulong>() + System.Text.Encoding.UTF8.GetByteCount(this.{{field.name}} ?? "") /* {{field.kind}} */
{%- elif field.kind == "vec" %}
{% if field.inner_kind == "primitive" or field.inner_kind == "enum" %}
Marshal.SizeOf<ulong>() + this.{{field.name}}.Length * Marshal.SizeOf<{{field.inner_type}}>()
{% else %}
WireInterop.CalculateVariableVecSize(this.{{field.name}}, Serde{{field.inner_type}}Extensions.CalculateSize) /* {{field.kind}} */
{% endif %}
{%- elif field.kind == "map" %}
{% set kv = field.inner_type | split(pat=", ") %}
WireInterop.CalculateVariableMapSize(this.{{field.name}}, Serde{{kv[0]}}Extensions.CalculateSize, Serde{{kv[1]}}Extensions.CalculateSize) /* {{field.kind}} */
{%- elif field.kind == "optional" %}
1 + (this.{{field.name}} != null ? Marshal.SizeOf<{{field.inner_type}}>() : 0) /* {{field.kind}} */
{%- elif field.kind == "enum" %}
Marshal.SizeOf<ulong>() /* {{field.kind}} */
{%- elif field.kind == "primitive" %}
{{field.primitive_size}} /* {{field.kind}} */
{%- else %}
this.{{field.name}}.CalculateSize() /* {{field.kind}} */
{%- endif %}
