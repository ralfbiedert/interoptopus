using System.Threading.Tasks;
using My.Company;
using Xunit;

public class TestPatternServicesAsyncRval
{
    [Fact]
    public async Task Create()
    {
        using var s = ServiceAsyncRval.Simple();
        Assert.Equal(123u, await s.Number(TestContext.Current.CancellationToken));
        Assert.Equal(new Vec3f32 { x = 0, y = 0, z = 0 }, await s.Vecf32(TestContext.Current.CancellationToken));
        Assert.Equal("hello", (await s.Wire(TestContext.Current.CancellationToken)).Unwire());
    }

}