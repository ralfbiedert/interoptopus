using System;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternStrings
{
    [Fact]
    public void pattern_ascii_pointer_1()
    {
        var x = Utf8String.From("hello world");
        Assert.Equal(11u, Interop.pattern_ascii_pointer_1("hello world"));
    }

    [Fact]
    public void pattern_ascii_pointer_2()
    {
        // TODO: Why does this not codegen properly?
        var rval = Interop.pattern_ascii_pointer_2();
        // Assert.Equal(string.IsNullOrEmpty(rval));
    }
    
    [Fact]
    public void pattern_string_1()
    {
        Assert.Equal("hello world", Interop.pattern_string_1("hello world".Utf8()).String);
        Assert.Equal("hello world", Interop.pattern_string_1("hello world".Utf8()).String);
    }

    [Fact]
    public void pattern_string_2()
    {
        Assert.Equal(11u, Interop.pattern_string_2("hello world".Utf8()));
    }

    [Fact]
    public void pattern_string_3()
    {
        Assert.Equal("pattern_string_3", Interop.pattern_string_3().String);
    }

    [Fact]
    public void pattern_string_4()
    {
        var w = new UseString { s1 = "hello".Utf8(), s2 = "world".Utf8() };
        var s = Interop.pattern_string_4(w);
        Assert.Equal("hello", s.s1.String);
        Assert.Equal("world", s.s2.String);
    }

    [Fact]
    public void pattern_string_6()
    {
        var r1 = new UseString { s1 = "hello".Utf8(), s2 = "world".Utf8() };
        Interop.pattern_string_6a(ref r1);

        var y = new UseString { s1 = "".Utf8(), s2 = "".Utf8() };
        Interop.pattern_string_6b(ref y).AsOk();
        Assert.Equal("s1", y.s1.String);
        Assert.Equal("s2", y.s2.String);
    }

    [Fact]
    public void pattern_string_7()
    {
        var slice = SliceUtf8String.From(new[] { "hello".Utf8(), "world".Utf8() });
        var r1 = Interop.pattern_string_7(slice, 0).AsOk();
        var r2 = Interop.pattern_string_7(slice, 1).AsOk();
        Assert.Equal("hello", r1.String);
        Assert.Equal("world", r2.String);
    }

    [Fact]
    public void pattern_string_8()
    {
        var x = new UseString[]
        {
            new() { s1 = "hello1".Utf8(), s2 = "world1".Utf8() },
            new() { s1 = "hello2".Utf8(), s2 = "world2".Utf8() },
        };

        var slice = SliceUseString.From(x);

        var r1 = Interop.pattern_string_8(slice, 0).AsOk();
        var r2 = Interop.pattern_string_8(slice, 1).AsOk();

        Assert.Equal("hello1", r1.s1.String);
        Assert.Equal("world2", r2.s2.String);
    }

    [Fact]
    public void pattern_string_9()
    {
        var rval = Interop.pattern_string_9();

        // Should not crash attempting to de-serialize a non-existing string

        Assert.Equal(rval.AsErr(), Error.Fail);
    }


    [Fact]
    public void string_by_ref_dont_leak()
    {
        // TODO - Can we somehow measure memory use?
        var w = new UseString { s1 = "hello".Utf8(), s2 = "world".Utf8() };
        for (var i = 0; i < 1024 * 1024; i++)
        {
            Interop.pattern_string_6a(ref w);
        }
    }

    [Fact]
    public void string_by_out_dont_leak()
    {
        // TODO - Can we somehow measure memory use?
        var w = new UseString { s1 = "hello".Utf8(), s2 = "world".Utf8() };
        for (var i = 0; i < 1024 * 1024; i++)
        {
            var r2 = Interop.pattern_string_6b(ref w);
        }
    }
}
