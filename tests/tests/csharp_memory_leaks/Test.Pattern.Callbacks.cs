using My.Company;
using Xunit;

public class TestPatternCallbacks
{
    public static void Check()
    {
        var x = 123;
        Interop.pattern_callback_1(value => value + 1, 0);
        Interop.pattern_callback_2(ptr => { });
        Interop.pattern_callback_4(value => value, 5);
        Interop.pattern_callback_5();
        Interop.pattern_ffi_slice_delegate(x => x[0]);

        try
        {
            Interop.pattern_callback_7((i, i1) => throw new System.Exception(), (i, i1) => { }, 0, 0, ref x);
        }
        catch { }

    }
}