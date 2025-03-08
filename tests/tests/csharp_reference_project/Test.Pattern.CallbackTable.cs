using System;
using System.Linq;
using System.Threading;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternDelegateTable
{
    private CallbackTable _table;

    void CreatePatternDelegateTable(ServiceCallbacks service)
    {
        _table = new CallbackTable
        {
            my_callback = new MyCallback(value => 1),
            sum_delegate_1 = new SumDelegate1(() => { }),
            sum_delegate_2 = new SumDelegate2((x, y) => x + y),
            sum_delegate_return = new SumDelegateReturn(((i, i1) => new ResultFFIError(FFIError.Ok))),
        };
        service.SetDelegateTable(_table);
    }

    [Fact]
    public void InvokeDelegates()
    {
        var service = ServiceCallbacks.New();
    
        CreatePatternDelegateTable(service);
        // ^-- this might run at risk of our delegates getting GC'ed if we don't add
        // special handling. For example, if we don't do GC.Collect() it might just pass
        // because the delegates didn't get cleaned up, but once you comment the line
        // below out it'll crash.
        //
        GC.Collect();
    
        service.InvokeDelegates();
    }

}