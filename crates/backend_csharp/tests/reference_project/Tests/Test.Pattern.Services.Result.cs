using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternServicesResult
{

    [Fact]
    public void New()
    {
        using var service = ServiceResult.Create();
    }

    [Fact]
    public void Test()
    {
        using var service = ServiceResult.Create();
        Assert.Throws<EnumException<Error>>(() => service.Test());
    }

    [Fact]
    public void ResultU32()
    {
        using var service = ServiceResult.Create();
        Assert.Equal(123u, service.ResultU32());
    }

    [Fact]
    public void ResultString()
    {
        using var service = ServiceResult.Create();
        Assert.Equal("hello world", service.ResultString().IntoString());
    }

    [Fact]
    public void ResultOptionEnum()
    {
        using var service = ServiceResult.Create();
        Assert.Equal(OptionEnumPayload.Some(EnumPayload.C(123)), service.ResultOptionEnum());
    }

    [Fact]
    public void ResultSlice()
    {
        using var service = ServiceResult.Create();
        using var slice = new uint[] {0, 1, 2}.Slice();
        Assert.Equal(2u, service.ResultSlice(slice, 2ul));
    }
}
