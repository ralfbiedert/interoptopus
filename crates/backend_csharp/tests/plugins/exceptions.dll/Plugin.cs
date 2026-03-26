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
        return Task.FromResult(ResultNestedAUint.Ok(new NestedA()));
    }

    public static ResultUintUint GetValue()
    {
        return ResultUintUint.Ok(42);
    }

    public static Task<ResultUintUint> GetValueAsync()
    {
        throw new NotImplementedException();
    }
}

partial class NestedA: INestedA<NestedA>
{
    public static ResultNestedAUint Create(uint value)
    {
        throw new NotImplementedException();
    }

    public static Task<ResultNestedAUint> CreateAsync(uint value)
    {
        throw new NotImplementedException();
    }

    public ResultUintUint GetValue()
    {
        throw new NotImplementedException();
    }

    public Task<ResultUintUint> GetValueAsync()
    {
        throw new NotImplementedException();
    }
}