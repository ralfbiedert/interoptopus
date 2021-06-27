using System.Linq;
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
        public void pattern_api_entry()
        {
            var api = Interop.my_api_init_v1();
            var input = new Tupled {x0 = 10};
            var output = api.tupled(input);
            
            Assert.Equal(input.x0, output.x0);
            
        }

    }
}