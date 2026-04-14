using My.Company;
using Xunit;

public class TestInvoke
{
    [Fact]
    public void Invoke()
    {
        var vec1 = new Vec2 { x = 1.0f, y = 2.0f };
        Interop.my_function(vec1);
    }
}