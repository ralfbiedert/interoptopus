using My.Company;
using My.Company.Common;
using Xunit;

namespace interop_test
{
    public class GeneralTests
    {
        [Fact]
        public void pattern_ffi_slice_delegate()
        {
            Interop.pattern_ffi_slice_delegate(delegate (Sliceu8 x0)
            {
                var span = x0.ReadOnlySpan;

                return span[0];
            });
        }


    }
}