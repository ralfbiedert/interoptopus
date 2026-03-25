using My.Company;
using My.Company.Common;

namespace My.Company;

class Plugin : IPlugin
{
    public static NestedA CreateA()
    {
        return new NestedA(10);
    }

    public static Task<NestedA> CreateAAsync()
    {
        return Task.FromResult(new NestedA(20));
    }

    public static Task<ResultNestedAError> CreateAAsyncResult()
    {
        return Task.FromResult(ResultNestedAError.Ok(new NestedA(30).IntoUnmanaged()._handle));
    }

    public static ResultNestedAError CreateAResult()
    {
        return ResultNestedAError.Ok(new NestedA(40).IntoUnmanaged()._handle);
    }
}

public partial class NestedA : INestedA<NestedA>
{
    public int Value { get; }

    public NestedA() { Value = 0; }
    public NestedA(int value) { Value = value; }

    public static NestedA NestedaCreate()
    {
        return new NestedA(100);
    }

    public static NestedA NestedaCreateResult()
    {
        return new NestedA(200);
    }

    public static Task<NestedA> NestedaCreateAsync()
    {
        return Task.FromResult(new NestedA(300));
    }

    public static Task<NestedA> NestedaCreateResultAsync()
    {
        return Task.FromResult(new NestedA(400));
    }

    public NestedB NestedaCreateOther()
    {
        return new NestedB(Value + 1);
    }

    public ResultNestedBError NestedaCreateOtherResult()
    {
        return ResultNestedBError.Ok(new NestedB(Value + 2).IntoUnmanaged()._handle);
    }

    public Task<NestedB> NestedaCreateOtherAsync()
    {
        return Task.FromResult(new NestedB(Value + 3));
    }

    public Task<ResultNestedBError> NestedaCreateOtherResultAsync()
    {
        return Task.FromResult(ResultNestedBError.Ok(new NestedB(Value + 4).IntoUnmanaged()._handle));
    }
}

public partial class NestedB : INestedB<NestedB>
{
    public int Value { get; }

    public NestedB() { Value = 0; }
    public NestedB(int value) { Value = value; }

    public void NestedbAccept(NestedA a)
    {
        _ = a.Value;
    }

    public Task NestedbAcceptAsync(NestedA a)
    {
        _ = a.Value;
        return Task.CompletedTask;
    }

    public void NestedbAcceptRef(NestedA a)
    {
        _ = a.Value;
    }

    public Task NestedbAcceptAsyncRef(NestedA a)
    {
        _ = a.Value;
        return Task.CompletedTask;
    }
}
