using My.Company;

namespace My.Company;

// User implementation
public class Plugin : IPlugin
{
    public static ResultVec3f32Error Result(ResultVec3f32Error res)
    {
        return ResultVec3f32Error.Ok(new Vec3f32());
    }
}
