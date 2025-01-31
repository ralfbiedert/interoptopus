using System.Linq;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternServicesLifetimes
{

    [Fact]
    public void service_lifetimes()
    {
        uint value = 123;
        var bools = new[] { Bool.True, Bool.True, Bool.False };
        var bytes = new byte[] { 0 };

        var service = ServiceUsingLifetimes.NewWith(ref value);

        service.Lifetime1(bools);
        service.Lifetime2(bools);

        var str = service.ReturnStringAcceptSlice(bytes);

        Assert.True(string.IsNullOrEmpty(str));
    }


}
