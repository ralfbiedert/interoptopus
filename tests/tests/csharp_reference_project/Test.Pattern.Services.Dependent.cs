using My.Company;
using Xunit;

public class TestPatternServicesDependent
{
    [Fact]
    public void NewMainAndDependent()
    {
        var main = ServiceMain.New(123);
        var dependent = ServiceDependent.FromMain(main.Context);


        var rval = dependent.Get();
        Assert.Equal(123u, rval);

        dependent.Dispose();
        main.Dispose();
    }
}
