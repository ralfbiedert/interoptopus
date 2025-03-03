using System.Linq;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternServicesSlices
{
    [Fact]
    public void service_slices()
    {
        var service_slices = ServiceVariousSlices.New();
        var b = new byte[] { 1, 2, 3 } ;

        var sliceMut = service_slices.ReturnSliceMut();
        sliceMut[0] = 44;

        var slice = service_slices.ReturnSlice();
        Assert.Equal(slice.Count, 64);
        Assert.Equal((int) slice[0], 44);
        Assert.Equal((int) slice[1], 123);

        uint value = 123;
        var lt = ServiceUsingLifetimes.NewWith(ref value);
        var s3 = lt.ReturnStringAcceptSlice(System.Array.Empty<byte>());
        var s4 = lt.ReturnStringAcceptSlice(System.Array.Empty<byte>());
        service_slices.Dispose();
    }
}
