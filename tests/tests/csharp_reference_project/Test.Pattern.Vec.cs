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

}