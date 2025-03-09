using My.Company;
using Xunit;

public class TestPatternResult
{
    [Fact]
    public void pattern_result_1()
    {
        var x = new ResultU32Error();
        Interop.pattern_result_1(x).Ok();
    }

    [Fact]
    public void pattern_result_3()
    {
        Interop.pattern_result_3(ResultError.OK).Ok();
        Assert.Equal(ResultError.OK, Interop.pattern_result_3(ResultError.OK));
        Assert.Equal(ResultError.FAIL, Interop.pattern_result_3(ResultError.FAIL));
    }
}
