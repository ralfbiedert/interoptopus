using System;
using System.Runtime.InteropServices;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestCoreSlices
{

    [Fact]
    public unsafe void pattern_ffi_slice_3()
    {
        var data = new byte[100_000];
        var smu = new SliceMutU8(data);

        var cc2 = new CallbackSliceMut((slice) =>
        {
            slice[0] = 1;
            slice[1] = 100;
        });

        Interop.pattern_ffi_slice_3b(smu, cc2);
        Assert.Equal(data[0], 1);
        Assert.Equal(data[1], 100);
    }

    [Fact]
    public void pattern_ffi_slice_2()
    {
        var data = new Vec3f32[] {
            new Vec3f32 { x = 1.0f, y = 2.0f, z = 3.0f },
            new Vec3f32 { x = 4.0f, y = 5.0f, z = 6.0f },
            new Vec3f32 { x = 7.0f, y = 8.0f, z = 9.0f },
        };

        var result = Interop.pattern_ffi_slice_2(data, 1);

        Assert.Equal(4.0f, result.x);
        Assert.Equal(5.0f, result.y);
        Assert.Equal(6.0f, result.z);
    }

    [Fact]
    public void pattern_ffi_slice_delegate_huge()
    {
        var cb = new CallbackHugeVecSlice(x => x[0]);
        var result = Interop.pattern_ffi_slice_delegate_huge(cb);
        // cb.Dispose();

        Assert.Equal(0, result.x);
    }

    [Fact]
    public void pattern_ffi_slice_6()
    {
        var data = new byte[] {1, 2, 3};
        var slice = new SliceMutU8(data);
        var cb = new CallbackU8(x =>
        {
            Assert.Equal(1, x);
            return 0;
        });
        Interop.pattern_ffi_slice_6(ref slice, cb);
        // cb.Dispose();
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
}