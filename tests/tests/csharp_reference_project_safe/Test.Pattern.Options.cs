using System.Linq;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternOptions
{
    [Fact]
    public void pattern_ffi_option_nullable()
    {
        var t = new Inner();
        OptionInner someOpt = OptionInner.FromNullable(t);
        Inner? nullableOpt = someOpt.ToNullable();
        Assert.True(nullableOpt.HasValue);

        OptionInner someOpt2 = OptionInner.FromNullable(null);
        Inner? nullableOpt2 = someOpt2.ToNullable();
        Assert.False(nullableOpt2.HasValue);
    }
}
