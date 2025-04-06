using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternVec
{
    [Fact]
    public void pattern_ffi_vec_1()
    {
        var vec = Interop.pattern_vec_1();

        Assert.Equal(3, vec.Count);
        Assert.Equal(1, vec[0]);
        Assert.Equal(2, vec[1]);
        Assert.Equal(3, vec[2]);

        vec.Dispose();
    }

    [Fact]
    public void pattern_ffi_vec_2()
    {
        var vec = Interop.pattern_vec_1();
        Interop.pattern_vec_2(vec);
        vec.Dispose();
    }

    [Fact]
    public void pattern_ffi_vec_2_no_reuse_after_move()
    {
        var vec = Interop.pattern_vec_1();
        Interop.pattern_vec_2(vec);

        // We must not be able to re-use the given vec
        Assert.Throws<InteropException>(() => Interop.pattern_vec_2(vec));
    }

    [Fact]
    public void pattern_ffi_vec_2_no_reuse_after_dispose()
    {
        var vec = Interop.pattern_vec_1();
        vec.Dispose();
        Assert.Throws<InteropException>(() => Interop.pattern_vec_2(vec));
    }

    [Fact]
    public void pattern_ffi_vec_2_dispose_dispose()
    {
        var vec = Interop.pattern_vec_1();
        vec.Dispose();
        vec.Dispose();
        vec.Dispose();
    }

    [Fact]
    public void pattern_ffi_vec_3()
    {

        var vec1 = Interop.pattern_vec_1();
        var vec2 = Interop.pattern_vec_3(vec1);

        Assert.Throws<InteropException>(() => vec1[0]);

        Assert.Equal(3, vec2.Count);
        Assert.Equal(1, vec2[0]);
        Assert.Equal(2, vec2[1]);
        Assert.Equal(3, vec2[2]);

        vec2.Dispose();

        Assert.Throws<InteropException>(() => vec2[0]);
    }

    [Fact]
    public void pattern_ffi_vec_4()
    {

        var vec1 = Interop.pattern_vec_1();
        var vec2 = Interop.pattern_vec_4(ref vec1);

        Assert.Equal(3, vec1.Count);
        Assert.Equal(3, vec2.Count);
        Assert.Equal(vec1[0], vec2[0]);
        Assert.Equal(vec1[1], vec2[1]);
        Assert.Equal(vec1[2], vec2[2]);

        vec1.Dispose();
        vec2.Dispose();
    }

    [Fact]
    public void pattern_ffi_vec_5()
    {

        var r = new [] { "1".Utf8(), "2".Utf8(), "3".Utf8() };
        var v = new [] { "1".Utf8(), "2".Utf8(), "3".Utf8() };
        var v1 = VecUtf8String.From(v);
        var v2 = Interop.pattern_vec_5(v1);

        Assert.Equal(r[0].String, v2[0].String);
        Assert.Equal(r[1].String, v2[1].String);
        Assert.Equal(r[2].String, v2[2].String);
    }


    [Fact]
    public void pattern_ffi_vec_6()
    {
        var v = new Vec3f32[]
        {
            new() { x = 1, y = 1, z = 1 },
            new() { x = 2, y = 2, z = 2 },
            new() { x = 3, y = 3, z = 3 },
        };

        var v1 = VecVec3f32.From(v);
        var v2 = Interop.pattern_vec_6(v1);

        Assert.Equal(v[0], v2[0]);
        Assert.Equal(v[1], v2[1]);
        Assert.Equal(v[2], v2[2]);
    }

    [Fact]
    public void pattern_ffi_vec_7()
    {
        var v = new [] { "1".Utf8(), "2".Utf8(), "3".Utf8() };
        var v1 = new UseSliceAndVec { s1 = SliceUtf8String.From(v), s2 = VecUtf8String.From(v) };
        Interop.pattern_vec_7(v1);

        // Assert.Equal(v[0], v2.s1[0]);
        // Assert.Equal(v[1], v2.s1[1]);
        // Assert.Equal(v[2], v2.s1[2]);

    }

    [Fact]
    public void pattern_ffi_vec_8()
    {
        var v = new [] { "1".Utf8(), "2".Utf8(), "3".Utf8() };
        var v1 = new UseSliceAndVec { s1 = SliceUtf8String.From(v), s2 = VecUtf8String.From(v) };

        // TODO: This rval deserialziation of slice inside composite has some issues
        var v2 = Interop.pattern_vec_8(v1);

        // Assert.Equal(v[0], v2.s1[0]);
        // Assert.Equal(v[1], v2.s1[1]);
        // Assert.Equal(v[2], v2.s1[2]);
    }

    [Fact]
    public void vec_empty()
    {
        var empty = VecUtf8String.Empty();
        Assert.Equal(0, empty.Count);
        empty.Dispose();
    }

}