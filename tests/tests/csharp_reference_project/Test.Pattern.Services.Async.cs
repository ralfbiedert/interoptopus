using System;
using System.Linq;
using System.Threading.Tasks;
using My.Company;
using Xunit;
using Array = My.Company.Array;


public class TestPatternServicesAsync
{
    [Fact]
    public async void ReturnAfterMs()
    {
        var s = ServiceAsync.New();
        var r = (await s.ReturnAfterMs(123, 500)).Ok();
        s.Dispose();
        Assert.Equal(r, 123u);
    }


    [Fact]
    public async void HandleString()
    {
        var s = ServiceAsync.New();
        var r = (await s.HandleString("abc")).Ok();
        s.Dispose();
        Assert.Equal(r, "abc");
    }


    [Fact]
    public async void HandleNestedString()
    {
        var s = ServiceAsync.New();
        var r = await s.HandleNestedString("abc");
        s.Dispose();
        Assert.Equal(r.s1, "abc");
        Assert.Equal(r.s2, "abc");
    }


    [Fact]
    public async void SupportsMultipleParallelCalls()
    {
        var s = ServiceAsync.New();
        
        // Spawn bunch of tasks returning in random order and make sure they work 
        var tasks = Enumerable.Range(0, 10).Select(async _ => {
            var x = Random.Shared.Next(100, 1000);
            var ms = Random.Shared.Next(100, 1000);
            
            var r = (await s.ReturnAfterMs((ulong)x, (ulong)ms)).Ok();
            Assert.Equal((int) r, x);
        }).ToList();
        
        await Task.WhenAll(tasks);
        s.Dispose();
    }
 
    
    [Fact]
    public async void ProcessStruct()
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
        var r = (await s.ProcessStruct(a)).Ok();
        s.Dispose();
        
        Assert.Equal(r.field_int, 124);
    }

}
