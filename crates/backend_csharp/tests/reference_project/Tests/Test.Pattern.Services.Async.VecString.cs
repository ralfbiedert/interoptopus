using System.Threading.Tasks;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternServicesAsyncVecString
{
    [Fact]
    public async Task HandleString()
    {
        using var s = ServiceAsyncVecString.Create();
        var r = await s.HandleString("abc".Utf8(), TestContext.Current.CancellationToken);
        Assert.Equal("abc", r.IntoString());
    }

    [Fact]
    public async Task HandleVecString()
    {
        using var s = ServiceAsyncVecString.Create();
        var v = new[]
        {
            "abc".Utf8()
        }.IntoVec();

        var r = await s.HandleVecString(v, TestContext.Current.CancellationToken);
        Assert.Equal("abc", r[0].IntoString());
    }


    [Fact]
    public async Task HandleNestedString()
    {
        using var s = ServiceAsyncVecString.Create();
        var r = await s.HandleNestedString("abc".Utf8(), TestContext.Current.CancellationToken);
        Assert.Equal("abc", r.s1.IntoString());
        Assert.Equal("abc", r.s2.IntoString());
    }
}
