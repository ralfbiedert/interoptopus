using System;
using System.Linq;
using System.Threading.Tasks;
using My.Company;
using My.Company.Common;
using Xunit;
using Array = My.Company.Array;


public class TestPatternServicesAsyncSleep
{
     [Fact]
     public async void ReturnAfterMs()
     {
         var s = ServiceAsyncSleep.New();
         var r = await s.ReturnAfterMs(123, 500);
         s.Dispose();
         Assert.Equal(r, 123u);
     }
}
