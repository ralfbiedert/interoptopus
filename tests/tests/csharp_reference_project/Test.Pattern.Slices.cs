using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternSlices
{
    [Fact]
    public void pattern_ffi_slice_1()
    {
        var data = SliceU32.From(new uint[100_000]);
        var result = Interop.pattern_ffi_slice_1(data);
        Assert.Equal(100_000u, result);
    }


    [Fact]
    public void pattern_ffi_slice_2()
    {
        var data = new Vec3f32[] {
            new() { x = 1.0f, y = 2.0f, z = 3.0f },
            new() { x = 4.0f, y = 5.0f, z = 6.0f },
            new() { x = 7.0f, y = 8.0f, z = 9.0f },
        };

        var slice = SliceVec3f32.From(data);

        var result = Interop.pattern_ffi_slice_2(slice, 1);

        Assert.Equal(4.0f, result.x);
        Assert.Equal(5.0f, result.y);
        Assert.Equal(6.0f, result.z);
    }

    [Fact]
    public void pattern_ffi_slice_3()
    {
        var data = SliceMutU8.From(new byte[100_000]);

        Interop.pattern_ffi_slice_3(data, (slice) =>
        {
            slice[0] = 1;
            slice[1] = 100;
        });

        Assert.Equal(data[0], 1);
        Assert.Equal(data[1], 100);
    }

    [Fact]
    public void pattern_ffi_slice_5()
    {
        var data1 = SliceU8.From(new byte[100_000]);
        var data2 = SliceMutU8.From(new byte[100_000]);
        Interop.pattern_ffi_slice_5(ref data1, ref data2);
    }

    [Fact]
    public void pattern_ffi_slice_6()
    {
        var data = SliceMutU8.From(new byte[] {1, 2, 3});
        Interop.pattern_ffi_slice_6(ref data, x =>
        {
            Assert.Equal(1, x);
            return 0;
        });
    }

    // [Fact]
    // public void pattern_ffi_slice7()
    // {
    //     var data = new CharArray { str = "test", str_2 = "test2" };
    //     var slice = new SliceMut<CharArray>([data]);
    //     Interop.pattern_ffi_slice_8(ref slice, (ca) => {
    //         Assert.Equal("test", ca.str);
    //         Assert.Equal("test2", ca.str_2);
    //     });
    // }

    [Fact]
    public void pattern_ffi_slice_delegate_huge()
    {
        var result = Interop.pattern_ffi_slice_delegate_huge(x => x[0]);
        Assert.Equal(0, result.x);
    }

    [Fact]
    public void pattern_ffi_slice_9()
    {
        // var use_string = new Utf8String("hello");

        // var use_string = new UseString()
        // {
        //     s1 = new Utf8String("hello"),
        //     s2 = "world".Utf8()
        // };

        // Interop.pattern_ffi_slice_9([use_string, use_string, use_string]);
        // Assert.Equal("hello", Interop.pattern_ffi_slice_9([use_string]).String);
    }

}