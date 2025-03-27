using System;
using System.Linq;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternDelegates
{
    [Fact]
    public void pattern_callback_1_adhoc()
    {
        var x = Interop.pattern_callback_1(value => value + 1, 0);
        Assert.Equal(1u, x);
    }

    [Fact]
    public void pattern_callback_1_retained()
    {
        var cb = new MyCallback(value => value + 1);
        var x = Interop.pattern_callback_1(cb, 0);
        Assert.Equal(1u, x);
    }

    [Fact]
    public void pattern_callback_2()
    {
        var called = false;
        var cb = new MyCallbackVoid(ptr => { called = true; });
        var x = Interop.pattern_callback_2(cb);
        x.Call(IntPtr.Zero);

        // TODO
        // Assert.True(called);
    }



    [Fact]
    public void pattern_callback_4()
    {
        var x = new MyCallbackNamespaced(value => value);
        var y = Interop.pattern_callback_4(x, 5);
        Assert.Equal(y, 5u);
    }

    [Fact]
    public void pattern_callback_5()
    {
        var cb = Interop.pattern_callback_5();
        cb.Call();
    }


    [Fact]
    public void pattern_ffi_slice_delegate()
    {

        Interop.pattern_ffi_slice_delegate(x =>
        {
            Assert.Equal(x.Count, 10);
            Assert.Equal(x[0], 0);
            Assert.Equal(x[5], 5);

            // Test IEnumerable using LINQ
            var arr = x.ToArray();
            Assert.Equal(arr.Length, 10);
            Assert.Equal(arr[0], 0);
            Assert.Equal(arr[5], 5);

            return x[0];

        });
        // cb.Dispose();
    }

    [Fact]
    public void pattern_callback_7_exception_handling()
    {
        // This value will be changed to `6` before the callbacks are invoked, and
        // changed to `8` after callbacks return.
        var rval = 0;
        var called_c1 = false;
        var called_c2 = false;
        var called_exception = false;


        ResultError C1(int x, int y)
        {
            // This should see `6` here, and it does, because the function has set that value.
            Assert.Equal(rval, 6);
            called_c1 = true;

            // However, when we now throw we would expect .NET to unwind that exception back
            // into Rust (and therefore eventually observe rval to be `8`). When we use this
            // safe wrapper it does that, but only because we implemented special handling for
            // it for callbacks that return a FFIError.
            throw new Exception("We handled this");
        }

        void C2(int x, int y)
        {
            // This callback looks very similar. However, it does not come with special handling
            // support. When this throws, the .NET runtime will NOT(!!!) unwind back through
            // Rust but instead just return the stack up without ever calling Rust again. That means
            // in particular that any code that you would expect to run in Rust subsequent to the invocation
            // of this callback (esp. `drop` code and friends) will NOT fire, leading to unexpected
            // memory loss or worse.
            Assert.Equal(rval, 6);
            called_c2 = true;
            throw new Exception("Unchecked callback which we didn't handle. Comment this out and see the test fail.");
        };

        var cc1 = new SumDelegateReturn(C1);
        var cc2 = new SumDelegateReturn2(C2); 

        try
        {
            // Interop.pattern_callback_7(C1, C2, 3, 7, out rval);
            Interop.pattern_callback_7(cc1, cc2, 3, 7, ref rval).AsOk();
            cc1.Dispose();
            cc2.Dispose();
        }
        catch (Exception e)
        {
            // If everything works Rust code after invoking C1 and C2 is still executed,
            // setting this variable to `8`.
            called_exception = true;
            Assert.Equal(rval, 8);
        }

        Assert.True(called_c1);
        Assert.True(called_c2);
        Assert.True(called_exception);
    }

    [Fact]
    public void pattern_callback_8()
    {
        var r1 = string.Empty;
        var r2a = string.Empty;
        var r2b = string.Empty;

        Interop.pattern_callback_8((s) =>
        {
            r1 = s;
        }, s =>
        {
            r2a = s.s1;
            r2b = s.s2;
        }, "hello world");

        Assert.Equal("hello world", r1);
        Assert.Equal("hello world", r2a);
        Assert.Equal("hello world", r2b);
    }


}