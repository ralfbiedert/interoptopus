using System;
using System.Linq;
using System.Threading.Tasks;
using My.Company;
using My.Company.Common;
using Xunit;
using Array = My.Company.Array;


public class TestPatternServicesAsyncResult
{
    [Fact]
    public async void Success()
    {
        var s = ServiceAsyncResult.New();
        await s.Success();
        s.Dispose();
    }

    [Fact]
    public async void Fail()
    {
        var exceptionThrown = false;
        var s = ServiceAsyncResult.New();

        try { await s.Fail(); }
        catch (EnumException<Error> e)
        {
            Assert.True(e.Value.IsFail);
            exceptionThrown = true;
        }
        s.Dispose();

        Assert.True(exceptionThrown);
    }
}
