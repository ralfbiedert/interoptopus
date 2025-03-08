using My.Company;
using Xunit;

public class TestPatternServicesOnPanic
{

    [Fact]
    public void ReturnResult()
    {
        var service = ServiceOnPanic.New();
        service.ReturnResult(123); // TODO: Should be .Ok()
        service.Dispose();
    }

    [Fact]
    public void ReturnDefaultValue()
    {
        var service = ServiceOnPanic.New();
        Assert.Equal(service.ReturnDefaultValue(123u), 123u);
        service.Dispose();
    }

    [Fact]
    public void ReturnUbOnPanic()
    {
        var service = ServiceOnPanic.New();
        Assert.Equal(service.ReturnUbOnPanic(), "Hello new_with");
        service.Dispose();
    }
}
