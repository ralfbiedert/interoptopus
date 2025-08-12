{%- for field in fields %}
{%- if field.kind == "string" %}
this.{{field.name}}.Serialize(writer); /* {{field.kind}} */
{%- elif field.kind == "vec" %}
WireInterop.SerializeVec(writer, this.{{field.name}}); /* {{field.kind}} */
{%- elif field.kind == "map" %}
WireInterop.SerializeMap(writer, this.{{field.name}}); /* {{field.kind}} */
{%- elif field.kind == "optional" %}
WireInterop.SerializeOptional<{{field.inner_type}}>(writer, this.{{field.name}});  /* {{field.kind}} */
{%- elif field.kind == "primitive" %}
{% include "wire/serialize_primitive.cs" %}
{%- else %}
this.{{field.name}}.Serialize(writer); /* {{field.kind}} */
{%- endif %}
{%- endfor %}
