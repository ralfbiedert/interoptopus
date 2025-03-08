using System;
using System.Linq;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestPatternServicesMultipleCtors
{

    [Fact]
    public void MultipleCtors()
    {
        using (ServiceMultipleCtors.NewWith(123)) { }
        using (ServiceMultipleCtors.NewWithout()) { };
        using (ServiceMultipleCtors.NewWithString("hello world")) { };
    }

    [Fact]
    public void NewFailing()
    {
        try
        {
            var serviceMultipleCtors = ServiceMultipleCtors.NewFailing(123);
            Assert.True(false);
        }
        catch (InteropException<FFIError>) { }
    }

}
