using System;
using System.Linq;
using System.Threading.Tasks;
using My.Company;
using Xunit;


public class TestPatternServicesAsync
{
    [Fact]
    public async void service_async_explicit_basic()
    {
        var s = ServiceAsync.New();
        var r = await s.ReturnAfterMsExplicit(123, 500);
        Assert.Equal(r.Ok(), 123u);
    }
    
    
    [Fact]
    public async void service_async_explicit_parallel()
    {
        var s = ServiceAsync.New();
        
        // Spawn bunch of tasks returning in random order and make sure they work 
        var tasks = Enumerable.Range(0, 10).Select(async _ => {
            var x = Random.Shared.Next(100, 1000);
            var ms = Random.Shared.Next(100, 1000);
            
            var r = await s.ReturnAfterMsExplicit((ulong) x, (ulong) ms);
            
            Assert.Equal((int) r.Ok(), x);
        }).ToList();
        
        await Task.WhenAll(tasks);
    }
    
    [Fact]
    public async void service_async_implicit_basic()
    {
        var s = ServiceAsync.New();
        var r = await s.ReturnAfterMs(123, 500);
        var t = await s.Xxx(45);
        Assert.Equal(r.Ok(), 123u);
        Assert.Equal(t.Ok(), 45);
        s.Dispose();
    }

}
