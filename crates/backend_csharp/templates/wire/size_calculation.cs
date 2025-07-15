int size = 0;

{%- for field in fields %}
{%- if field.kind == "string" %}
size += 8 + System.Text.Encoding.UTF8.GetByteCount(this.{{field.name}} ?? ""); /* {{field.kind}} */
{%- elif field.kind == "vec" %}
size += WireInterop.CalculateVecSize(this.{{field.name}}); /* {{field.kind}} */
{%- elif field.kind == "optional" %}
size += 1 + (this.{{field.name}}.HasValue ? Marshal.SizeOf<{{field.inner_type}}>() : 0); /* {{field.kind}} */
{%- elif field.kind == "enum" %}
size += 8; /* {{field.kind}} */
{%- elif field.kind == "primitive" %}
size += {{field.primitive_size}}; /* {{field.kind}} */
{%- else %}
size += this.{{field.name}}.CalculateSize(); /* {{field.kind}} */
{%- endif %}
{%- endfor %}

return size;
