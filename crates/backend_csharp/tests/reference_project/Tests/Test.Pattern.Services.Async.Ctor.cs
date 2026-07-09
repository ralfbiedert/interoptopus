using System.Threading.Tasks;
using My.Company;
using Xunit;

public class TestPatternServicesAsyncCtor
{
    [Fact]
    public async Task NewAsync()
    {
        using var asyncBasic = ServiceAsyncBasic.Simple();
        using var asyncCtor = await ServiceAsyncCtor.NewAsync(asyncBasic, 42, TestContext.Current.CancellationToken);
        Assert.Equal(42u, asyncCtor.GetValue());
    }
}
