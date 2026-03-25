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

    public static Task<ResultNintError> CreateAAsyncResult()
    {
        throw new NotImplementedException();
    }

    public static ResultNintError CreateAResult()
    {
        throw new NotImplementedException();
    }
}

public class NestedA : INestedA<NestedA>
{
    public static NestedA NestedaCreate()
    {
        throw new NotImplementedException();
    }

    public static NestedA NestedaCreateResult()
    {
        throw new NotImplementedException();
    }

    public static NestedA NestedaCreateAsync()
    {
        throw new NotImplementedException();
    }

    public static NestedA NestedaCreateResultAsync()
    {
        throw new NotImplementedException();
    }

    public NestedB NestedaCreateOther()
    {
        throw new NotImplementedException();
    }

    public ResultNintError NestedaCreateOtherResult()
    {
        throw new NotImplementedException();
    }

    public Task<IntPtr> NestedaCreateOtherAsync()
    {
        throw new NotImplementedException();
    }

    public Task<ResultNintError> NestedaCreateOtherResultAsync()
    {
        throw new NotImplementedException();
    }
}

public class NestedB : INestedA<NestedB>
{
    public static NestedB NestedaCreate()
    {
        throw new NotImplementedException();
    }

    public static NestedB NestedaCreateResult()
    {
        throw new NotImplementedException();
    }

    public static NestedB NestedaCreateAsync()
    {
        throw new NotImplementedException();
    }

    public static NestedB NestedaCreateResultAsync()
    {
        throw new NotImplementedException();
    }

    public NestedB NestedaCreateOther()
    {
        throw new NotImplementedException();
    }

    public ResultNintError NestedaCreateOtherResult()
    {
        throw new NotImplementedException();
    }

    public Task<IntPtr> NestedaCreateOtherAsync()
    {
        throw new NotImplementedException();
    }

    public Task<ResultNintError> NestedaCreateOtherResultAsync()
    {
        throw new NotImplementedException();
    }
}