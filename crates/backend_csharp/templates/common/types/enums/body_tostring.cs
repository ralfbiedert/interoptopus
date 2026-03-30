{{ _fns_decorators_all }}
public override string ToString()
{
    {%- for v in variants %}
    {%- if v.has_payload %}
    if (_variant == {{ v.id }}) return "{{ v.name }}(...)";
    {%- else %}
    if (_variant == {{ v.id }}) return "{{ v.name }}";
    {%- endif %}
    {%- endfor %}
    throw new InteropException("Illegal enum state detected. This is a severe error and should never happen.");
}
