using System.Linq;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternResult
{
    [Fact]
    public void simple_result_passthrough_works()
    {
        Interop.pattern_result_1(new ResultU32FFIError()).Ok();
    }

    [Fact]
    public void result_void_works()
    {
        Assert.Equal(new ResultFFIError(FFIError.Ok), Interop.pattern_result_3(new ResultFFIError(FFIError.Ok)));
        Assert.Equal(new ResultFFIError(FFIError.Fail), Interop.pattern_result_3(new ResultFFIError(FFIError.Fail)));
    }
}
