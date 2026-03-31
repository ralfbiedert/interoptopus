using My.Company;
using Xunit;

public class TestPatternServicesDependent
{
    [Fact]
    public void NewMainAndDependent()
    {
        using var main = ServiceMain.Create(123);
        using var dependent = ServiceDependent.FromMain(main);


        var rval = dependent.Get();
        Assert.Equal(123u, rval);

        dependent.PassMain(main);
    }
}
