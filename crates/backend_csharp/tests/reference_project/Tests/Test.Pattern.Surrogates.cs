using My.Company;
using Xunit;
using Interop = My.Company.Interop;

public class TestPatternSurrogates
{
    [Fact]
    public void pattern_surrogates_1()
    {
        var local = new Local { x = 42 };
        var container = new Container { foreign = new Local { x = 0 } };

        Interop.pattern_surrogates_1(local, ref container);

        Assert.Equal(42u, container.foreign.x);
    }
}
