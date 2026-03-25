using My.Company;
using My.Company.Common;

namespace My.Company;

class Plugin : IPlugin
{
    public static NestedA CreateA()
    {
        throw new NotImplementedException();
    }

    public static Task<IntPtr> CreateAAsync()
    {
        throw new NotImplementedException();
    }

    public static Task<ResultNestedAError> CreateAAsyncResult()
    {
        throw new NotImplementedException();
    }

    public static ResultNestedAError CreateAResult()
    {
        throw new NotImplementedException();
    }
}

public class NestedA : INestedA<NestedA>
{
  
}

public class NestedB : INestedA<NestedB>
{
    public static NestedB NestedaCreate()
  
}