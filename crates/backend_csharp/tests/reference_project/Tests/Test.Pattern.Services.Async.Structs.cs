using My.Company;
using Xunit;
using Array = My.Company.Array;


public class TestPatternServicesAsyncStructs
{
    [Fact]
    public async void ProcessStruct()
    {
        using var s = ServiceAsyncStructs.Create();
        var a = new NestedArray
        {
            field_enum = EnumRenamed.X,
            field_vec = new Vec3f32 { x = 0, y = 0, z = 0 },
            field_bool = false,
            field_int = 123,
            field_array = new ushort[5],
            field_array_2 = new ushort[5],
            field_struct = new Array
            {
                data = new byte[16]
            }
        };
        var r = await s.ProcessStruct(a);
        Assert.Equal(r.field_int, 124);
    }
}