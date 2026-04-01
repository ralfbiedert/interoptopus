using System.Threading;
using My.Company;
using My.Company.Common;

namespace My.Company;

class Plugin : IPlugin
{

    public static async Task<uint> AddOne(uint x, CancellationToken ct)
    {
        await Task.Yield();
        return x + 1;
    }

    public static async Task CallVoid(CancellationToken ct)
    {
        await Task.Yield();
    }

    public static async Task<WireOfHashMapStringString> Wire1(WireOfHashMapStringString x, CancellationToken ct)
    {
        await Task.Yield();
        var dictionary = x.Unwire();
        dictionary["hello"] = "world";
        return dictionary.Wire();
    }

    public static async Task<ResultWireOfHashMapStringStringError> Wire2(WireOfHashMapStringString x, CancellationToken ct)
    {
        await Task.Yield();
        var dictionary = x.Unwire();
        dictionary["hello"] = "world";
        var wire = dictionary.Wire();
        return ResultWireOfHashMapStringStringError.Ok(wire);
    }
}

partial class AsyncBasic: IAsyncBasic<AsyncBasic>
{
    public static AsyncBasic Create() => new();
    
    public async Task CallVoid(CancellationToken ct)
    {
        await Task.Yield();
    }

    public async Task<uint> AddOne(uint x, CancellationToken ct)
    {
        await Task.Yield();
        return x + 1;
    }

    public async Task<WireOfHashMapStringString> Wire1(WireOfHashMapStringString x, CancellationToken ct)
    {
        await Task.Yield();
        var dictionary = x.Unwire();
        dictionary["hello"] = "world";
        return dictionary.Wire();
    }

    public async Task<ResultWireOfHashMapStringStringError> Wire2(WireOfHashMapStringString x, CancellationToken ct)
    {
        await Task.Yield();
        var dictionary = x.Unwire();
        dictionary["hello"] = "world";
        var wire = dictionary.Wire();
        return ResultWireOfHashMapStringStringError.Ok(wire);
    }
}