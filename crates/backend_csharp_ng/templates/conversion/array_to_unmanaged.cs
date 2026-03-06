{
    if ({{ field }} == null) { throw new InvalidOperationException("Array '{{ field }}' must not be null"); }
    if ({{ field }}.Length != {{ len }}) { throw new InvalidOperationException("Array size mismatch for '{{ field }}'"); }
    var src = new ReadOnlySpan<{{ element_type }}>({{ field }}, 0, {{ len }});
    fixed ({{ element_type }}* _fixed = _unmanaged.{{ field }})
    {
        var dst = new Span<{{ element_type }}>(_fixed, {{ len }});
        src.CopyTo(dst);
    }
}