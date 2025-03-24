using My.Company;
using Xunit;

public class TestEnums
{
    [Fact]
    public void enum1()
    {
        Interop.enums_1(EnumPayload.A);
        Interop.enums_1(EnumPayload.B(new Vec3f32()));
        Interop.enums_1(EnumPayload.C(123));
    }

    [Fact]
    public void enum2()
    {
        var v1 = new Vec3f32
        {
            x = 1.0f,
            y = 2.0f,
            z = 3.0f
        };

        var v2 = new Vec3f32
        {
            x = 2.0f,
            y = 4.0f,
            z = 6.0f
        };

        var r1 = Interop.enums_2(EnumPayload.A);
        var r2 = Interop.enums_2(EnumPayload.B(v1));
        var r3 = Interop.enums_2(EnumPayload.C(123));

        Assert.Equal(EnumPayload.A, r1);
        Assert.Equal(EnumPayload.B(v2), r2);
        Assert.Equal(EnumPayload.C(246), r3);
    }

    [Fact]
    public void enum3()
    {
        var v1 = new Vec3f32
        {
            x = 1.0f,
            y = 2.0f,
            z = 3.0f
        };

        var v2 = new Vec3f32
        {
            x = 2.0f,
            y = 4.0f,
            z = 6.0f
        };

        var e1 = EnumPayload.A;
        var e2 = EnumPayload.B(v1);
        var e3 = EnumPayload.C(123);


        Interop.enums_3(ref e1);
        Interop.enums_3(ref e2);
        Interop.enums_3(ref e3);

        Assert.Equal(EnumPayload.A, e1);
        Assert.Equal(v2, e2.AsB());
        Assert.Equal(246u, e3.AsC());
    }

}