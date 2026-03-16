[Serializable]
[StructLayout(LayoutKind.Sequential)]
public partial struct Bool
{
    byte value;
}

public partial struct Bool
{
    public static readonly Bool True = new Bool { value = 1 };
    public static readonly Bool False = new Bool { value = 0 };

    public Bool(bool b)
    {
        value = (byte)(b ? 1 : 0);
    }

    public bool Is => value == 1;

    public static implicit operator Bool(bool b) => new Bool(b);
    public static implicit operator bool(Bool b) => b.Is;
}
