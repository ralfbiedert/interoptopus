using My.Company.Common;
using Xunit;
using Interop = My.Company.Interop;

public class TestWireArray
{
    [Fact]
    public void wire_accept_byte_array()
    {
        var data = new byte[32];
        data[0] = 42;
        var result = Interop.wire_accept_byte_array(data.Wire());
        Assert.Equal(42, result);
    }

    [Fact]
    public void wire_return_byte_array()
    {
        using var wire = Interop.wire_return_byte_array();
        var result = wire.Unwire();
        Assert.Equal(32, result.Length);
        Assert.Equal(42, result[0]);
    }
}