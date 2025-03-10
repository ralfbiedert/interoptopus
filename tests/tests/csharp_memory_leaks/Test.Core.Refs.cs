using My.Company;
using Xunit;

public class TestRefs
{
    public static void Check()
    {
        var x = 123l;

        Interop.ref1(ref x);
        Interop.ref2(ref x);
        Interop.ref3(ref x);
        Interop.ref4(ref x);
    }
}