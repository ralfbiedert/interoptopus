using System.Runtime.InteropServices;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestRefs
{
    [Fact]
    public void ref1()
    {
        var x = 123l;
        var rval = Interop.ref1(ref x);
        Assert.Equal(123l, Marshal.ReadInt64(rval));
    }

    [Fact]
    public void ref2()
    {
        var x = 123l;
        var rval = Interop.ref2(ref x);
        Assert.Equal(-123l, Marshal.ReadInt64(rval));
        Assert.Equal(-123l, x);
    }

    [Fact]
    public void ref3()
    {
        var x = 123l;
        var rval = Interop.ref3(ref x);
        Assert.Equal(true, rval);
    }

    [Fact]
    public void ref4()
    {
        var x = 123l;
        var rval = Interop.ref4(ref x);
        Assert.Equal(true, rval);
    }

    [Fact]
    public void ref5()
    {
        var v = new Vec3f32();
        var x = EnumPayload.B(v);

        Interop.ref5(ref x);

        Assert.Equal(123u, x.AsC());
    }

    [Fact]
    public void ref6()
    {
        var x = OptionEnumPayload.None;

        Interop.ref6(ref x);

        Assert.Equal(123u, x.AsSome().AsC());
    }

    [Fact]
    public void ref7()
    {
        var x = new[] { "a".Utf8() }.IntoVec();

        Interop.ref7(ref x);

        Assert.Equal(3, x.Count);
        Assert.Equal("1", x[0].IntoString());
        Assert.Equal("2", x[1].IntoString());
        Assert.Equal("3", x[2].IntoString());
    }

}