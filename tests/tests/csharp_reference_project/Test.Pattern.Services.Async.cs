using My.Company;
using Xunit;


public class TestPatternServicesAsync
{
    [Fact]
    public async void service_async()
    {
        var s = ServiceAsync.New();
        var r = await s.AsyncMock(123);
        Assert.Equal(r, 123u);
    }
}
