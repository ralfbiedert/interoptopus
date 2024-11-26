using My.Company;
using Xunit;

public class TestPatternServicesCallbacks
{
    CallbackTable NewTable()
    {
        return new CallbackTable
        {
            error = (x, y) => FFIError.Ok,
            callback = (x) => x,
            namespaced = (x) => x
        };
    }

    void CreatePatternDelegateTable(ServiceCallbacksTable service)
    {
        var table = NewTable();
        service.SetCallbackTable(ref table);
    }

    [Fact]
    public void set_delegates()
    {
        var service = ServiceCallbacksTable.New();

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
        var service = ServiceCallbacksTable.NewWithTable(table);
        service.InvokeCallbacks();
    }
}