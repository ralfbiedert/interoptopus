using My.Company;

namespace My.Company;

// User implementation
public class Plugin : IPlugin
{
    public static uint PrimitiveU32(uint x)
    {
        return x + 1;
    }

    public static ushort PrimitiveU16(ushort x)
    {
        return (ushort)(x + 1);
    }

    public static short PrimitiveI16(short x)
    {
        return (short)(x + 1);
    }

    public static double PrimitiveF64(double x)
    {
        return x + 1;
    }

    public static ulong PrimitiveU64(ulong x)
    {
        return x + 1;
    }

    public static long PrimitiveI64(long x)
    {
        return x + 1;
    }

    public static byte PrimitiveU8(byte x)
    {
        return (byte)(x + 1);
    }

    public static float PrimitiveF32(float x)
    {
        return x + 1;
    }

    public static sbyte PrimitiveI8(sbyte x)
    {
        return (sbyte)(x + 1);
    }

    public static int PrimitiveI32(int x)
    {
        return x + 1;
    }

    public static void PrimitiveVoid() { }
}
