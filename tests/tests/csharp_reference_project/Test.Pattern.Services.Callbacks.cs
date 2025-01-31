using System.Linq;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternServicesCallbacks
{

    [Fact]
    public void service_callbacks()
    {
        var callbacks = ServiceCallbacks.New();
        var called = false;

        callbacks.CallbackSimple((x) =>
        {
            called = true;
            Assert.Equal(x, 0u);
            return x;
        });

        Assert.True(called);
    }

    [Fact]
    public void service_callbacks_with_slice()
    {
        var callbacks = ServiceCallbacks.New();
        var called = false;
        var slice = new[] { 1, 2, 3 };

        callbacks.CallbackWithSlice((x, y) =>
        {
            Assert.Equal(x, 1);
            Assert.Equal(y, 2);
            called = true;
            return FFIError.Ok;
        }, slice);

        Assert.True(called);
    }

}
