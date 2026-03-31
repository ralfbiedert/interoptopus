using My.Company;
using Xunit;

public class TestPatternServicesDependent
{
    [Fact]
    public void NewMainAndDependent()
    {
        var main = ServiceMain.Create(123);
        var dependent = ServiceDependent.FromMain(main);


        var rval = dependent.Get();
        Assert.Equal(123u, rval);

        dependent.PassMain(main);

        dependent.Dispose();
        main.Dispose();
    }
}
