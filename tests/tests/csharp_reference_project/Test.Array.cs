using System.Linq;
using My.Company;
using Xunit;

public class TestArray
{
    [Theory]
    [InlineData(new byte[] { })]
    [InlineData(new byte[] { 1, 2, 3 })]
    [InlineData(new byte[] { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16 })]
    [InlineData(new byte[] { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17 })]
    [InlineData(null)]
    public void Test_array_1(byte[] array)
    {
        Assert.Equal((array?.Length ?? 0) > 0 ? 1 : 0, Interop.array_1(new Array
        {
            data = array
        }));
    }

    [Fact]
    public void Test_array_2()
    {
        var result = Interop.array_2();
        Assert.Equal(Enumerable.Range(1, 16).Select(i => (byte)i).ToArray(), result.data);
    }

    [Fact]
    public void Test_array_3()
    {
        Interop.array_3(out var result);
        Assert.Equal(42, result.data[0]);
    }
}