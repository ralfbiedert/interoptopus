
public class EnumException() : InteropException($"Enum variant mismatch.")  { }

public class EnumException<T>(T t) : InteropException($"Enum variant mismatch.")
{
    public T Value { get; } = t;
}

public class InteropException(string text) : Exception(text) { }
