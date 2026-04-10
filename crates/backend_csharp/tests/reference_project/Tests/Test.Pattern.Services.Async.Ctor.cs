using My.Company;
using Xunit;

public class TestPatternServicesAsyncCtor
{
    [Fact]
    public async void NewAsyncAndGetValue()
    {
        using var asyncBasic = ServiceAsyncBasic.Create();
        using var asyncCtor = await ServiceAsyncCtor.NewAsync(asyncBasic, 42);
        Assert.Equal(42u, asyncCtor.GetValue());
    }
}