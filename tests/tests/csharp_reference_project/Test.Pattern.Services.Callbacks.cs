using System;
using My.Company;
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
        }).Ok();

        Assert.True(called);
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
            return ResultError.OK;
        }, slice).Ok();

        Assert.True(called);
    }

}
