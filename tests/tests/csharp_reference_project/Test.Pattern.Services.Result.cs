using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternServicesResult
{

    [Fact]
    public void New()
    {
        var service = ServiceResult.New();
        service.Dispose();
    }

    [Fact]
    public void Test()
    {
        var service = ServiceResult.New();
        Assert.Throws<InteropException>(() => service.Test());
        service.Dispose();
    }

    [Fact]
    public void ResultU32()
    {
        var service = ServiceResult.New();
        Assert.Equal(123u, service.ResultU32());
        service.Dispose();
    }

    [Fact]
    public void ResultString()
    {
        var service = ServiceResult.New();
        Assert.Equal("hello world", service.ResultString().IntoString());
        service.Dispose();
    }

    [Fact]
    public void ResultOptionEnum()
    {
        var service = ServiceResult.New();
        Assert.Equal(OptionEnumPayload.Some(EnumPayload.C(123)), service.ResultOptionEnum());
        service.Dispose();
    }

    [Fact]
    public void ResultSlice()
    {
        var service = ServiceResult.New();
        var slice = new uint[] {0, 1, 2}.Slice();
        Assert.Equal(2u, service.ResultSlice(slice, 2ul));
        slice.Dispose();
        service.Dispose();
    }
}
