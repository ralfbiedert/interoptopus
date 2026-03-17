using System.Collections.Generic;
using My.Company;
using My.Company.Common;
using Xunit;
using DeeplyNestedWire2 = My.Company.DeeplyNestedWire2;
using Interop = My.Company.Interop;

public class TestWireNested
{
    [Fact]
    public void wire_deeply_nested()
    {
        var nested = new DeeplyNestedWire1
        {
            name = "Hello",
            values = new Dictionary<uint, DeeplyNestedWire2>()
            {

            }
        }.Wire();
        var rval  = Interop.wire_deeply_nested_1(nested);
    }

}