using My.Company;

namespace My.Company;

class Plugin : IPlugin
{
    public async static Task<uint> Raw1()
    {
        await Task.Yield();
        return 123;
    }
}

class AsyncBasic: IAsyncBasic<AsyncBasic>
{
    public static AsyncBasic AsyncbasicCreate() => new();

    public async Task<uint> AsyncbasicRaw(uint x)
    {
        await Task.Yield();
        return x;
    }

    public async Task<uint> AsyncbasicRaw2()
    {
        await Task.Yield();
        return 123;
    }
}