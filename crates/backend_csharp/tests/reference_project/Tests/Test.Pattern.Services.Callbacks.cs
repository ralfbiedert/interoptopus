using System;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternServicesCallbacks
{

    [Fact]
    public void CallbackSimple()
    {
        using var callbacks = ServiceCallbacks.Create();
        var called = false;

        callbacks.CallbackSimple(x =>
        {
            called = true;
            Assert.Equal(x, 0u);
            return x;
        });

        Assert.True(called);
    }

    [Fact]
    public void CallbackWithSlice()
    {
        using var callbacks = ServiceCallbacks.Create();
        var called = false;
        using var slice = new[] { 1, 2, 3 }.Slice();

        callbacks.CallbackWithSlice((x, y) =>
        {
            Assert.Equal(x, 1);
            Assert.Equal(y, 2);
            called = true;
            return ResultVoidError.Ok;
        }, slice);

        Assert.True(called);
    }

    [Fact]
    public void CallbackFfiReturn()
    {
        using var service = ServiceCallbacks.Create();

        service.CallbackFfiReturn((x, y) => ResultVoidError.Ok);
    }

    /// <summary>
    /// Verify that a callback passed to a service constructor survives GC.
    /// Without _prevent_gc in the generated service class, the managed delegate
    /// backing the callback's function pointer would be collected, and the
    /// subsequent invoke would crash with "callback was made on a garbage
    /// collected delegate".
    /// </summary>
    [Fact]
    public void ConstructorCallbackSurvivesGC()
    {
        var service = ServiceCallbacks.CreateWithCallback(x => x + 100);

        GC.Collect();
        GC.WaitForPendingFinalizers();
        GC.Collect();

        var result = service.InvokeStoredCallback(42);

        Assert.Equal(142u, result);
        service.Dispose();
    }
}
