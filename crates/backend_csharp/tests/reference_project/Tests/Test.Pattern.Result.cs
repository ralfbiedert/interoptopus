using My.Company;
using My.Company.Common;
using Xunit;
using Interop = My.Company.Interop;

public class TestPatternResult
{
    [Fact]
    public void pattern_result_1()
    {
        var x = new ResultUintError();
        Interop.pattern_result_1(x).AsOk();
    }

    [Fact]
    public void pattern_result_2()
    {
        var result = Interop.pattern_result_2();
        Assert.True(result.IsOk);
    }

    [Fact]
    public void pattern_result_3()
    {
        Assert.True(Interop.pattern_result_3(ResultVoidError.Ok).IsOk);
        Assert.Equal(ResultVoidError.Ok, Interop.pattern_result_3(ResultVoidError.Ok));
        Assert.Equal(ResultVoidError.Null, Interop.pattern_result_3(ResultVoidError.Null));
    }

    [Fact]
    public void pattern_result_4()
    {
        Assert.True(Interop.pattern_result_4(ResultVoidVoid.Ok).IsOk);
    }

    [Fact]
    public void pattern_string_5()
    {
        var w = new UseString { s1 = "hello".Utf8(), s2 = "world".Utf8() };
        var result = Interop.pattern_string_5(w);
        var ok = result.AsOk();
        Assert.Equal("hello", ok.s1.String);
        Assert.Equal("world", ok.s2.String);
    }
}