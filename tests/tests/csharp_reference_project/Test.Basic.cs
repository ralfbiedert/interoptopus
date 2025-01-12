using My.Company;
using Xunit;

public class TestBasics
{
    [Fact]
    public void Primitives()
    {
        var rval = Interop.primitive_u8(0);
        Assert.Equal(rval, 255);
    }
}
