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
        var rval = string.Empty;

        var s = ServiceStrings.Create();
        s.CallbackString(str.Utf8(), s1 => rval = s1.IntoString());
        s.Dispose();

        Assert.Equal(str, rval);
    }

    [Fact]
    public void PassCstr()
    {
        var s = ServiceStrings.Create();
        s.PassCstr("hello world");
        s.Dispose();
    }

    [Fact]
    public void ReturnCstr()
    {
        var s = ServiceStrings.Create();
        var result = s.ReturnCstr();
        Assert.Equal(result, "hello.world");
        s.Dispose();
    }
}
