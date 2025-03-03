using My.Company;
using Xunit;

public class TestStructs
{
    [Fact]
    public void Test_bool_field()
    {
        var rval = Interop.bool_field(new BoolField
        {
            val = true
        });
        
        Assert.True(rval);
    }
}