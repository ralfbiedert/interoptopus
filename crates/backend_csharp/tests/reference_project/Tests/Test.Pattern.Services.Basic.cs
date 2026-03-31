using My.Company;
using Xunit;

public class TestPatternServicesBasic
{
    [Fact]
    public void NewDispose()
    {
        var service = ServiceBasic.Create();
        service.Dispose();
    }

}
