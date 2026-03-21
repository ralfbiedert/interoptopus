using My.Company;

namespace My.Company;

class AsyncBasic: IAsyncBasic<AsyncBasic>
{
    public static AsyncBasic AsyncbasicCreate()
    {
        return  new AsyncBasic();
    }

    public void AsyncbasicRaw(uint x, AsyncCallbackCommonNative cb)
    {
    }
}