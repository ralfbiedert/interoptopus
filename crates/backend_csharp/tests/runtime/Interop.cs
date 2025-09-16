using System.Runtime.InteropServices;

namespace Plugin;

// Interface (auto-generated)
public interface IPlugin
{
    static abstract long DoMath(long a, long b);
}

// Static wrapper (auto-generated)
public static class PluginExports
{
    [UnmanagedCallersOnly]
    public static long DoMath(long a, long b) => Plugin.DoMath(a, b);
}

