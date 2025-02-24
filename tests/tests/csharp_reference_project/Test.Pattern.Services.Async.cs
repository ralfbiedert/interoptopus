using System;
using System.Linq;
using System.Runtime.InteropServices;
using System.Runtime.InteropServices.Marshalling;
using System.Threading.Tasks;
using My.Company;
using My.Company.Common;
using Xunit;

    // Debug - write_type_definition_named_callback 
    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate void AsyncHelperNative(IntPtr data, IntPtr callback_data);
    public delegate void AsyncHelperDelegate(IntPtr data);

    [NativeMarshalling(typeof(MarshallerMeta))]
    public struct AsyncHelper : IDisposable
    {
        private AsyncHelperDelegate _callbackUser;
        private IntPtr _callbackNative;

        public AsyncHelper() { }

        public AsyncHelper(AsyncHelperDelegate callbackUser)
        {
            _callbackUser = callbackUser;
            _callbackNative = Marshal.GetFunctionPointerForDelegate(new AsyncHelperNative(Call));
        }

        public void Call(IntPtr data, IntPtr callback_data)
        {
            _callbackUser(data);
        }

        public void Dispose()
        {
            if (_callbackNative == IntPtr.Zero) return;
            Marshal.FreeHGlobal(_callbackNative);
            _callbackNative = IntPtr.Zero;
        }


        [CustomMarshaller(typeof(AsyncHelper), MarshalMode.Default, typeof(Marshaller))]
        private struct MarshallerMeta {  }

        [StructLayout(LayoutKind.Sequential)]
        public struct Unmanaged
        {
            internal IntPtr Callback;
            internal IntPtr Data;
        }

        public ref struct Marshaller
        {
            private AsyncHelper managed;
            private Unmanaged native;
            private Unmanaged sourceNative;
            private GCHandle? pinned;

            public void FromManaged(AsyncHelper managed)
            {
                this.managed = managed;
            }

            public Unmanaged ToUnmanaged()
            {
                return new Unmanaged
                {
                    Callback = managed._callbackNative,
                    Data = IntPtr.Zero
                };
            }

            public void FromUnmanaged(Unmanaged unmanaged)
            {
                sourceNative = unmanaged;
            }

            public AsyncHelper ToManaged()
            {
                return new AsyncHelper
                {
                    _callbackNative = sourceNative.Callback,
                };
            }

            public void Free() { }
        }
    }



public static partial class XXX 
{
    
    public static unsafe Task<byte> xxx()
    {
        var completionSource = new TaskCompletionSource<byte>();
        // async_fn(x =>
        // {
        //     completionSource.SetResult(x);
        //     return 0;
        // });
        return completionSource.Task;
    }

}

public class TestPatternServicesAsync
{
    [Fact]
    public async void service_async()
    {
        var _ = await XXX.xxx();
    }
}
