using My.Company;
using Xunit;

public class TestStructs
{
    [Fact]
    public void struct3()
    {
        var rval = Interop.struct3(new BoolField
        {
            val = true
        });
        
        Assert.True(rval);
    }
}