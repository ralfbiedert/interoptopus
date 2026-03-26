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

    public static NestedA NestedaCreate(uint value)
    {
        return new NestedA(value);
    }

    public uint NestedaGetValue() => Value;

    public uint NestedaAdd(uint x) => Value + x;

    public NestedB NestedaCreateOther()
    {
        return new NestedB(Value);
    }

    public NestedB NestedaCreateOtherWith(uint extra)
    {
        return new NestedB(Value + extra);
    }
}

public partial class NestedB : INestedB<NestedB>
{
    public uint Value { get; }

    public NestedB() { Value = 0; }
    public NestedB(uint value) { Value = value; }

    public uint NestedbGetValue() => Value;

    public uint NestedbAdd(uint x) => Value + x;

    public uint NestedbAccept(NestedA a) => Value + a.Value;

    public uint NestedbAcceptRef(NestedA a) => Value + a.Value;
}
