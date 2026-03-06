fixed(ushort* _fixed = {{ field }})
{
    _managed.{{ field }} = new {{ type }}[{{ len }}];
    var src = new ReadOnlySpan<{{ type }}>(_fixed, {{ len }});
    var dst = new Span<{{ type }}>(_managed.{{ field }}, 0, {{ len }});
    src.CopyTo(dst);
}
