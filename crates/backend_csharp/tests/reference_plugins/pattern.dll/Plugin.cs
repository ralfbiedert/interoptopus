using My.Company;
using My.Company.Common;

namespace My.Company;



// User implementation
public class Plugin : IPlugin
{
    // We need to pin the returned delegate instance here, otherwise C# might 
    // get rid of the class and dispose its allocated interop handle. 
    static SumDelegate2? Instance;
    
    public static SumDelegate2 Delegate1(MyCallback res)
    {
        Instance = new((x, y) => (int)res.Call((uint)(x + y))); 
        return Instance;
    }

    public static ResultVec3f32Error Result(ResultVec3f32Error res)
    {
        return ResultVec3f32Error.Ok(new Vec3f32 { x = 0, y = 0, z = 0 });
    }
}
