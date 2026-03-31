using My.Company;
using Xunit;

public class TestPatternServicesBasic
{
    [Fact]
    public void NewDispose()
    {
        using var service = ServiceBasic.Create();
    }

}
