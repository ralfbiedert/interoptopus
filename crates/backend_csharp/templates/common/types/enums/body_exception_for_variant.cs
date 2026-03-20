[MethodImpl(MethodImplOptions.AggressiveOptimization)]
public Exception ExceptionForVariant()
{
    {%- for v in variants %}
    {%- if v.has_payload %}
    if (_variant == {{ v.id }}) return new EnumException<{{ v.type }}>(_{{ v.name }});
    {%- else %}
    if (_variant == {{ v.id }}) return new EnumException();
    {%- endif %}
    {%- endfor %}
    throw new InteropException("Illegal enum state detected. This is a severe error and should never happen.");
}
