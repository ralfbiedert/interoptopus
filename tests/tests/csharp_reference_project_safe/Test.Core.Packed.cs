using My.Company;
using Xunit;

public class TestCorePacked
{
    [Fact]
    public void packed_types()
    {
        var p1 = new Packed1
        {
            x = 12,
            y = 34
        };

        var p2 = Interop.packed_to_packed1(p1);

        Assert.Equal(p1.x, p2.x);
        Assert.Equal(p1.y, p2.y);
    }
}