using System.Collections.Generic;
using My.Company;
using Xunit;
using DeeplyNestedWire2 = My.Company.DeeplyNestedWire2;
using DeeplyNestedWire3 = My.Company.DeeplyNestedWire3;
using DeeplyNestedWire4 = My.Company.DeeplyNestedWire4;
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
                {
                    1, new DeeplyNestedWire2
                    {
                        values =
                        [
                            new DeeplyNestedWire3()
                            {
                                x = new Dictionary<uint, DeeplyNestedWire4>
                                {
                                    { 2, new DeeplyNestedWire4 { a = 42 } }
                                },
                                y = "World"
                            }
                        ]
                    }
                }
            }
        }.Wire();
        var rval  = Interop.wire_deeply_nested_1(nested);

        Assert.Equal(42u, rval);
    }

}