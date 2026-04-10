using My.Company;
using Xunit;
using Interop = My.Company.Interop;

public class TestWireEnum
{
    [Fact]
    public void wire_accept_enum_a()
    {
        var result = Interop.wire_accept_enum_1(MyEnum.A.Wire());
        Assert.Equal(0u, result);
    }

    [Fact]
    public void wire_accept_enum_b()
    {
        var result = Interop.wire_accept_enum_1(MyEnum.B.Wire());
        Assert.Equal(1u, result);
    }

    [Fact]
    public void wire_accept_enum_c()
    {
        var result = Interop.wire_accept_enum_1(MyEnum.C.Wire());
        Assert.Equal(2u, result);
    }
}