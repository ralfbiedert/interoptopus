using My.Company;
using Newtonsoft.Json;

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

    public static WireOfString WireString(WireOfString nested)
    {
        var s = nested.Unwire();
        var dict = JsonConvert.DeserializeObject<Dictionary<string, string>>(s) ?? new();
        dict["hello"] = "world";
        return WireOfString.From(JsonConvert.SerializeObject(dict));
    }
}
