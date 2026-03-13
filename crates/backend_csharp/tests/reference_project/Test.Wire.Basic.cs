using My.Company;
using My.Company.Common;
using Xunit;

public class TestWireBasic
{
    [Fact]
    public void wire_accept_string_2()
    {
        var x = new MyString("hello world");
        var wired = x.Wire();

        Interop.wire_accept_string_2(wired);

        wired.Dispose();
    }
}