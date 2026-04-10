using System;
using System.Diagnostics;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;
using My.Company;
using Xunit;

public class TestPatternServicesAsyncCancel
{
    [Fact]
    public async Task LongRunningCompletesNormally()
    {
        using var s = ServiceAsyncCancel.Create();
        var result = await s.LongRunning(5, 10);
        Assert.Equal(5u, result);
    }

    [Fact]
    public async Task LongRunningCancelledByToken()
    {
        using var s = ServiceAsyncCancel.Create();
        using var cts = new CancellationTokenSource();

        // Start a task that would take ~5 seconds
        var task = s.LongRunning(100, 50, cts.Token);

        // Cancel after 200ms
        cts.CancelAfter(200);

        var sw = Stopwatch.StartNew();
        await Assert.ThrowsAnyAsync<Exception>(async () => await task);
        sw.Stop();

        // Should complete much faster than 5000ms
        Assert.True(sw.ElapsedMilliseconds < 3000);
    }

    [Fact]
    public async Task SleepForeverCancelledByToken()
    {
        using var s = ServiceAsyncCancel.Create();
        using var cts = new CancellationTokenSource(300);

        var sw = Stopwatch.StartNew();
        await Assert.ThrowsAnyAsync<Exception>(async () => await s.SleepForever(cts.Token));
        sw.Stop();

        Assert.True(sw.ElapsedMilliseconds < 3000);
    }

    [Fact]
    public async Task PreCancelledTokenThrowsImmediately()
    {
        // Pre-cancelled token: the task should fail almost immediately.
        using var s = ServiceAsyncCancel.Create();
        using var cts = new CancellationTokenSource();
        cts.Cancel(); // Already cancelled

        var sw = Stopwatch.StartNew();
        await Assert.ThrowsAnyAsync<Exception>(async () => await s.LongRunning(1000, 100, cts.Token));
        sw.Stop();

        Assert.True(sw.ElapsedMilliseconds < 3000);
    }

    [Fact]
    public async Task DefaultTokenDoesNotCancel()
    {
        using var s = ServiceAsyncCancel.Create();
        var result = await s.LongRunning(5, 10, CancellationToken.None);
        Assert.Equal(5u, result);
    }

    /// Cancel one of several parallel tasks; the others should complete.
    [Fact]
    public async Task CancelOneOfManyParallelTasks()
    {
        using var s = ServiceAsyncCancel.Create();
        using var cts = new CancellationTokenSource(200);

        // This task will be cancelled
        var cancelledTask = s.LongRunning(1000, 50, cts.Token);

        // These tasks run with no cancellation and complete normally
        var normalTasks = Enumerable.Range(0, 3)
            .Select(_ => s.LongRunning(3, 10))
            .ToArray();

        // Normal tasks should complete fine
        var results = await Task.WhenAll(normalTasks);
        Assert.All(results, r => Assert.Equal(3u, r));

        // Cancelled task should throw
        await Assert.ThrowsAnyAsync<Exception>(async () => await cancelledTask);
    }

    /// The counter should stop incrementing after cancellation,
    /// proving the Rust future was actually dropped.
    [Fact]
    public async Task CountingWorkStopsAfterCancel()
    {
        using var s = ServiceAsyncCancel.Create();
        using var cts = new CancellationTokenSource();

        // Start counting work that would run for 200 iterations * 20ms = 4s
        var task = s.CountingWork(200, 20, cts.Token);

        // Let it run for 500ms, then cancel
        await Task.Delay(500);
        cts.Cancel();

        await Assert.ThrowsAnyAsync<Exception>(async () => await task);

        // Read the counter just after cancellation
        var counterAtCancel = s.Counter();

        // Wait a bit and read again — should NOT have increased
        await Task.Delay(500);
        var counterAfterWait = s.Counter();

        Assert.True(counterAtCancel > 0);
        Assert.True(counterAtCancel < 200);
        Assert.Equal(counterAtCancel, counterAfterWait);
    }

    /// Multiple cancellations in sequence on the same service.
    [Fact]
    public async Task RepeatedCancelAndReuse()
    {
        using var s = ServiceAsyncCancel.Create();

        for (var i = 0; i < 5; i++)
        {
            using var cts = new CancellationTokenSource(100);
            await Assert.ThrowsAnyAsync<Exception>(async () => await s.SleepForever(cts.Token));
        }

        // Service should still be usable after repeated cancellations
        var result = await s.LongRunning(3, 10);
        Assert.Equal(3u, result);
    }
}