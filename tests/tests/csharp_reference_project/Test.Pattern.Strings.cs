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
    public void pattern_string_1()
    {
        var i = new Utf8String("hello world");
        var o = Interop.pattern_string_1(i);
        Assert.Equal(o.String, "hello world");
    }

    [Fact]
    public void pattern_string_2()
    {
        var l = Interop.pattern_string_2("hello world");
        Assert.Equal(l, 11u);
    }

    [Fact]
    public void pattern_string_3()
    {
        var s = Interop.pattern_string_3();
        Assert.Equal(s.String, "pattern_string_3");
    }


    [Fact]
    public void pattern_string_4()
    {
        var w = new UseUtf8String { s1 = "hello", s2 = "world" };
        var s = Interop.pattern_string_4(w);
        Assert.Equal(s.s1, "hello");
        Assert.Equal(s.s2, "world");
    }

    [Fact]
    public void pattern_string_6()
    {
        var w = new UseUtf8String { s1 = "hello", s2 = "world" };
        Interop.pattern_string_6a(ref w);

        // var y = new UseUtf8String();
        var r2 = Interop.pattern_string_6b(out var y);
        Assert.Equal(y.s1, "s1");
        Assert.Equal(y.s2, "s2");
    }

    [Fact]
    public void pattern_string_7()
    {
        // TODO
        var a = new Utf8String("hello");
        var b = new Utf8String("world");
        var r1 = Interop.pattern_string_7([a, b], 0).Ok();
        var r2 = Interop.pattern_string_7([a, b], 1).Ok();
        Assert.Equal(r1, "hello");
        Assert.Equal(r2, "world");
    }


    [Fact]
    public void string_by_ref_dont_leak()
    {
        // TODO - Can we somehow measure memory use?
        var w = new UseUtf8String { s1 = "hello", s2 = "world" };
        for (var i = 0; i < 1024 * 1024; i++)
        {
            Interop.pattern_string_6a(ref w);
        }
    }

    [Fact]
    public void string_by_out_dont_leak()
    {
        // TODO - Can we somehow measure memory use?
        var w = new UseUtf8String { s1 = "hello", s2 = "world" };
        for (var i = 0; i < 1024 * 1024; i++)
        {
            var r2 = Interop.pattern_string_6b(out var y);
        }
    }
}
