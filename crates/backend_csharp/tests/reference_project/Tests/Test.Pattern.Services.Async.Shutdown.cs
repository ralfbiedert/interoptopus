using System;
using System.Diagnostics;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;
using My.Company;
using Xunit;
using Xunit.Abstractions;

public class TestPatternServicesAsyncShutdown
{
    [Fact]
    public async Task AwaitThenDisposeWithoutSyncContext()
    {
        var task = Task.Run(async () =>
        {
            // Task.Run runs on a thread pool thread with no SynchronizationContext,
            // simulating a .NET Core service (ASP.NET Core has no SyncContext).
            Assert.Null(SynchronizationContext.Current);

            var s = ServiceAsyncSleep.Create();
            await s.ReturnAfterMs(0, 100);
            s.Dispose();
        });

        var winner = await Task.WhenAny(task, Task.Delay(5000));
        if (winner != task)
        {
            Assert.True(false, "DEADLOCK DETECTED: await + Dispose without SyncContext hung for 5s");
        }

        Assert.True(winner == task, "Deadlock: await + Dispose without SyncContext timed out after 5s");

        await task; // propagate any exceptions
    }
}
