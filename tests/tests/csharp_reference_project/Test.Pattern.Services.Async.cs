using System;
using System.Linq;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using My.Company;
using My.Company.Common;
using Xunit;

// public static partial class XXX {
//
//     [LibraryImport("foo", EntryPoint = "xxx")]
//     public static partial FFIError async_fn(CallbackU8 callback);
//
//     public static unsafe Task<byte> xxx()
//     {
//         var completionSource = new TaskCompletionSource<byte>();
//         async_fn(x =>
//         {
//             completionSource.SetResult(x);
//             return 0;
//         });
//         return completionSource.Task;
//     }
//
//
// }
//
// public class TestPatternServicesAsync
// {
//     [Fact]
//     public async void service_async()
//     {
//         var _ = await XXX.xxx();
//     }
// }
