using My.Company;
using Xunit;

public class TestPatternServicesAsyncBasic
{
    [Fact]
    public async void Call()
    {
        var s = ServiceAsyncBasic.Create();
        await s.Call();
    }
}