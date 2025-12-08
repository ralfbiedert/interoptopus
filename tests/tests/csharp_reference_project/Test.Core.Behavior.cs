using System;
using My.Company;
using My.Company.Common;
using Xunit;

public class TestBehavior
{
    [Fact]
    public void behavior_panics_via_result()
    {
        var v = Interop.behavior_panics_via_result();
        Assert.Equal(ResultError.Panic, v);
    }

    [Fact]
    public void behavior_sleep()
    {
        Interop.behavior_sleep(10);
    }

    [Fact]
    public void behavior_panics()
    {
        try
        {
             Interop.behavior_panics();

        }
        catch (Exception e)
        {
            Console.WriteLine(e);
        }
    }

}