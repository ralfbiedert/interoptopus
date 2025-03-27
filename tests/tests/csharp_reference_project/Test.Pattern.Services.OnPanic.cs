using My.Company;
using Xunit;

public class TestPatternServicesOnPanic
{

    [Fact]
    public void ReturnResult()
    {
        var service = ServiceOnPanic.New();
        service.ReturnResult(123);
        service.Dispose();
    }

    [Fact]
    public void ReturnDefaultValue()
    {
        var service = ServiceOnPanic.New();
        Assert.Equal(123u, service.ReturnDefaultValue(123u));
        service.Dispose();
    }

    [Fact]
    public void ReturnUbOnPanic()
    {
        var service = ServiceOnPanic.New();
        Assert.Equal("Hello new_with", service.ReturnUbOnPanic());
        service.Dispose();
    }
}
