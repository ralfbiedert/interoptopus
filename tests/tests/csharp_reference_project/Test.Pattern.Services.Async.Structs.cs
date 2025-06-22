using System;
using System.Linq;
using System.Threading.Tasks;
using My.Company;
using My.Company.Common;
using Xunit;
using Array = My.Company.Array;


public class TestPatternServicesAsyncStructs
{
    [Fact]
    public async void ProcessStruct()
    {
        var s = ServiceAsyncStructs.New();
        var a = new NestedArray
        {
            field_array = new ushort[5],
            field_array_2 = new ushort[5],
            field_struct = new Array
            {
                data = new byte[16],
            },
            field_int = 123,
        };
        var r = await s.ProcessStruct(a);
        s.Dispose();
        Assert.Equal(r.field_int, 124);
    }
}
