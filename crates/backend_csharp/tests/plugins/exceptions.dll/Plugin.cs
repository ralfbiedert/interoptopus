using My.Company;
using My.Company.Common;

namespace My.Company;

partial class Plugin : IPlugin
{
    public static ResultNestedAUint CreateA(uint value)
    {
        throw new NotImplementedException();
    }

    public static ResultUintUint GetValue()
    {
        throw new NotImplementedException();
    }
}

partial class NestedA: INestedA<NestedA>
{
    public static NestedA NestedaCreate(uint value)
    {
        throw new NotImplementedException();
    }

    public ResultUintUint NestedaGetValue()
    {
        throw new NotImplementedException();
    }
}