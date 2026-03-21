using My.Company;

namespace My.Company;

class Plugin : IPlugin
{
    public static void Raw1(AsyncCallbackCommonNative cb)
    {
        throw new NotImplementedException();
    }
}

class AsyncBasic: IAsyncBasic<AsyncBasic>
{
    public static AsyncBasic AsyncbasicCreate()
    {
        return  new AsyncBasic();
    }

    public void AsyncbasicRaw(uint x, AsyncCallbackCommonNative cb)
    {
        
    }

    public void AsyncbasicRaw2(AsyncCallbackCommonNative cb)
    {
        cb.UnsafeComplete();
    }
}