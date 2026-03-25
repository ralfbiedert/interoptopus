using My.Company;

namespace My.Company;

partial class ServiceA: IServiceA<ServiceA>
{
    public static ServiceA ServiceaCreate()
    {
        return new();
    }

    public uint ServiceaCall(uint x)
    {
        return x + 1;
    }
}