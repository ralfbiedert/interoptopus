using My.Company;
using Xunit;

public class TestPatternServicesString
{
    [Fact]
    public void CallbackString()
    {
        var str = "Hello World";
        var rval = string.Empty;

        var s = ServiceStrings.New();
        s.CallbackString(str, s1 => rval = s1);
        s.Dispose();

        Assert.Equal(str, rval);
    }
}
