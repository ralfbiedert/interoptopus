namespace Plugin;

// User implementation
public class Plugin : IPlugin
{
    public static long DoMath(long a, long b)
    {
        return a + b;
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
}
