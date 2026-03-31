using System;
using System.Linq;
using System.Threading.Tasks;
using My.Company;
using My.Company.Common;
using Xunit;
using Array = My.Company.Array;


public class TestPatternServicesAsyncSleep
{
    [Fact]
    public async void ReturnAfterMs()
    {
        using var s = ServiceAsyncSleep.Create();
        var r = await s.ReturnAfterMs(123, 500);
        Assert.Equal(r, 123u);
    }

    [Fact]
    public async void SupportsMultipleParallelCalls()
    {
        using var s = ServiceAsyncSleep.Create();

        // Spawn bunch of tasks returning in random order and make sure they work
        var tasks = Enumerable.Range(0, 10).Select(async _ =>
        {
            var x = Random.Shared.Next(100, 1000);
            var ms = Random.Shared.Next(100, 1000);

            var r = await s.ReturnAfterMs((ulong)x, (ulong)ms);
            Assert.Equal((int)r, x);
        }).ToList();

        await Task.WhenAll(tasks);
    }


}
