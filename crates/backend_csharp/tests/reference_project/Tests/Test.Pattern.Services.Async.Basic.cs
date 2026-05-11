using My.Company;
using Xunit;

public class TestPatternServicesAsyncBasic
{
    [Fact]
    public async void Create()
    {
        using var s = ServiceAsyncBasic.Simple();
    }

}