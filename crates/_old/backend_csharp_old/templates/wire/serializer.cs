{%- macro ser_fn(type) -%}
{%- if type == "String" -%}
WireInterop.SerializeString
{%- else -%}
Serde{{type}}Extensions.Serialize{{type}}
{%- endif -%}
{%- endmacro ser_fn -%}

{%- for field in fields %}
{%- if field.kind == "string" %}
this.{{field.name}}.Serialize(writer); /* {{field.kind}} */
{%- elif field.kind == "vec" %}
{% if field.inner_type == "byte" -%}
WireInterop.SerializeVecOfByte(writer, this.{{field.name}}); /* {{field.kind}} */
{%- else -%}
WireInterop.SerializeVec(writer, this.{{field.name}}, {{ self::ser_fn(type=field.inner_type) }}); /* {{field.kind}} */
{% endif %}
{%- elif field.kind == "map" %}
{% set kv = field.inner_type | split(pat=", ") %}
WireInterop.SerializeMap(writer, this.{{field.name}}, {{ self::ser_fn(type=kv[0]) }}, {{ self::ser_fn(type=kv[1]) }}); /* {{field.kind}} */
{%- elif field.kind == "optional" %}
WireInterop.SerializeOptional<{{field.inner_type}}>(writer, this.{{field.name}}, {{ self::ser_fn(type=field.inner_type) }});  /* {{field.kind}} */
{%- elif field.kind == "primitive" %}
{% include "wire/serialize_primitive.cs" %}
{%- else %}
this.{{field.name}}.Serialize(writer); /* {{field.kind}} */
{%- endif %}
{%- endfor %}
