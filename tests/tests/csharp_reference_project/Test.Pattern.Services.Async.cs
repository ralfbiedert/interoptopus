using System;
using System.Linq;
using System.Threading.Tasks;
using My.Company;
using Xunit;
using Array = My.Company.Array;


public class TestPatternServicesAsync
{
    [Fact]
    public async void service_async_explicit_basic()
    {
        var s = ServiceAsync.New();
        var r1 = await s.ReturnAfterMs(123, 500);
        var r2 = await s.HandleString("abc");
        
        s.Dispose();
        
        var ss = r2.Ok();
        Assert.Equal(r1.Ok(), 123u);
        Assert.Equal(r2.Ok(), "abc");
    }
    
    
    [Fact]
    public async void service_async_explicit_parallel()
    {
        var s = ServiceAsync.New();
        
        // Spawn bunch of tasks returning in random order and make sure they work 
        var tasks = Enumerable.Range(0, 10).Select(async _ => {
            var x = Random.Shared.Next(100, 1000);
            var ms = Random.Shared.Next(100, 1000);
            
            var r = await s.ReturnAfterMs((ulong) x, (ulong) ms);
            Assert.Equal((int) r.Ok(), x);
        }).ToList();
        
        await Task.WhenAll(tasks);
        s.Dispose();
    }
 
    
    [Fact]
    public async void service_async_complex_struct()
    {
        var s = ServiceAsync.New();
        var a = new NestedArray
        {
            field_array = new ushort [5],
            field_array_2 = new ushort [5],
            field_struct = new Array
            {
                data = new byte[16],
            },
            field_int = 123,
        };
        var r = await s.ProcessStruct(a);
        s.Dispose();
        
        Assert.Equal(r.Ok().field_int, 124);
    }

}
