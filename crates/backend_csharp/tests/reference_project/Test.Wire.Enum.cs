using My.Company;
using My.Company.Common;
using Xunit;
using Interop = My.Company.Interop;

public class TestWireEnum
{
    [Fact]
    public void wire_accept_enum_a()
    {
        var x = WireOfMyEnum.From(MyEnum.A);
        var result = Interop.wire_accept_enum_1(x);
        Assert.Equal(0u, result);
    }

    [Fact]
    public void wire_accept_enum_b()
    {
        var x = WireOfMyEnum.From(MyEnum.B);
        var result = Interop.wire_accept_enum_1(x);
        Assert.Equal(1u, result);
    }

    [Fact]
    public void wire_accept_enum_c()
    {
        var x = WireOfMyEnum.From(MyEnum.C);
        var result = Interop.wire_accept_enum_1(x);
        Assert.Equal(2u, result);
    }
}
