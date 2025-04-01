using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternServicesString
{
    [Fact]
    public void NewStr()
    {
        ServiceStrings.NewString("hello world".Utf8()).Dispose();
    }

    [Fact]
    public void CallbackString()
    {
        var str = "Hello World";
        var rval = new Utf8String("");

        var s = ServiceStrings.New();
        s.CallbackString(str.Utf8(), s1 => rval = s1);
        s.Dispose();

        Assert.Equal(str, rval.String);
    }
}
