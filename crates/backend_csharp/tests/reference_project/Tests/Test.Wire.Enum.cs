using System.Collections.Generic;
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

    [Fact]
    public void wire_accept_enum_string()
    {
        var len = Interop.wire_accept_enum_2(DataEnum.S("hello").Wire());
        Assert.Equal(5u, len);
    }

    [Fact]
    public void wire_accept_enum_vec()
    {
        var len = Interop.wire_accept_enum_2(DataEnum.V([0, 1, 2]).Wire());
        Assert.Equal(3u, len);
    }

    [Fact]
    public void wire_accept_enum_hashmap()
    {
        var dict = new Dictionary<string, MyEnum>
        {
            ["a"] = MyEnum.A,
            ["b"] = MyEnum.B,
        };
        var len = Interop.wire_accept_enum_2(DataEnum.H(dict).Wire());
        Assert.Equal(2u, len);
    }
}