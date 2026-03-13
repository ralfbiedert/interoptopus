using System;
using System.Linq;
using System.Threading.Tasks;
using My.Company;
using My.Company.Common;
using Xunit;
using Array = My.Company.Array;


public class TestPatternServicesAsyncBasic
{
    [Fact]
    public async void Call()
    {
        var s = ServiceAsyncBasic.New();
        await s.Call();
    }
}
