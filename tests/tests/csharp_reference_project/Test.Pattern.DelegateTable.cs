using System;
using System.Linq;
using System.Threading;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternDelegateTable
{
    void CreatePatternDelegateTable(ServiceCallbacks service)
    {
    
        var table = new DelegateTableX
        {
            my_callback = new MyCallback(value => 1),
            // sum_delegate_1 = () => { },
            sum_delegate_2 = new SumDelegate2((x, y) => x + y),
            sum_delegate_return = new SumDelegateReturn(((i, i1) => FFIError.Ok)),
        };
        service.SetDelegateTable(ref table);
    }

    [Fact]
    public void pattern_ffi_slice_delegate()
    {
        var service = ServiceCallbacks.New();
    
        CreatePatternDelegateTable(service); // TODO
        // ^-- this might run at risk of our delegates getting GC'ed if we don't add
        // special handling. For example, if we don't do GC.Collect() it might just pass
        // because the delegates didn't get cleaned up, but once you comment the line
        // below out it'll crash.
        //
        // GC.Collect();
    
        service.InvokeDelegates();
    }

}