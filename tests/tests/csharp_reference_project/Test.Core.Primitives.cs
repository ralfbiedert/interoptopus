using My.Company;
using Xunit;

public class TestBasics
{

    [Fact]
    public void primitive_void()
    {
        Interop.primitive_void();
    }

    [Fact]
    public void primitive_u8()
    {
        Assert.Equal(255, Interop.primitive_u8(0));
    }

    [Fact]
    public void primitive_bool()
    {
        Assert.Equal(false, Interop.primitive_bool(true));
    }

    [Fact]
    public void primitive_i64()
    {
        Assert.Equal(-123, Interop.primitive_i64(123));
    }

    [Fact]
    public void primitive_f32()
    {
        Assert.Equal(-1.0, Interop.primitive_f32(1.0f));
    }

    [Fact]
    public void primitive_f64()
    {
        Assert.Equal(-1.0, Interop.primitive_f64(1.0f));
    }

}
