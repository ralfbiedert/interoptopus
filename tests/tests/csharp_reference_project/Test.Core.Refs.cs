using System.Runtime.InteropServices;
using My.Company;
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

}