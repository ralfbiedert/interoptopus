using System.Linq;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternServicesSlices
{
    [Fact]
    public void ReturnSliceMut()
    {
        var s = ServiceVariousSlices.New();
        var slice = s.ReturnSliceMut();
        s.Dispose();
        slice[0] = 44;
    }

    [Fact]
    public void ReturnSlice()
    {
        var s = ServiceVariousSlices.New();
        var slice = s.ReturnSlice();
        Assert.Equal(slice.Count, 64);
        Assert.Equal(123, (int) slice[0]);
        Assert.Equal(123, (int) slice[1]);
    }
}
