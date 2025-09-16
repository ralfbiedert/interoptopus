using System.Runtime.InteropServices;

namespace Plugin;


// User implementation
public class Plugin : IPlugin
{
    public static long DoMath(long a, long b)
    {
        return a + b;
    }
}