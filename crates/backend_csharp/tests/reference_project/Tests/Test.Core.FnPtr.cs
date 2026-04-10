using Xunit;
using Interop = My.Company.Interop;

public class TestFnPtr
{
    [Fact]
    public void fnptr_1()
    {
        var result = Interop.fnptr_1(x =>
        {
            Assert.Equal(42, x);
            return 43;
        }, 42);

        Assert.Equal(43, result);
    }

    // [Fact]
    // public void callback_marshalled()
    // {
    //     Interop.fnptr_2((x) =>
    //     {
    //         Assert.Equal("test", x.str);
    //     }, new CharArray
    //     {
    //         str = "test",
    //         str_2 = ""
    //     });
    // }
}