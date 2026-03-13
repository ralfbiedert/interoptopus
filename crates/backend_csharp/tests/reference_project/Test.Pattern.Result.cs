using My.Company;
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
    public void pattern_result_3()
    {
        Assert.True(Interop.pattern_result_3(ResultVoidError.Ok).IsOk);
        Assert.Equal(ResultVoidError.Ok, Interop.pattern_result_3(ResultVoidError.Ok));
        Assert.Equal(ResultVoidError.Null, Interop.pattern_result_3(ResultVoidError.Null));
    }
    
    [Fact]
    public void pattern_result_4()
    {
        Assert.True(Interop.pattern_result_4(ResultVoidError.Ok).IsOk);
    }

}
