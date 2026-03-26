using My.Company;
using My.Company.Common;

namespace My.Company;

partial class Plugin : IPlugin
{
    public static ResultNestedAUint CreateA(uint value)
    {
        throw new NotImplementedException();
    }

    public static Task<ResultNestedAUint> CreateAAsync(uint value)
    {
        throw new NotImplementedException();
    }

    public static ResultUintUint GetValue()
    {
        throw new NotImplementedException();
    }

    public static Task<ResultUintUint> GetValueAsync()
    {
        throw new NotImplementedException();
    }
}

partial class NestedA: INestedA<NestedA>
{
    public static ResultNestedAUint NestedaCreate(uint value)
    {
        throw new NotImplementedException();
    }

    public static Task<ResultNestedAUint> NestedaCreateAsync(uint value)
    {
        throw new NotImplementedException();
    }

    public ResultUintUint NestedaGetValue()
    {
        throw new NotImplementedException();
    }

    public Task<ResultUintUint> NestedaGetValueAsync()
    {
        throw new NotImplementedException();
    }
}