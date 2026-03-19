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
public partial class Foo
{
    private int _accumulator;

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
