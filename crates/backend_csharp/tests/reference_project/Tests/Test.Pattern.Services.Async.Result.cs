using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternServicesAsyncResult
{
    [Fact]
    public async void Success()
    {
        using var s = ServiceAsyncResult.Create();
        await s.Success();
    }

    [Fact]
    public async void Fail()
    {
        var exceptionThrown = false;
        using var s = ServiceAsyncResult.Create();

        try
        {
            await s.Fail();
        }
        catch (EnumException<Error> e)
        {
            Assert.True(e.Value.IsFail);
            exceptionThrown = true;
        }

        Assert.True(exceptionThrown);
    }
}