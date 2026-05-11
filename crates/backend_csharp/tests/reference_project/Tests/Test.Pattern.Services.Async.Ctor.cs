using My.Company;
using Xunit;

public class TestPatternServicesAsyncCtor
{
    [Fact]
    public async void NewAsync()
    {
        using var asyncBasic = ServiceAsyncBasic.Simple();
        using var asyncCtor = await ServiceAsyncCtor.NewAsync(asyncBasic, 42);
        Assert.Equal(42u, asyncCtor.GetValue());
    }
}