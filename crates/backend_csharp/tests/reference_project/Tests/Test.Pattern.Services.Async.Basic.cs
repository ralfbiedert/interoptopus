using System.Threading.Tasks;
using My.Company;
using Xunit;

public class TestPatternServicesAsyncBasic
{
    [Fact]
    public async Task Create()
    {
        using var s = ServiceAsyncBasic.Simple();
    }

}