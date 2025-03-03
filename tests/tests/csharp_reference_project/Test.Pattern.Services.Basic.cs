using System.Linq;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternServicesBasic
{
    [Fact]
    public void service_basic()
    {
        var service = BasicService.New();
        service.Dispose();
    }

}
