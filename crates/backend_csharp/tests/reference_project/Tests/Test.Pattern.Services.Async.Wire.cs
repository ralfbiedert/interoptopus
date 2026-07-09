using System.Collections.Generic;
using System.Threading.Tasks;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternServicesAsyncWire
{
    [Fact]
    public async Task Passthrough()
    {
        var s = ServiceAsyncWire.Create();
        var d = new Dictionary<string, string>
        {
            { "hello", "world" }
        };
        var r = await s.WirePassthrough(d.Wire(), TestContext.Current.CancellationToken);
        r.Unwire();
    }
}