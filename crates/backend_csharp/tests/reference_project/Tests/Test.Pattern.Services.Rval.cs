using My.Company;
using Xunit;

public class TestPatternServicesRval
{
    [Fact]
    public void Create()
    {
        using var s = ServiceRval.Create();
        Assert.Equal(123u, s.Number());
        Assert.Equal(new Vec3f32 { x = 0, y = 0, z = 0 }, s.Vecf32());
        Assert.Equal("hello", s.Wire().Unwire());
    }
}