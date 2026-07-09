using System;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using My.Company;
using Xunit;
using Interop = My.Company.Interop;

public class TestBehavior
{
    public static bool SupportsSehPanicBoundary =>
        OperatingSystem.IsWindows() && RuntimeFeature.IsDynamicCodeSupported;

    [Fact(
        Skip = "Requires a runtime that exposes Rust panics as catchable SEH exceptions (only managed windows).",
        SkipUnless = nameof(SupportsSehPanicBoundary)
    )]
    public void behavior_panics()
    {
        Assert.Throws<SEHException>(Interop.behavior_panics);
    }

    [Fact]
    public void behavior_panics_via_result()
    {
        var v = Interop.behavior_panics_via_result();
        Assert.Equal(ResultVoidError.Panic, v);
    }

    [Fact]
    public void behavior_sleep()
    {
        Interop.behavior_sleep(10);
    }
}
