using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternServicesAsyncPanic
{
    [Fact]
    public async void ServiceWorksAfterRepeatedPanics()
    {
        using var s = ServiceAsyncPanic.Create();

        for (int i = 0; i < 10; i++)
        {
            await Assert.ThrowsAsync<EnumException>(async () =>
            {
                await s.Panicking();
            });
        }

        // Service is still functional after repeated panics
        await s.NotPanicking();
    }
}
