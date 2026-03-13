using My.Company;
using Xunit;

public class TestPatternServicesSlices
{
    [Fact]
    public void ReturnSliceMut()
    {
        var s = ServiceVariousSlices.New();
        var slice = s.ReturnSliceMut();
        slice[0] = 44;
        s.Dispose();
    }

    [Fact]
    public void ReturnSlice()
    {
        var s = ServiceVariousSlices.New();
        var slice = s.ReturnSlice();
        Assert.Equal(slice.Count, 64);
        Assert.Equal(123, (int) slice[0]);
        Assert.Equal(123, (int) slice[1]);
        s.Dispose();
    }
}
