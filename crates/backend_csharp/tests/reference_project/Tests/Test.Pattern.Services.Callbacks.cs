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

    [Fact]
    public void CallbackSurvivesGC()
    {
        var service = ServiceCallbacks.CreateWithCallback(x => x + 100);

        GC.Collect();
        GC.WaitForPendingFinalizers();
        GC.Collect();

        var result = service.InvokeStoredCallback(42);

        Assert.Equal(142u, result);
        service.Dispose();
    }

    [Fact]
    public void CallbackDisposePreventsFurtherCalls()
    {
        var cb = new MyCallback(x => x * 2);
        Assert.Equal(10u, cb.Call(5));
        cb.Dispose();
        Assert.Throws<ObjectDisposedException>(() => cb.Call(5));
    }

    [Fact]
    public void CallbackCreateDisposeLoop()
    {
        for (var i = 0; i < 100_000; i++)
        {
            using var cb = new MyCallback(x => x);
            Assert.Equal(42u, cb.Call(42));
        }
    }

    [Fact]
    public void CallbackExceptionPropagatesFromCall()
    {
        using var cb = new MyCallback(_ => throw new InvalidOperationException("boom"));
        Assert.Throws<InvalidOperationException>(() => cb.Call(0));
    }
}
