{{ visibility }} class EnumException : InteropException {
    public EnumException() : base($"Enum variant mismatch.")  { }
}

{{ visibility }} class EnumException<T> : InteropException
{
    public T Value { get; }

    public EnumException(T t) : base("Enum variant mismatch.")
    {
        Value = t;
    }
}
