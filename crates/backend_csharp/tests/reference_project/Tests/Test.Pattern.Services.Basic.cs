using My.Company;
using Xunit;

public class TestPatternServicesBasic
{
    [Fact]
    public void Create()
    {
        using var service = ServiceBasic.Create();
    }
}