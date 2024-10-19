using System;
using System.Linq;
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
#if (NETSTANDARD2_1_OR_GREATER || NET5_0_OR_GREATER || NETCOREAPP2_1_OR_GREATER)
            Interop.pattern_ffi_slice_delegate(delegate(SliceU8 x0)
            {
                var span = x0.ReadOnlySpan;

                return span[0];
            });
#endif
        }

        // Ensure that the Copied property has the correct length and contents
        [Fact]
        public void ensure_unsafe_copy_length()
        {
            const int size = 20;
            var service = SimpleService.NewWith(20);
            var res = service.ReturnSliceMut();

            foreach (var r in res.Copied)
            {
                Assert.True(r == size);
            }
        }

        // Ensure that the Copied property has the correct length and contents
        [Fact]
        public void namespaced_inner_slice_readonly_span_works()
        {
            var vecs = new Vec[10];
            var span = new ReadOnlySpan<Vec>(vecs);

            vecs[0].x = 2.0;

            var rval = Interop.namespaced_inner_slice(vecs);

            Assert.True(rval[0].x > 1.0);
        }
    }
}