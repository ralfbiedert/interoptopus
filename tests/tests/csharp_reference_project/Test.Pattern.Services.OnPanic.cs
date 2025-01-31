using System;
using System.Linq;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternServicesOnPanic
{

    [Fact]
    public void service_onpanic()
    {
        var service = ServiceOnPanic.New();

        service.ReturnResult(123);

        Assert.Equal(service.ReturnDefaultValue(123u), 123u);
        Assert.Equal(service.ReturnUbOnPanic(), "Hello new_with");
    }
}
