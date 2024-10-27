using System;
using System.Linq;
using System.Threading;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternDelegateTable
{
    DelegateTable NewTable()
    {
        var sum_delegate_return = new SumDelegateReturnExceptionSafe((x, y) => FFIError.Ok);
        return new DelegateTable
        {
            my_callback = value => 1,
            sum_delegate_1 = () => { },
            sum_delegate_2 = (x, y) => x + y,
            sum_delegate_return = sum_delegate_return.Call
        };
    }

    void CreatePatternDelegateTable(ServiceCallbacks service)
    {
        var table = NewTable();
        service.SetCallbackTable(ref table);
    }

    [Fact]
    public void set_delegates()
    {
        var service = ServiceCallbacks.New();

        CreatePatternDelegateTable(service); // TODO
        // ^-- this might run at risk of our delegates getting GC'ed if we don't add
        // special handling. For example, if we don't do GC.Collect() it might just pass
        // because the delegates didn't get cleaned up, but once you comment the line
        // below out it'll crash.
        //
        // GC.Collect();

        service.InvokeCallbacks();
    }


    [Fact]
    public void new_with_delegates()
    {
        var table = NewTable();
        var service = ServiceCallbacks.NewWithTable(table);
        service.InvokeCallbacks();
    }
}