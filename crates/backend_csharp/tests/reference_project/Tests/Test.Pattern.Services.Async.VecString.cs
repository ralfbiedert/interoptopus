using System;
using System.Linq;
using System.Threading.Tasks;
using My.Company;
using My.Company.Common;
using Xunit;
using Array = My.Company.Array;


public class TestPatternServicesAsyncVecString
{
    [Fact]
    public async void HandleString()
    {
        using var s = ServiceAsyncVecString.Create();
        var r = await s.HandleString("abc".Utf8());
        Assert.Equal(r.IntoString(), "abc");
    }

    [Fact]
    public async void HandleVecString()
    {
        using var s = ServiceAsyncVecString.Create();
        var v = new[]
        {
            "abc".Utf8()
        }.IntoVec();

        var r = await s.HandleVecString(v);
        Assert.Equal(r[0].IntoString(), "abc");
    }


    [Fact]
    public async void HandleNestedString()
    {
        using var s = ServiceAsyncVecString.Create();
        var r = await s.HandleNestedString("abc".Utf8());
        Assert.Equal(r.s1.IntoString(), "abc");
        Assert.Equal(r.s2.IntoString(), "abc");
    }
}
