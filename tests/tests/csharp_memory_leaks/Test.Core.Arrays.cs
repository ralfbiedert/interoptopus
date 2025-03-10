using My.Company;
using Xunit;

public class TestArrays
{
    public static void Check()
    {
        var a = new Array { data = new byte[16] };
        var x = Interop.array_1(a);
    }
}