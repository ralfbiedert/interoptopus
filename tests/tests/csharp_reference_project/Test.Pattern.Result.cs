using System.Linq;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternResult
{
    [Fact]
    public void simple_result_passthrough_works()
    {
        var x = new ResultU32FFIError();
        Interop.pattern_result_1(x).Ok();
    }

    [Fact]
    public void result_void_works()
    {
        Interop.pattern_result_3(ResultFFIError.OK).Ok();
        Assert.Equal(ResultFFIError.OK, Interop.pattern_result_3(ResultFFIError.OK));
        Assert.Equal(ResultFFIError.FAIL, Interop.pattern_result_3(ResultFFIError.FAIL));
    }
}
