using System;
using My.Company;
using My.Company.Common;
using Xunit;
using Interop = My.Company.Interop;

public class TestPatternSlices
{
    [Fact]
    public void pattern_ffi_slice_1()
    {
        using var data = new uint[100_000].Slice();
        var result = Interop.pattern_ffi_slice_1(data);
        Assert.Equal(100_000u, result);
    }

    [Fact]
    public void slice_rejects_negative_indices()
    {
        using var data = new byte[] { 1, 2, 3 }.Slice();
        Assert.Throws<IndexOutOfRangeException>(() => _ = data.Slice[-1]);
    }

    [Fact]
    public void managed_slice_span_keeps_array_alive_without_the_lease()
    {
        var span = CreateManagedSliceSpan(out var managed);

        GC.Collect(2, GCCollectionMode.Forced, true, true);
        GC.WaitForPendingFinalizers();
        GC.Collect(2, GCCollectionMode.Forced, true, true);

        Assert.True(managed.IsAlive);
        Assert.Equal(new byte[] { 1, 2, 3 }, span.ToArray());
    }

    private static ReadOnlySpan<byte> CreateManagedSliceSpan(out WeakReference managed)
    {
        var array = new byte[] { 1, 2, 3 };
        var lease = array.Slice();
        managed = new WeakReference(array);
        return lease.Slice.ReadOnlySpan;
    }

    [Fact]
    public void empty_managed_slice_is_not_disposed()
    {
        using var data = System.Array.Empty<uint>().Slice();
        Assert.Equal(0u, Interop.pattern_ffi_slice_1(data));
    }

    [Fact]
    public void pattern_ffi_slice_of_structs_from_native_memory()
    {
        Interop.pattern_ffi_slice_of_structs_callback(attributes =>
        {
            Assert.IsNotAssignableFrom<IDisposable>(attributes);
            var attribute = attributes[0];
            Assert.Equal(3, attribute.bytes.Count);
            Assert.Equal(2, attribute.bytes[1]);
        });
    }


    [Fact]
    public void pattern_ffi_slice_2()
    {
        using var data = new Vec3f32[]
        {
            new() { x = 1.0f, y = 2.0f, z = 3.0f },
            new() { x = 4.0f, y = 5.0f, z = 6.0f },
            new() { x = 7.0f, y = 8.0f, z = 9.0f }
        }.Slice();

        var result = Interop.pattern_ffi_slice_2(data, 1);

        Assert.Equal(4.0f, result.x);
        Assert.Equal(5.0f, result.y);
        Assert.Equal(6.0f, result.z);
    }

    [Fact]
    public void pattern_ffi_slice_3()
    {
        using var data = new byte[100_000].SliceMut();

        Interop.pattern_ffi_slice_3(data, slice =>
        {
            slice[0] = 1;
            slice[1] = 100;
        });

        Assert.Equal(1, data.Slice[0]);
        Assert.Equal(100, data.Slice[1]);
    }

    [Fact]
    public void pattern_ffi_slice_5()
    {
        using var lease1 = new byte[100_000].Slice();
        using var lease2 = new byte[100_000].SliceMut();
        var data1 = lease1.Slice;
        var data2 = lease2.Slice;

        Interop.pattern_ffi_slice_5(ref data1, ref data2);
        Assert.Same(lease1.Slice, data1);
        Assert.NotSame(lease2.Slice, data2);
        Assert.Equal(100_000, lease2.Slice.Count);
        Assert.Equal(99_999, data2.Count);

        lease2.Dispose();
        Assert.Equal(0, data2.Count);
        Assert.Throws<ObjectDisposedException>(() => _ = data2[0]);
    }

    [Fact]
    public void pattern_ffi_slice_6()
    {
        using var lease = new byte[] { 1, 2, 3 }.SliceMut();
        var data = lease.Slice;

        using var callback = new CallbackU8(x =>
        {
            Assert.Equal(1, x);
            return 0;
        });
        Interop.pattern_ffi_slice_6(ref data, callback);
        Assert.Same(lease.Slice, data);
    }

    [Fact]
    public void non_blittable_slice_lease_owns_the_native_allocation()
    {
        var first = new byte[32];
        var second = new byte[32];
        first[0] = 1;
        second[0] = 2;
        var data = new CharArray
        {
            str = new FixedString32 { data = first },
            str_2 = new FixedString32 { data = second }
        };
        var lease = new[] { data }.SliceMut();
        var slice = lease.Slice;

        Interop.pattern_ffi_slice_8(ref slice, ca =>
        {
            Assert.Equal(1, ca.str.data[0]);
            Assert.Equal(2, ca.str_2.data[0]);
        });

        var ownedSlice = lease.Slice;
        lease.Dispose();
        Assert.Equal(0, ownedSlice.Count);
        Assert.Throws<ObjectDisposedException>(() => _ = ownedSlice[0]);
    }

    [Fact]
    public void pattern_ffi_slice_delegate_huge()
    {
        var result = Interop.pattern_ffi_slice_delegate_huge(x => x[0]);
        Assert.Equal(0, result.x);
    }

    [Fact]
    public void pattern_ffi_slice_9()
    {
        using var use_string = new UseString
        {
            s1 = "hello".Utf8(),
            s2 = "world".Utf8()
        };

        using var slice = new[]
        {
            use_string,
            use_string,
            use_string
        }.Slice();

        var rval = Interop.pattern_ffi_slice_9(slice).IntoString();

        Assert.Equal("hello", rval);
    }

    [Fact]
    public void pattern_ffi_slice_1b()
    {
        using var data = new uint[500].SliceMut();
        var result = Interop.pattern_ffi_slice_1b(data);
        Assert.Equal(500u, result);
    }

    [Fact]
    public void pattern_ffi_slice_4()
    {
        using var slice = new byte[] { 1, 2, 3 }.Slice();
        using var sliceMut = new byte[] { 4, 5, 6 }.SliceMut();
        Interop.pattern_ffi_slice_4(slice, sliceMut);
    }
}
