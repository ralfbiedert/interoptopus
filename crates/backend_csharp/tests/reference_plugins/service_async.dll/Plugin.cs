using My.Company;

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

    public static Task<WireOfHashMapStringString> Wire1(WireOfHashMapStringString x)
    {
        throw new NotImplementedException();
    }

    public static Task<ResultWireOfHashMapStringStringError> Wire2(WireOfHashMapStringString x)
    {
        throw new NotImplementedException();
    }
}

class AsyncBasic: IAsyncBasic<AsyncBasic>
{
    public static AsyncBasic AsyncbasicCreate() => new();
    
    public async Task AsyncbasicCallVoid()
    {
        await Task.Yield();
    }

    public async Task<uint> AsyncbasicAddOne(uint x)
    {
        await Task.Yield();
        return x + 1;
    }

    public Task<WireOfHashMapStringString> AsyncbasicWire1(WireOfHashMapStringString x)
    {
        throw new NotImplementedException();
    }

    public Task<ResultWireOfHashMapStringStringError> AsyncbasicWire2(WireOfHashMapStringString x)
    {
        throw new NotImplementedException();
    }
}