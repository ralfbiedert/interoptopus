using A;
using Xunit;

public class TestInvoke
{
    [Fact]
    public void Invoke()
    {
        var vec1 = new Interop.Vec2 { x = 1.0f, y = 2.0f };
        var vec2 = A.Interop.my_function(vec1);
    }

}