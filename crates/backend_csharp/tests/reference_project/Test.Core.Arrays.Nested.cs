using System.Linq;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestArrayNested
{
    [Fact]
    public void nested_array_1()
    {
        var result = Interop.nested_array_1();
        Assert.Equal(EnumRenamed.X, result.field_enum);
        Assert.Equal(new Vec3f32
        {
            x = 1.0f,
            y = 2.0f,
            z = 3.0f
        }, result.field_vec);
        Assert.Equal(true, result.field_bool);
        Assert.Equal(42, result.field_int);
        Assert.Equal(Enumerable.Range(1, 5).Select(i => (ushort)i).ToArray(), result.field_array);
        Assert.Equal(Enumerable.Range(1, 16).Select(i => (byte)i).ToArray(), result.field_struct.data);
    }

    [Fact]
    public void nested_array_2()
    {
        var result = CreateNestedArray();
        Interop.nested_array_2(ref result);
        Assert.Equal(EnumRenamed.X, result.field_enum);
        Assert.Equal(new Vec3f32
        {
            x = 1.0f,
            y = 2.0f,
            z = 3.0f
        }, result.field_vec);
        Assert.Equal(true, result.field_bool);
        Assert.Equal(42, result.field_int);
        Assert.Equal(Enumerable.Range(1, 5).Select(i => (ushort)i).ToArray(), result.field_array);
        Assert.Equal(Enumerable.Range(1, 16).Select(i => (byte)i).ToArray(), result.field_struct.data);
    }

    [Fact]
    public void nested_array_3()
    {
        var result = Interop.nested_array_3(CreateNestedArray());
        Assert.Equal(2, result);
    }

    [Fact]
    public void nested_array_3_throws()
    {
        Assert.Throws<System.InvalidOperationException>(() =>
        {
            Interop.nested_array_3(new NestedArray
            {
                field_array = [1, 2, 3],
                field_struct = new Array
                {
                    data = Enumerable.Range(1, 16).Select(i => (byte)i).ToArray()
                }
            });
        });
    }

    private static NestedArray CreateNestedArray()
    {

        return new NestedArray
        {
            field_array = [1, 2, 3, 4, 5],
            field_array_2 = [1, 2, 3, 4, 5],
            field_struct = new Array
            {
                data = Enumerable.Range(1, 16).Select(i => (byte)i).ToArray()
            }
        };
    }
}


