using My.Company;
using My.Company.Common;
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

    [Fact]
    public void enum4()
    {
        var l1 = new Layer1Utf8String()
        {
            maybe_1 = OptionUtf8String.None,
            maybe_2 = new VecUtf8String(new[]
            {
                "hello",
                "world"
            }),
            maybe_3 = "hello world"
        };
        var l2 = new Layer2Utf8String()
        {
            layer_1 = l1,
            strings = new VecUtf8String(new[]
            {
                "hello",
                "world"
            }),
            vec = new Vec3f32(),
            the_enum = EnumPayload.A
        };
        var l3 = Layer3.B(l2);

        Assert.Equal("hello world", Interop.enums_4(l3).String);
    }

}