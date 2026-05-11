using My.Company;
using Xunit;

public class TestPatternServicesAsyncRval
{
    [Fact]
    public async void Create()
    {
        using var s = ServiceAsyncRval.Simple();
        Assert.Equal(123u, await s.Number());
        Assert.Equal(new Vec3f32 { x = 0, y = 0, z = 0 }, await s.Vecf32());
        Assert.Equal("hello", (await s.Wire()).Unwire());
    }

}