using System.Threading.Tasks;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternServicesAsyncResult
{
    [Fact]
    public async Task Success()
    {
        using var s = ServiceAsyncResult.Create();
        await s.Success(TestContext.Current.CancellationToken);
    }

    [Fact]
    public async Task Fail()
    {
        var exceptionThrown = false;
        using var s = ServiceAsyncResult.Create();

        try
        {
            await s.Fail(TestContext.Current.CancellationToken);
        }
        catch (EnumException<Error> e)
        {
            Assert.True(e.Value.IsFail);
            exceptionThrown = true;
        }

        Assert.True(exceptionThrown);
    }
}
