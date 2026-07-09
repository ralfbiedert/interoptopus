using System;
using System.Linq;
using System.Threading.Tasks;
using My.Company;
using Xunit;

public class TestPatternServicesAsyncSleep
{
    [Fact]
    public async Task ReturnAfterMs()
    {
        using var s = ServiceAsyncSleep.Create();
        var r = await s.ReturnAfterMs(123, 500, TestContext.Current.CancellationToken);
        Assert.Equal(123u, r);
    }

    [Fact]
    public async Task SupportsMultipleParallelCalls()
    {
        using var s = ServiceAsyncSleep.Create();

        // Spawn bunch of tasks returning in random order and make sure they work
        var tasks = Enumerable.Range(0, 10).Select(async _ =>
        {
            var x = Random.Shared.Next(100, 1000);
            var ms = Random.Shared.Next(100, 1000);

            var r = await s.ReturnAfterMs((ulong)x, (ulong)ms, TestContext.Current.CancellationToken);
            Assert.Equal(x, (int)r);
        }).ToList();

        await Task.WhenAll(tasks);
    }
}
