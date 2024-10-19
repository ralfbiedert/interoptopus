using System.Linq;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternServices
{
    [Fact]
    public void pattern_service_generated()
    {
        var simpleService = SimpleService.NewWith(123);
        var b = new byte[] { 1, 2, 3 } ;

        simpleService.MethodMutSelfFfiError(b);
        var s1 = simpleService.ReturnString();
        var s2 = simpleService.ReturnString();

        var sliceMut = simpleService.ReturnSliceMut();
        sliceMut[0] = 44;

        var slice = simpleService.ReturnSlice();
        Assert.Equal(slice.Count, 123);
        Assert.Equal((int) slice[0], 44);
        Assert.Equal((int) slice[1], 123);

        uint value = 123;
        var lt = SimpleServiceLifetime.NewWith(ref value);
        var s3 = lt.ReturnStringAcceptSlice(System.Array.Empty<byte>());
        var s4 = lt.ReturnStringAcceptSlice(System.Array.Empty<byte>());
    }
}
