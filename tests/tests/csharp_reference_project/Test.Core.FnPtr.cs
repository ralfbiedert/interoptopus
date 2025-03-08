using My.Company;
using My.Company.Common;
using Xunit;

public class TestCoreFnPtr
{
    [Fact]
    public void callback()
    {
        var result = Interop.fnptr_1((x) =>
        {
            Assert.Equal(42, x);
            return 43;
        }, 42);

        Assert.Equal(43, result);
    }

    // [Fact]
    // public void callback_marshalled()
    // {
    //     Interop.callback_marshalled((x) =>
    //     {
    //         Assert.Equal("test", x.str);
    //     }, new CharArray
    //     {
    //         str = "test",
    //         str_2 = ""
    //     });
    // }
}