using System;
using System.Linq;
using System.Threading.Tasks;
using System.Text;
using My.Company;
using Xunit;

public class TestPatternServiceAsync
{
    public async static Task Check()
    {

        var s = ServiceAsync.New();
        var x = RandomString();

        var tasks = Enumerable.Range(0, 10).Select(async _ =>
        {
            var y = await s.HandleString(x);
            Assert.Equal(y, x);
        }).ToList();

        await Task.WhenAll(tasks);
        s.Dispose();
    }

    private static String RandomString()
    {
        var randomString = new StringBuilder(100 * 1024);
        var random = new Random();
        for (int i = 0; i < 100 * 1024; i++)
        {
            randomString.Append((char)random.Next('a', 'z' + 1));
        }
        return randomString.ToString();
    }
}