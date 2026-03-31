using My.Company;
using Xunit;

public class TestPatternServicesIgnored
{
    [Fact]
    public void CreateDispose()
    {
        using var s = ServiceIgnoringMethods.Create();
    }

    [Fact]
    public void ThisIsIgnored()
    {
        using var s = ServiceIgnoringMethods.Create();
        s.ThisIsIgnored();
    }

    [Fact]
    public void Test()
    {
        using var s = ServiceIgnoringMethods.Create();
        s.Test(42);
    }
}
