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
    
    [Fact]
    public void string_utf8_passthrough()
    {
        var i = new Utf8String("hello world");
        var o = Interop.pattern_string_1(i);
        Assert.Equal(o.String, "hello world");
    }

    [Fact]
    public void string_utf8_in_out()
    {
        var s = Interop.pattern_string_3();
        Assert.Equal(s.String, "pattern_string_3");

        var l = Interop.pattern_string_2(s);
        Assert.Equal(l, 16u);
    }

}
