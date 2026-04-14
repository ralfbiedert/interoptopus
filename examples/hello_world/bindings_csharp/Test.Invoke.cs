using My.Company;
using Xunit;

public class TestInvoke
{
    [Fact]
    public void Invoke()
    {
        var vec1 = new Vec2() { x = 1.0f, y = 2.0f };
        var vec2 = Interop.my_function(vec1);

        Assert.Equal(1.0f, vec2.x);
        Assert.Equal(2.0f, vec2.y);
    }
}
