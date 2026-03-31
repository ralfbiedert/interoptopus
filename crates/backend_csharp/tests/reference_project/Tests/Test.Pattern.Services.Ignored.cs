using My.Company;
using Xunit;

public class TestPatternServicesIgnored
{
    [Fact]
    public void CreateDispose()
    {
        var s = ServiceIgnoringMethods.Create();
        s.Dispose();
    }

    [Fact]
    public void ThisIsIgnored()
    {
        var s = ServiceIgnoringMethods.Create();
        s.ThisIsIgnored();
        s.Dispose();
    }

    [Fact]
    public void Test()
    {
        var s = ServiceIgnoringMethods.Create();
        s.Test(42);
        s.Dispose();
    }
}
