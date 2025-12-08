using System;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;
using My.Company;
using My.Company.Common;
using Xunit;
using Array = My.Company.Array;


public class TestPatternServicesAsyncLoad
{
    private const int N = 1;

    [Fact]
    public async void Load()
    {
        try
        {
            var s = ServiceAsyncLoad.New();
            var cts = new CancellationTokenSource(TimeSpan.FromSeconds(1));
            var tasks = new Task[N];

            for (int i = 0; i < N; i++)
            {
                tasks[i] = Task.Run(async () =>
                {
                    var random = new Random();
                    while (!cts.Token.IsCancellationRequested)
                    {
                        var input = (uint) random.Next();
                        try
                        {
                            var result = await s.Load( input);
                            Assert.Equal(input, result);
                        }
                        catch (Exception e)
                        {
                            // throw;
                        }
                    }
                }, cts.Token);
            }

            try
            {
                await Task.WhenAll(tasks);
            }
            catch (OperationCanceledException)
            {
                // Expected when timeout occurs
            }

        }
        catch (Exception e)
        {
        }
    }

}
