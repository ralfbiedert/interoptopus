using My.Company;
using Xunit;

namespace interop_test
{
    public class GeneralTests
    {
        [Fact]
        public void CanCall()
        {
            Assert.True(InteropClass.do_math(10) == 11);
            
            InteropClass.xxx_entry_points(out var pointers);
            Assert.True(pointers.do_math(10) == 11);
        }
    }
}