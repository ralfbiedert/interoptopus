using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternServicesSlices
{
    [Fact]
    public void ReturnSliceMut()
    {
        var s = ServiceVariousSlices.Create();
        var slice = s.ReturnSliceMut();
        slice[0] = 44;
        s.Dispose();
    }

    [Fact]
    public void ReturnSlice()
    {
        var s = ServiceVariousSlices.Create();
        var slice = s.ReturnSlice();
        Assert.Equal(slice.Count, 64);
        Assert.Equal(123, (int) slice[0]);
        Assert.Equal(123, (int) slice[1]);
        s.Dispose();
    }

    [Fact]
    public void MutSelf()
    {
        var s = ServiceVariousSlices.Create();
        var data = new byte[] { 42, 10, 20 }.Slice();
        var result = s.MutSelf(data);
        Assert.Equal(42, result);
        data.Dispose();
        s.Dispose();
    }

    [Fact]
    public void MutSelfRef()
    {
        var s = ServiceVariousSlices.Create();
        byte x = 99;
        byte y = 0;
        var result = s.MutSelfRef(ref x, ref y);
        Assert.Equal(99, result);
        s.Dispose();
    }

    [Fact]
    public void MutSelfRefSlice()
    {
        var s = ServiceVariousSlices.Create();
        byte x = 77;
        byte y = 0;
        var slice = new byte[] { 1, 2, 3 }.Slice();
        var result = s.MutSelfRefSlice(ref x, ref y, slice);
        Assert.Equal(77, result);
        slice.Dispose();
        s.Dispose();
    }

    [Fact]
    public void MutSelfRefSliceLimited()
    {
        var s = ServiceVariousSlices.Create();
        byte x = 55;
        byte y = 0;
        var slice1 = new byte[] { 1, 2 }.Slice();
        var slice2 = new byte[] { 3, 4 }.Slice();
        var result = s.MutSelfRefSliceLimited(ref x, ref y, slice1, slice2);
        Assert.Equal(55, result);
        slice1.Dispose();
        slice2.Dispose();
        s.Dispose();
    }

    [Fact]
    public void MutSelfFfiError()
    {
        var s = ServiceVariousSlices.Create();
        var data = new byte[10].SliceMut();
        s.MutSelfFfiError(data);
        data.Dispose();
        s.Dispose();
    }

    [Fact]
    public void MutSelfNoError()
    {
        var s = ServiceVariousSlices.Create();
        var data = new byte[10].SliceMut();
        s.MutSelfNoError(data);
        data.Dispose();
        s.Dispose();
    }
}
