// Ctors
{%- for v in variants %}
{%- if v.docs %}
{{ v.docs }}
{%- endif %}
{%- if v.has_payload %}
public static {{ name }} {{ v.name }}({{ v.type }} value) => new() { _variant = {{ v.id }}, _{{ v.name }} = value };
{%- else %}
public static {{ name }} {{ v.name }} => new() { _variant = {{ v.id }} };
{%- endif %}
{%- endfor %}

// Checks
{%- for v in variants %}
public bool Is{{ v.name }} => _variant == {{ v.id }};
{%- endfor %}

// Conversions
{%- for v in variants %}
{%- if v.has_payload %}
public {{ v.type }} As{{ v.name }}() { if (_variant != {{ v.id }}) { throw ExceptionForVariant(); } else { return _{{ v.name }}; } }
{%- else %}
public void As{{ v.name }}() { if (_variant != {{ v.id }}) throw ExceptionForVariant(); }
{%- endif %}
{%- endfor %}
