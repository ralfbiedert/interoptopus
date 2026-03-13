using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternResult
{
    [Fact]
    public void pattern_result_1()
    {
        var x = new ResultU32Error();
        Interop.pattern_result_1(x).AsOk();
    }

    [Fact]
    public void pattern_result_3()
    {
        Assert.True(Interop.pattern_result_3(ResultError.Ok).IsOk);
        Assert.Equal(ResultError.Ok, Interop.pattern_result_3(ResultError.Ok));
        Assert.Equal(ResultError.Null, Interop.pattern_result_3(ResultError.Null));
    }
    
    [Fact]
    public void pattern_result_4()
    {
        Assert.True(Interop.pattern_result_4(ResultVoid.Ok).IsOk);
    }

}
