using My.Company;
using My.Company.Common;

namespace My.Company;

class Plugin : IPlugin
{

    public static async Task<uint> AddOne(uint x)
    {
        await Task.Yield();
        return x + 1;
    }

    public static async Task CallVoid()
    {
        await Task.Yield();
    }

    public static async Task<WireOfHashMapStringString> Wire1(WireOfHashMapStringString x)
    {
        await Task.Yield();
        var dictionary = x.Unwire();
        dictionary["hello"] = "world";
        return WireOfHashMapStringString.From(dictionary);
    }

    public static async Task<ResultWireOfHashMapStringStringError> Wire2(WireOfHashMapStringString x)
    {
        await Task.Yield();
        var dictionary = x.Unwire();
        dictionary["hello"] = "world";
        var wire = WireOfHashMapStringString.From(dictionary);
        return ResultWireOfHashMapStringStringError.Ok(wire);
    }
}

partial class AsyncBasic: IAsyncBasic<AsyncBasic>
{
    public static AsyncBasic Create() => new();
    
    public async Task CallVoid()
    {
        await Task.Yield();
    }

    public async Task<uint> AddOne(uint x)
    {
        await Task.Yield();
        return x + 1;
    }

    public async Task<WireOfHashMapStringString> Wire1(WireOfHashMapStringString x)
    {
        await Task.Yield();
        var dictionary = x.Unwire();
        dictionary["hello"] = "world";
        return WireOfHashMapStringString.From(dictionary);
    }

    public async Task<ResultWireOfHashMapStringStringError> Wire2(WireOfHashMapStringString x)
    {
        await Task.Yield();
        var dictionary = x.Unwire();
        dictionary["hello"] = "world";
        var wire = WireOfHashMapStringString.From(dictionary);
        return ResultWireOfHashMapStringStringError.Ok(wire);
    }
}