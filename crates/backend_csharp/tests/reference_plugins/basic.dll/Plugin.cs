using My.Company;

namespace My.Company;

// User implementation
public class Plugin : IPlugin
{
    public static uint PrimitiveU32(uint x)
    {
        return x + 1;
    }

    public static void PrimitiveVoid() { }
}
