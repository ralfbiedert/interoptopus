fixed ({{ element_type }}* _fixed = {{ field }})
{
    _managed.{{ field }} = new {{ element_type }}[{{ len }}];
    var src = new ReadOnlySpan<{{ element_type }}>(_fixed, {{ len }});
    var dst = new Span<{{ element_type }}>(_managed.{{ field }}, 0, {{ len }});
    src.CopyTo(dst);
}