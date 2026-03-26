using My.Company;
using My.Company.Common;

namespace My.Company;

class Plugin : IPlugin
{
    public static NestedA CreateA(uint value)
    {
        return new NestedA(value);
    }

    public static ResultNestedAError CreateAResult(uint value)
    {
        return ResultNestedAError.Ok(new NestedA(value));
    }
}

public partial class NestedA : INestedA<NestedA>
{
    public uint Value { get; }

    public NestedA() { Value = 0; }
    public NestedA(uint value) { Value = value; }

    public static NestedA Create(uint value)
    {
        return new NestedA(value);
    }

    public uint GetValue() => Value;

    public uint Add(uint x) => Value + x;

    public NestedB CreateOther()
    {
        return new NestedB(Value);
    }

    public NestedB CreateOtherWith(uint extra)
    {
        return new NestedB(Value + extra);
    }
}

public partial class NestedB : INestedB<NestedB>
{
    public uint Value { get; }

    public NestedB() { Value = 0; }
    public NestedB(uint value) { Value = value; }

    public uint GetValue() => Value;

    public uint Add(uint x) => Value + x;

    public uint Accept(NestedA a) => Value + a.Value;

    public uint AcceptRef(NestedA a) => Value + a.Value;
}
