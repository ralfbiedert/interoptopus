using System;
using System.Linq;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternServicesMultipleCtors
{

    [Fact]
    public void service_ctors()
    {
        ServiceMultipleCtors.NewWith(123);
        ServiceMultipleCtors.NewWithout();
        ServiceMultipleCtors.NewWithString("hello world");

        try
        {
            ServiceMultipleCtors.NewFailing(123);
            Assert.True(false);
        }
        catch (InteropException<FFIError>) { }
    }


}
