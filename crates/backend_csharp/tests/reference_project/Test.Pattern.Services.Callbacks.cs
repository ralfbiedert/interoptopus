using System;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternServicesCallbacks
{

    [Fact]
    public void CallbackSimple()
    {
        var callbacks = ServiceCallbacks.Create();
        var called = false;

        callbacks.CallbackSimple(x =>
        {
            called = true;
            Assert.Equal(x, 0u);
            return x;
        });

        Assert.True(called);
        callbacks.Dispose();
    }

    [Fact]
    public void CallbackWithSlice()
    {
        var callbacks = ServiceCallbacks.Create();
        var called = false;
        var slice = new[] { 1, 2, 3 }.Slice();

        callbacks.CallbackWithSlice((x, y) =>
        {
            Assert.Equal(x, 1);
            Assert.Equal(y, 2);
            called = true;
            return ResultVoidError.Ok;
        }, slice);

        Assert.True(called);
        slice.Dispose();
        callbacks.Dispose();
    }

    [Fact]
    public void CallbackFfiReturn()
    {
        var service = ServiceCallbacks.Create();

        service.CallbackFfiReturn((x, y) => ResultVoidError.Ok);

        service.Dispose();
    }
}
