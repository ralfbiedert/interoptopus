using System.Threading.Tasks;
using My.Company;
using Xunit;

public class TestPatternServicesAsyncCtor
{
    [Fact]
    public async void NewAsyncAndGetValue()
    {
        var asyncBasic = ServiceAsyncBasic.Create();
        var asyncCtor = await ServiceAsyncCtor.NewAsync(asyncBasic.Context, 42);
        Assert.Equal(42u, asyncCtor.GetValue());
        asyncCtor.Dispose();
        asyncBasic.Dispose();
    }
}
