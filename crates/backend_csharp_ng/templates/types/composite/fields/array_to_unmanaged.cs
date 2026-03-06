{
    if ({{ field }} == null) { throw new InvalidOperationException("Array '{{ field }}' must not be null"); }
    if ({{ field }}.Length != {{ len }}) { throw new InvalidOperationException("Array size mismatch for 'field'"); }
    var src = new ReadOnlySpan<{{ type }}>({{ field }}, 0, {{ len });
    var dst = new Span<{{ type }}>(_unmanaged.{{ field }}, {{ len });
    src.CopyTo(dst);
}
