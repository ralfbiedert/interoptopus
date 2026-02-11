using System;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;
using My.Company;
using My.Company.Common;
using Xunit;
using Array = My.Company.Array;


public class TestPatternServicesAsyncNew
{
    [Fact]
    public async void Call()
    {
        var wrapper = Wrapper.New();
        var s = await ServiceAsyncNew.New(wrapper.Context);
        await s.Call();
    }
}
