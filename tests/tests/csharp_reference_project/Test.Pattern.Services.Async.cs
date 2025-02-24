using My.Company;
using Xunit;


public class TestPatternServicesAsync
{
    [Fact]
    public async void service_async()
    {
        var s = ServiceAsync.New();
        var r = await s.ReturnAfterMs(123, 500);
        Assert.Equal(r, 123u);
    }
}
