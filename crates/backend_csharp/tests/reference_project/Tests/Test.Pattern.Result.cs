using System;
using My.Company;
using My.Company.Common;
using Xunit;
using Interop = My.Company.Interop;

public class TestPatternResult
{
    [Fact]
    public void pattern_result_1()
    {
        var x = new ResultUintError();
        Interop.pattern_result_1(x).AsOk();
    }

    [Fact]
    public void pattern_result_2()
    {
        var result = Interop.pattern_result_2();
        Assert.True(result.IsOk);
    }

    [Fact]
    public void pattern_result_3()
    {
        Assert.True(Interop.pattern_result_3(ResultVoidError.Ok).IsOk);
        Assert.Equal(ResultVoidError.Ok, Interop.pattern_result_3(ResultVoidError.Ok));
        Assert.Equal(ResultVoidError.Null, Interop.pattern_result_3(ResultVoidError.Null));
    }

    [Fact]
    public void pattern_result_4()
    {
        Assert.True(Interop.pattern_result_4(ResultVoidVoid.Ok).IsOk);
    }

    [Fact]
    public void pattern_string_5()
    {
        var w = new UseString { s1 = "hello".Utf8(), s2 = "world".Utf8() };
        var result = Interop.pattern_string_5(w);
        var ok = result.AsOk();
        Assert.Equal("hello", ok.s1.String);
        Assert.Equal("world", ok.s2.String);
    }

    [Fact]
    public void pattern_result_interface()
    {
        var result = Interop.pattern_result_2();
        IResult<Unit, Error> resultInterface = result;
        Assert.True(resultInterface.IsOk);
        Assert.False(resultInterface.IsErr);
        Assert.False(resultInterface.IsPanic);
        Assert.False(resultInterface.IsNull);

        resultInterface.AsOk();
        Assert.Throws<EnumException>(() => resultInterface.AsErr());
    }

    [Fact]
    public void pattern_result_custom_matches()
    {
        var resultVoidVoid = ResultVoidVoid.Ok;
        var resultVoidError = Interop.pattern_result_2();
        var resultUIntError = ResultUintError.Ok(42);

        var match1 = my_match(resultVoidVoid, v => v.ToString(), e => e.ToString());
        var match2 = my_match(resultVoidError, v => v.ToString(), e => e.ToString());
        var match3 = my_match(resultUIntError, v => v.ToString(), e => e.ToString());

        Assert.Equal("()", match1);
        Assert.Equal("()", match2);
        Assert.Equal("42", match3);
    }

    private static TResult my_match<T, TErr, TResult>(
        IResult<T, TErr> res,
        Func<T, TResult> okMatcher,
        Func<TErr, TResult> errMatcher
    )
    {
        if (res.IsOk)
            return okMatcher(res.AsOk());
        if (res.IsErr)
            return errMatcher(res.AsErr());
        throw res.ExceptionForVariant();
    }
}
