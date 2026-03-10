using A;
using Xunit;

public class TestInvoke
{
    [Fact]
    public void Invoke()
    {
        var vec1 = new Vec2() { x = 1.0f, y = 2.0f };
        var vec2 = Interop.my_function(vec1);

        var svc = ServiceBasic.New();
        var res = svc.Sum(1, 2);
        svc.Dispose();

        Assert.Equal(3, res);
    }

}