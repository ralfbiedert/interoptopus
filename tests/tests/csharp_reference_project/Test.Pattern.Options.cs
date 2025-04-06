using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternOptions
{
    [Fact]
    public void pattern_ffi_option_1()
    {
        var option = OptionInner.Some(new Inner { x = 123.0f });
        Assert.Equal(123.0f, Interop.pattern_ffi_option_1(option).AsSome().x);
    }

    [Fact]
    public void pattern_ffi_option_2()
    {
        Assert.True(float.IsNaN(Interop.pattern_ffi_option_2(OptionInner.None).x));
    }

    [Fact]
    public void pattern_ffi_option_3()
    {
        // Don't try this at home
        var x = OptionOptionResultOptionUtf8StringError.Some(
            OptionResultOptionUtf8StringError.Some(
                ResultOptionUtf8StringError.Ok(OptionUtf8String.Some("hello world".Utf8()))
            )
        );

        var rval = Interop.pattern_ffi_option_3(x).AsSome().AsSome().AsOk().AsSome();
        Assert.Equal("hello world", rval.IntoString());
    }

}
