using A;
using Xunit;

public class TestInvoke
{
    [Fact]
    public async void Invoke()
    {
        var vec1 = new Vec2() { x = 1.0f, y = 2.0f };
        var vec2 = Interop.my_function(vec1);

        var svc = ServiceBasic.New();
        var res = svc.Sum(1, 2);
        svc.Dispose();

        var svc2 = ServiceBasic2.New();
        await svc2.Sum(1, 2);

        Assert.Equal(3, res);
    }

}