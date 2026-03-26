using My.Company;

namespace My.Company;

partial class ServiceA: IServiceA<ServiceA>
{
    public static ServiceA Create()
    {
        return new();
    }

    public uint Call(uint x)
    {
        return x + 1;
    }
}