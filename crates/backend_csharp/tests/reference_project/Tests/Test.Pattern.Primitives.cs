using My.Company.Common;
using Xunit;
using Interop = My.Company.Interop;

public class TestPatternPrimitives
{
    [Fact]
    public void pattern_ffi_bool()
    {
        Assert.Equal(Bool.False, Interop.pattern_ffi_bool(Bool.True));
        Assert.Equal(Bool.True, Interop.pattern_ffi_bool(Bool.False));
    }

    [Fact]
    public void pattern_ffi_cchar()
    {
        var c = (byte)'A';
        Assert.Equal(c, Interop.pattern_ffi_cchar(c));
    }

    [Fact]
    public void pattern_ascii_pointer_5()
    {
        var result = Interop.pattern_ascii_pointer_5("hello", 1);
        Assert.Equal((byte)'e', result);
    }
}