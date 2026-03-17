using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternServicesResult
{

    [Fact]
    public void New()
    {
        var service = ServiceResult.Create();
        service.Dispose();
    }

    [Fact]
    public void Test()
    {
        var service = ServiceResult.Create();
        Assert.Throws<EnumException<Error>>(() => service.Test());
        service.Dispose();
    }

    [Fact]
    public void ResultU32()
    {
        var service = ServiceResult.Create();
        Assert.Equal(123u, service.ResultU32());
        service.Dispose();
    }

    [Fact]
    public void ResultString()
    {
        var service = ServiceResult.Create();
        Assert.Equal("hello world", service.ResultString().IntoString());
        service.Dispose();
    }

    [Fact]
    public void ResultOptionEnum()
    {
        var service = ServiceResult.Create();
        Assert.Equal(OptionEnumPayload.Some(EnumPayload.C(123)), service.ResultOptionEnum());
        service.Dispose();
    }

    [Fact]
    public void ResultSlice()
    {
        var service = ServiceResult.Create();
        var slice = new uint[] {0, 1, 2}.Slice();
        Assert.Equal(2u, service.ResultSlice(slice, 2ul));
        slice.Dispose();
        service.Dispose();
    }
}
