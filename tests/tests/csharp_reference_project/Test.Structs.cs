using My.Company;
using Xunit;

public class TestStructs
{
    [Fact]
    public void Test_bool_field()
    {
        Assert.True(Interop.bool_field(new BoolField{
            val = true
        }));
    }
}