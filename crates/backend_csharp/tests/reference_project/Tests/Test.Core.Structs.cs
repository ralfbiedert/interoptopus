using System;
using My.Company;
using Xunit;

public class TestStructs
{
    [Fact]
    public void struct3()
    {
        var rval = Interop.struct3(new BoolField
        {
            val = true
        });

        Assert.True(rval.Is);
    }

    [Fact]
    public void struct1()
    {
        var x = new Tupled { field_0 = 5 };
        var result = Interop.struct1(x);
        Assert.Equal(10, result.field_0);
    }

    [Fact]
    public void struct2_with_ref()
    {
        var v = new Vec3f32 { x = 1.0f, y = 2.0f, z = 3.0f };
        var t = new Tupled { field_0 = 42 };
        var result = Interop.struct2(v, ref t);
        Assert.True(result.IsOk);
    }

    [Fact]
    public void struct2_with_null()
    {
        var v = new Vec3f32 { x = 1.0f, y = 2.0f, z = 3.0f };
        var result = Interop.struct2(v, IntPtr.Zero);
        Assert.True(result.IsOk);
    }
}