using System.Linq;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternStrings
{
    [Fact]
    public void string_slices()
    {
        // TODO - This won't work as C# would need more special marshalling of the individual strings.
        // List<string> strings = ["a", "bb", "ccc"];
        // var total_length  = Interop.pattern_ffi_slice_7(strings.ToArray());
        // Assert.Equal(total_length, 6u);
    }
}
