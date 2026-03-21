using My.Company;

namespace My.Company;

// User implementation
public class Plugin : IPlugin
{
    public static EnumPayload EnumPayload(EnumPayload nested)
    {
        return Company.EnumPayload.B(new Vec3f32
        {
            x = 1,
            y = 2,
            z = 3
        });
    }

    public static Vec3f32 Vec3f32(Vec3f32 nested)
    {
        (nested.x, nested.y) = (nested.y, nested.x);
        return nested;
    }
}
