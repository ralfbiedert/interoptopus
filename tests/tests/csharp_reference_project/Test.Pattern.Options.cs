using System.Linq;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternOptions
{
    [Fact]
    public void FromToNullable()
    {
        var t = new Inner();
        var someOpt = OptionInner.FromNullable(t);
        var nullableOpt = someOpt.ToNullable();
        Assert.True(nullableOpt.HasValue);

        var someOpt2 = OptionInner.FromNullable(null);
        var nullableOpt2 = someOpt2.ToNullable();
        Assert.False(nullableOpt2.HasValue);
    }
}
