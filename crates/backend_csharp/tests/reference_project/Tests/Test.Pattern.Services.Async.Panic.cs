using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternServicesAsyncPanic
{
    [Fact]
    public async void PanickingThrowsException()
    {
        using var s = ServiceAsyncPanic.Create();

        await Assert.ThrowsAsync<EnumException>(async () =>
        {
            await s.Panicking();
        });
    }

    [Fact]
    public async void NotPanickingSucceeds()
    {
        using var s = ServiceAsyncPanic.Create();
        await s.NotPanicking();
    }

    [Fact]
    public async void ServiceWorksAfterPanic()
    {
        using var s = ServiceAsyncPanic.Create();

        // First call panics
        await Assert.ThrowsAsync<EnumException>(async () =>
        {
            await s.Panicking();
        });

        // Service is still functional after the panic
        await s.NotPanicking();
    }

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
