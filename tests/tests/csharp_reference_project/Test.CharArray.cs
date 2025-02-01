using My.Company;
using Xunit;

public class TestCharArray
{
    [Fact]
    public void Test_char_array_1()
    {
        var result = Interop.char_array_1();
        Assert.Equal("Hello, World!", result.str);
    }

    [Fact]
    public void Test_char_array_2()
    {
        var result = Interop.char_array_2(new CharArray
        {
            str = "Hello, World!"
        });
        Assert.Equal("Hello, World!", result.str);
    }

    [Fact]
    public void Test_char_array_2_throws()
    {
        Assert.Throws<System.InvalidOperationException>(() => Interop.char_array_2(new CharArray
        {
            str = "Hello, World! Hello, World! Hello, World! Hello, World!"
        }));
    }

    [Fact]
    public void Test_char_array_3()
    {
        var arr = new CharArray
        {
            str = "Hello, World!"
        };
        var result = Interop.char_array_3(ref arr);
        Assert.Equal((byte)'H', result);
    }
}