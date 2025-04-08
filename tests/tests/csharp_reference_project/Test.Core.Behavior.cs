using My.Company;
using My.Company.Common;
using Xunit;

public class TestBehavior
{
    [Fact]
    public void behavior_panics_via_result()
    {
        var v = Interop.behavior_panics_via_result();
        Assert.Equal(ResultError.Panic, v);
    }

    [Fact]
    public void behavior_sleep()
    {
        Interop.behavior_sleep(10);
    }
}