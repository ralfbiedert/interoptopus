using System;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternStrings
{
    [Fact]
    public void pattern_ascii_pointer_1()
    {
        var x = new Utf8String("hello world");
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
        Assert.Equal("hello world", Interop.pattern_string_1("hello world").String);
        Assert.Equal("hello world", Interop.pattern_string_1("hello world".Utf8()).String);
    }

    [Fact]
    public void pattern_string_2()
    {
        Assert.Equal(11u, Interop.pattern_string_2("hello world"));
    }

    [Fact]
    public void pattern_string_3()
    {
        Assert.Equal("pattern_string_3", Interop.pattern_string_3().String);
    }

    [Fact]
    public void pattern_string_4()
    {
        var w = new UseString { s1 = "hello", s2 = "world" };
        var s = Interop.pattern_string_4(w);
        Assert.Equal("hello", s.s1);
        Assert.Equal("world", s.s2);
    }

    [Fact]
    public void pattern_string_6()
    {
        var r1 = new UseString { s1 = "hello", s2 = "world" };
        Interop.pattern_string_6a(ref r1);

        var y = new UseString { s1 = "", s2 = "" };
        Interop.pattern_string_6b(ref y).AsOk();
        Assert.Equal("s1", y.s1);
        Assert.Equal("s2", y.s2);
    }

    [Fact]
    public void pattern_string_7()
    {
        var r1 = Interop.pattern_string_7(["hello", "world"], 0).AsOk();
        var r2 = Interop.pattern_string_7(["hello", "world"], 1).AsOk();
        Assert.Equal("hello", r1);
        Assert.Equal("world", r2);
    }

    [Fact]
    public void pattern_string_8()
    {
        var x = new UseString[]
        {
            new() { s1 = "hello1", s2 = "world1" },
            new() { s1 = "hello2", s2 = "world2" },
        };

        var r1 = Interop.pattern_string_8(x, 0).AsOk();
        var r2 = Interop.pattern_string_8(x, 1).AsOk();

        Assert.Equal("hello1", r1.s1);
        Assert.Equal("world2", r2.s2);
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
        var w = new UseString { s1 = "hello", s2 = "world" };
        for (var i = 0; i < 1024 * 1024; i++)
        {
            Interop.pattern_string_6a(ref w);
        }
    }

    [Fact]
    public void string_by_out_dont_leak()
    {
        // TODO - Can we somehow measure memory use?
        var w = new UseString { s1 = "hello", s2 = "world" };
        for (var i = 0; i < 1024 * 1024; i++)
        {
            var r2 = Interop.pattern_string_6b(ref w);
        }
    }
}
