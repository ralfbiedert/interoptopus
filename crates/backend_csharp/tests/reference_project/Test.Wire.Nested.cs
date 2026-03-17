using My.Company;
using My.Company.Common;
using Xunit;
using Interop = My.Company.Interop;

public class TestWireNested
{
    [Fact]
    public void wire_deeply_nested()
    {
        var nested = new DeeplyNestedWire1 { };
        var wire = WireOfDeeplyNestedWire1.From(nested);
        Interop.wire_deeply_nested(wire);
    }

}