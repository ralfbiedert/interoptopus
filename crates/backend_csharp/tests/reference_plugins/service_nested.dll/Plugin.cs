using My.Company;
using My.Company.Common;

namespace My.Company;

class Plugin : IPlugin
{
    public static NestedA CreateA()
    {
        throw new NotImplementedException();
    }

    public static Task<NestedA> CreateAAsync()
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

public partial class NestedA : INestedA<NestedA>
{
    public static NestedA NestedaCreate()
    {
        throw new NotImplementedException();
    }

    public static NestedA NestedaCreateResult()
    {
        throw new NotImplementedException();
    }

    public static Task<NestedA> NestedaCreateAsync()
    {
        throw new NotImplementedException();
    }

    public static Task<NestedA> NestedaCreateResultAsync()
    {
        throw new NotImplementedException();
    }

    public NestedB NestedaCreateOther()
    {
        throw new NotImplementedException();
    }

    public ResultNestedBError NestedaCreateOtherResult()
    {
        throw new NotImplementedException();
    }

    public Task<NestedB> NestedaCreateOtherAsync()
    {
        throw new NotImplementedException();
    }

    public Task<ResultNestedBError> NestedaCreateOtherResultAsync()
    {
        throw new NotImplementedException();
    }
}

public partial class NestedB : INestedB<NestedB>
{
    public void NestedbAccept(NestedA a)
    {
        throw new NotImplementedException();
    }

    public Task NestedbAcceptAsync(NestedA a)
    {
        throw new NotImplementedException();
    }

    public void NestedbAcceptRef(NestedA a)
    {
        throw new NotImplementedException();
    }

    public Task NestedbAcceptAsyncRef(NestedA a)
    {
        throw new NotImplementedException();
    }
}