{%- macro de_fn(type) -%}
{%- if type == "String" -%}
WireInterop.DeserializeString
{%- else -%}
Serde{{type}}Extensions.Deserialize{{type}}
{%- endif -%}
{%- endmacro de_fn -%}

return new {{type}} {
    {%- for field in fields %}
    {%- if field.kind == "string" %}
    {{field.name}} = WireInterop.DeserializeString(reader), /* {{field.kind}} */
    {%- elif field.kind == "vec" %}
    {% if field.inner_type == "byte" -%}
    {{field.name}} = WireInterop.DeserializeVecOfByte(reader), /* {{field.kind}} */
    {%- else -%}
    {{field.name}} = WireInterop.DeserializeVec<{{field.inner_type}}>(reader, {{ self::de_fn(type=field.inner_type) }}), /* {{field.kind}} */
    {% endif %}
    {%- elif field.kind == "map" %}
    {% set kv = field.inner_type | split(pat=", ") %}
    {{field.name}} = WireInterop.DeserializeMap<{{field.inner_type}}>(reader, {{ self::de_fn(type=kv[0]) }}, {{ self::de_fn(type=kv[1]) }}), /* {{field.kind}} */
    {%- elif field.kind == "optional" %}
    {{field.name}} = WireInterop.DeserializeOptional(reader, {{ self::de_fn(type=field.inner_type) }}), /* {{field.kind}} */
    {%- elif field.kind == "enum" %}
    {{field.name}} = WireInterop.DeserializeEnum<{{field.inner_type}}>(reader), /* {{field.kind}} */
    {%- elif field.kind == "primitive" %}
    {% include "wire/deserialize_primitive.cs" %}
    {%- else %}
    {{field.name}} = {{ self::de_fn(type=field.inner_type) }}(reader), /* {{field.kind}} */
    {%- endif %}
    {%- endfor %}
};
