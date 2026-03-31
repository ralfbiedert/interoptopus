using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternServicesSlices
{
    [Fact]
    public void ReturnSliceMut()
    {
        using var s = ServiceVariousSlices.Create();
        var slice = s.ReturnSliceMut();
        slice[0] = 44;
    }

    [Fact]
    public void ReturnSlice()
    {
        using var s = ServiceVariousSlices.Create();
        var slice = s.ReturnSlice();
        Assert.Equal(slice.Count, 64);
        Assert.Equal(123, (int) slice[0]);
        Assert.Equal(123, (int) slice[1]);
    }

    [Fact]
    public void MutSelf()
    {
        using var s = ServiceVariousSlices.Create();
        using var data = new byte[] { 42, 10, 20 }.Slice();
        var result = s.MutSelf(data);
        Assert.Equal(42, result);
    }

    [Fact]
    public void MutSelfRef()
    {
        using var s = ServiceVariousSlices.Create();
        byte x = 99;
        byte y = 0;
        var result = s.MutSelfRef(ref x, ref y);
        Assert.Equal(99, result);
    }

    [Fact]
    public void MutSelfRefSlice()
    {
        using var s = ServiceVariousSlices.Create();
        byte x = 77;
        byte y = 0;
        using var slice = new byte[] { 1, 2, 3 }.Slice();
        var result = s.MutSelfRefSlice(ref x, ref y, slice);
        Assert.Equal(77, result);
    }

    [Fact]
    public void MutSelfRefSliceLimited()
    {
        using var s = ServiceVariousSlices.Create();
        byte x = 55;
        byte y = 0;
        using var slice1 = new byte[] { 1, 2 }.Slice();
        using var slice2 = new byte[] { 3, 4 }.Slice();
        var result = s.MutSelfRefSliceLimited(ref x, ref y, slice1, slice2);
        Assert.Equal(55, result);
    }

    [Fact]
    public void MutSelfFfiError()
    {
        using var s = ServiceVariousSlices.Create();
        using var data = new byte[10].SliceMut();
        s.MutSelfFfiError(data);
    }

    [Fact]
    public void MutSelfNoError()
    {
        using var s = ServiceVariousSlices.Create();
        using var data = new byte[10].SliceMut();
        s.MutSelfNoError(data);
    }
}
