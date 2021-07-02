using System.Linq;
using System.Runtime.InteropServices;
using My.Company;
using Xunit;

namespace interop_test
{
    public class GeneralTests
    {
        [Fact]
        public void pattern_ffi_slice_delegate()
        {
            Interop.pattern_ffi_slice_delegate(delegate (FFISliceu8 x0)
            {
                Assert.Equal(x0.Count, 10);
                Assert.Equal(x0[0], 0);
                Assert.Equal(x0[5], 5);

                // Test IEnumerable using LINQ 
                var arr = x0.ToArray();
                Assert.Equal(arr.Length, 10);
                Assert.Equal(arr[0], 0);
                Assert.Equal(arr[5], 5);

                return x0[0];
            });
        }
        
        [Fact]
        public void pattern_ffi_slice_3()
        {
            var data = new byte[100_000];
            var handle = GCHandle.Alloc(data, GCHandleType.Pinned);
            var slice = new FFISliceMutu8(handle, 100_000);
            
            Interop.pattern_ffi_slice_3(slice, x0 =>
            {
                x0[1] = 100;
            });
            
            handle.Free();
            
            Assert.Equal(data[0], 1);
            Assert.Equal(data[1], 100);
        }


        [Fact]
        public void pattern_ffi_option_nullable()
        {
            var t = new Inner();
            FFIOptionInner someOpt = FFIOptionInner.FromNullable(t);
            Inner? nullableOpt = someOpt.ToNullable();
            Assert.True(nullableOpt.HasValue);

            FFIOptionInner someOpt2 = FFIOptionInner.FromNullable(null);
            Inner? nullableOpt2 = someOpt2.ToNullable();
            Assert.False(nullableOpt2.HasValue);
        }

        [Fact]
        public void pattern_api_entry()
        {
            // TODO: Why does this not work?
            Interop.my_api_init_v1(out var api);
            
            var input = new Tupled {x0 = 10};
            var output = api.tupled(input);
            
            Assert.Equal(2 * input.x0, output.x0);
            
        }

    }
}