{{ visibility }} class EnumException() : InteropException($"Enum variant mismatch.")  { }

{{ visibility }} class EnumException<T>(T t) : InteropException($"Enum variant mismatch.")
{
    public T Value { get; } = t;
}
