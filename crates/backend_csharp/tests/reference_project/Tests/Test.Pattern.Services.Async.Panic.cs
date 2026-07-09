using My.Company;
using System.Threading.Tasks;
using Xunit;

public class TestPatternServicesAsyncPanic
{
    [Fact]
    public async Task PanickingThrowsException()
    {
        using var s = ServiceAsyncPanic.Create();

        // A panic inside an async method aborts the future; the cancellation
        // guard then signals `AsyncOutcome::Cancelled`, which the trampoline
        // turns into a TaskCanceledException on the .NET side.
        await Assert.ThrowsAsync<TaskCanceledException>(async () =>
        {
            await s.Panicking(TestContext.Current.CancellationToken);
        });
    }

    [Fact]
    public async Task NotPanickingSucceeds()
    {
        using var s = ServiceAsyncPanic.Create();
        await s.NotPanicking(TestContext.Current.CancellationToken);
    }

    [Fact]
    public async Task ServiceWorksAfterPanic()
    {
        using var s = ServiceAsyncPanic.Create();

        // First call panics
        await Assert.ThrowsAsync<TaskCanceledException>(async () =>
        {
            await s.Panicking(TestContext.Current.CancellationToken);
        });

        // Service is still functional after the panic
        await s.NotPanicking(TestContext.Current.CancellationToken);
    }

    [Fact]
    public async Task ServiceWorksAfterRepeatedPanics()
    {
        using var s = ServiceAsyncPanic.Create();

        for (int i = 0; i < 10; i++)
        {
            await Assert.ThrowsAsync<TaskCanceledException>(async () =>
            {
                await s.Panicking(TestContext.Current.CancellationToken);
            });
        }

        // Service is still functional after repeated panics
        await s.NotPanicking(TestContext.Current.CancellationToken);
    }
}
