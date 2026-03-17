using My.Company;
using My.Company.Common;
using Xunit;
using Interop = My.Company.Interop;

public class TestWireBasic
{
    [Fact]
    public void wire_accept_string_1()
    {
        var x = WireOfString.From("hello world");
        Interop.wire_accept_string_1(x);
    }

    [Fact]
    public void wire_accept_string_2()
    {
        var x = new MyString
        {
            x = "hello world"
        }.Wire();

        Interop.wire_accept_string_2(x);
    }
}