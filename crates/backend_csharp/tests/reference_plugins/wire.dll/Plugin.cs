using My.Company;

namespace My.Company;

// User implementation
public class Plugin : IPlugin
{
    public static WireOfHashMapStringString WireHashmapString(WireOfHashMapStringString nested)
    {
        var unwired = nested.Unwire();
        unwired["hello"] = "world";
        return WireOfHashMapStringString.From(unwired);
    }
}
