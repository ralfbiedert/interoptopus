using My.Company;
using Xunit;

public class TestPrimitives
{
    public static void Check()
    {
        Interop.primitive_i8(0);
        Interop.primitive_bool(true);
        Interop.primitive_f32(1.0f);
    }
}