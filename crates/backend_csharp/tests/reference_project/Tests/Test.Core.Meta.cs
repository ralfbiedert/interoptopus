using My.Company;
using Xunit;

public class TestMeta
{
    [Fact]
    public void meta_ambiguous_1()
    {
        var v = new Vec1 { x = 1.0f, y = 2.0f };
        var result = Interop.meta_ambiguous_1(v);
        Assert.Equal(1.0f, result.x);
        Assert.Equal(2.0f, result.y);
    }

    [Fact]
    public void meta_ambiguous_2()
    {
        var v = new Vec2 { x = 1.0, z = 2.0 };
        var result = Interop.meta_ambiguous_2(v);
        Assert.Equal(1.0, result.x);
        Assert.Equal(2.0, result.z);
    }

    [Fact]
    public void meta_ambiguous_3()
    {
        var v1 = new Vec1 { x = 5.0f, y = 0.0f };
        var v2 = new Vec2 { x = 5.0, z = 0.0 };
        var result = Interop.meta_ambiguous_3(v1, v2);
        Assert.True(result.Is);
    }

    [Fact]
    public void meta_ambiguous_3_false()
    {
        var v1 = new Vec1 { x = 5.0f, y = 0.0f };
        var v2 = new Vec2 { x = 100.0, z = 0.0 };
        var result = Interop.meta_ambiguous_3(v1, v2);
        Assert.False(result.Is);
    }

    [Fact]
    public void meta_renamed()
    {
        var s = new StructRenamed { e = EnumRenamed.X };
        var result = Interop.meta_renamed(s);
        Assert.Equal(EnumRenamed.X, result);
    }

    [Fact]
    public void meta_documented()
    {
        var s = new StructDocumented { x = 0 };
        var result = Interop.meta_documented(s);
        Assert.Equal(EnumDocumented.A, result);
    }
}