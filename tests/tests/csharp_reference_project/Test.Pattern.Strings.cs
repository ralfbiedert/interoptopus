using System.Linq;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternStrings
{
    [Fact]
    public void pattern_ascii_pointer_1()
    {
        var x = Interop.pattern_ascii_pointer_1("hello world");
        Assert.Equal(x, 11u);
    }

    [Fact]
    public void pattern_ascii_pointer_2()
    {
        // TODO: Why does this not codegen properly?
        var rval = Interop.pattern_ascii_pointer_2();
        // Assert.Equal(string.IsNullOrEmpty(rval));
    }


    [Fact]
    public void string_slices()
    {
        // TODO - This won't work as C# would need more special marshalling of the individual strings.
        // List<string> strings = ["a", "bb", "ccc"];
        // var total_length  = Interop.pattern_ffi_slice_7(strings.ToArray());
        // Assert.Equal(total_length, 6u);
    }
}
