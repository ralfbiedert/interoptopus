namespace My.Company;

// User implementation
public class Plugin : IPlugin
{
    // Ok, if this is uncaught before reaching [UnmanagedCallersOnly] .NET
    // will crash the entire application. 
    public static void Panic()
    {
        throw new Exception("Panic!");
    }

    public static Task<uint> PanicAsyncWithRval(CancellationToken ct)
    {
        throw new NotImplementedException();
    }

    public static ResultVoidError PanicWithResult()
    {
        throw new Exception("Panic!");
    }

    public static uint PanicWithRval()
    {
        throw new NotImplementedException();
    }

    public static async Task PanicAsync(CancellationToken ct)
    {
        await Task.Yield();
        throw new Exception("Panic async!");
    }

    public static async Task<ResultVoidError> PanicAsyncWithResult(CancellationToken ct)
    {
        await Task.Yield();
        throw new Exception("Panic async with result!");
    }
}
