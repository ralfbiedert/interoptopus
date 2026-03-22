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

    public static ResultVoidError PanicWithResult()
    {
        throw new Exception("Panic!");
    }
}
