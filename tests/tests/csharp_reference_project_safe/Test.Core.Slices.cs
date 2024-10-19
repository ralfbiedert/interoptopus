using My.Company;
using Xunit;

public class TestCoreSlices
{
    [Fact]
    public void pattern_ffi_slice_3()
    {
        var data = new byte[100_000];

        Interop.pattern_ffi_slice_3(data, x0 =>
        {
            x0[1] = 100;
        });

        Assert.Equal(data[0], 1);
        Assert.Equal(data[1], 100);
    }

}