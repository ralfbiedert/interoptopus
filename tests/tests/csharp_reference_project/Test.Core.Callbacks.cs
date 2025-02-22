using My.Company;
using My.Company.Common;
using Xunit;

public class TestCoreCallbacks
{
    [Fact]
    public void callback()
    {
        var result = Interop.callback((x) =>
        {
            Assert.Equal(42, x);
            return 43;
        }, 42);

        Assert.Equal(43, result);
    }

    [Fact]
    public void pattern_callback_4()
    {
        var x = new MyCallbackNamespaced(value => value);
        var y = Interop.pattern_callback_4(x, 5);
        Assert.Equal(y, 5u);
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