using My.Company;

namespace My.Company;

// User implementation
public class Plugin : IPlugin
{
    public static long DoMath(long a, long b)
    {
        return a + b;
    }

    public static ResultVec3f32ErrorABC SumAll(long x, long y, uint z)
    {

        var vec = new Vec3f32
        {
            x = (float)x,
            y = (float)y,
            z = (float)z,
        };

        return ResultVec3f32ErrorABC.Ok(vec);
    }
}

// User-written class with state
public class Foo : IFoo<Foo>
{
    private int _accumulator;

    public static Foo Create() => new Foo();

    public Foo()
    {
        _accumulator = 0;
    }

    public void Bar(int x)
    {
        _accumulator += x;
    }

    public int GetAccumulator()
    {
        return _accumulator;
    }

    public WireOfString Wire(WireOfString x)
    {
        var xx = x.Unwire().ToLower();
        return WireOfString.From(xx);
    }

    public WireOfHashMapStringString Wire2(WireOfHashMapStringString x)
    {
        var xx = x.Unwire();
        xx["hello"] = "world";
        return WireOfHashMapStringString.From(xx);
    }
}
