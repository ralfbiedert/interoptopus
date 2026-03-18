using System.Threading.Tasks;
using My.Company;
using Xunit;

public class TestPatternServicesAsyncCtor
{
    [Fact]
    public async void NewAsyncAndGetValue()
    {
        var basic = ServiceAsyncBasic.Create();
        var ctor = await ServiceAsyncCtor.NewAsync(basic.Context, 42);
        Assert.Equal(42u, ctor.GetValue());
        ctor.Dispose();
        basic.Dispose();
    }
}
