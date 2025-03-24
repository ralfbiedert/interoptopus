using System;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternServicesCallbacks
{

    [Fact]
    public void CallbackSimple()
    {
        var callbacks = ServiceCallbacks.New();
        var called = false;

        callbacks.CallbackSimple(x =>
        {
            called = true;
            Assert.Equal(x, 0u);
            return x;
        }).AsOk();

        Assert.True(called);
        Interop.service_callbacks_callback_simple(callbacks.Context, x => x);
        Interop.service_callbacks_destroy(IntPtr.Zero);
        Interop.service_callbacks_destroy(IntPtr.Zero);
        Interop.service_callbacks_destroy(IntPtr.Zero);
        Interop.service_callbacks_destroy(IntPtr.Zero);
        Interop.service_callbacks_destroy(IntPtr.Zero);
        Interop.service_callbacks_destroy(callbacks.Context);
        // callbacks.Dispose();
    }

    [Fact]
    public void CallbackWithSlice()
    {
        var callbacks = ServiceCallbacks.New();
        var called = false;
        var slice = new[] { 1, 2, 3 };


        callbacks.CallbackWithSlice((x, y) =>
        {
            Assert.Equal(x, 1);
            Assert.Equal(y, 2);
            called = true;
            return ResultError.Ok;
        }, slice).AsOk();

        Assert.True(called);
        callbacks.Dispose();
    }

}
