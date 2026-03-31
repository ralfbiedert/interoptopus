using My.Company;
using My.Company.Common;
using Xunit;
using Interop = My.Company.Interop;

public class TestBehavior
{
    [Fact]
    public void behavior_panics()
    {
        // We must only run this on Windows as SEH is only supported there. On Linux this would abort the process
        if (!System.Runtime.InteropServices.RuntimeInformation.IsOSPlatform(System.Runtime.InteropServices.OSPlatform.Windows))
            return;

        try
        {
            Interop.behavior_panics();
        } catch (System.Runtime.InteropServices.SEHException)
        {
            return;
        }
        Assert.True(false);
    }

    [Fact]
    public void behavior_panics_via_result()
    {
        var v = Interop.behavior_panics_via_result();
        Assert.Equal(ResultVoidError.Panic, v);
    }

    [Fact]
    public void behavior_sleep()
    {
        Interop.behavior_sleep(10);
    }
}