using System;
using System.Text;
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
        Assert.Throws<IndexOutOfRangeException>(() => _ = data[-1]);
    }

    [Fact]
    public void pattern_ffi_slice_of_structs_from_native_memory()
    {
        Interop.pattern_ffi_slice_of_structs_callback(attributes =>
        {
            var attribute = attributes[0];
            Assert.Equal(3, attribute.bytes.Count);
            Assert.Equal(2, attribute.bytes[1]);
        });
    }

    [Fact]
    public void disposing_a_rust_provided_slice_does_not_free_it()
    {
        Interop.pattern_ffi_slice_of_structs_callback(attributes =>
        {
            // `attributes` borrows Rust-owned memory. Disposing it must not call
            // FreeHGlobal on a pointer the native allocator never returned, and
            // must leave the borrowed view usable for the rest of the call.
            attributes.Dispose();
            attributes.Dispose();

            Assert.Equal(1, attributes.Count);
            Assert.Equal(3, attributes[0].bytes.Count);
            Assert.Equal(2, attributes[0].bytes[1]);
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

        Assert.Equal(1, data[0]);
        Assert.Equal(100, data[1]);
    }

    [Fact]
    public void pattern_ffi_slice_5()
    {
        var data1 = new byte[100_000].Slice();
        var data2 = new byte[100_000].SliceMut();

        Interop.pattern_ffi_slice_5(ref data1, ref data2);
        data1.Dispose();
        data2.Dispose();
    }

    [Fact]
    public void pattern_ffi_slice_6()
    {
        var data = new byte[] { 1, 2, 3 }.SliceMut();

        using var callback = new CallbackU8(x =>
        {
            Assert.Equal(1, x);
            return 0;
        });
        Interop.pattern_ffi_slice_6(ref data, callback);
        data.Dispose();
    }

    [Fact]
    public void pattern_ffi_slice_8()
    {
        var slice = new[] { CharArrayOf("test", "test2") }.SliceMut();

        var calls = 0;
        Interop.pattern_ffi_slice_8(ref slice, ca =>
        {
            calls += 1;
            Assert.Equal("test", AsString(ca.str));
            Assert.Equal("test2", AsString(ca.str_2));
        });

        Assert.Equal(1, calls);
    }

    [Fact]
    public void disposing_a_ref_round_tripped_slice_still_frees_it()
    {
        var slice = new[] { CharArrayOf("test", "test2") }.SliceMut();

        Interop.pattern_ffi_slice_8(ref slice, _ => { });

        // The `ref` write-back replaces the caller's wrapper. This one allocated its own
        // buffer through `From(CharArray[])`, so the replacement must keep that ownership
        // or the allocation can never be released.
        slice.Dispose();
        Assert.Throws<NullReferenceException>(() => _ = slice[0]);
    }

    private static CharArray CharArrayOf(string str, string str2) =>
        new() { str = FixedStringOf(str), str_2 = FixedStringOf(str2) };

    private static FixedString32 FixedStringOf(string s)
    {
        var data = new byte[32];
        Encoding.UTF8.GetBytes(s).CopyTo(data, 0);
        return new FixedString32 { data = data };
    }

    private static string AsString(FixedString32 s) =>
        Encoding.UTF8.GetString(s.data).TrimEnd('\0');

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
