using System;
using System.Linq;
using My.Company;
using My.Company.Common;
using Xunit;
using Array = My.Company.Array;

namespace interop_test
{
    public class GeneralTests
    {

        [Fact]
        public void array_1()
        {
            var array = new Array {data0 = 5};
            Assert.Equal(5, Interop.array_1(array));
        }

        [Fact]
        public void pattern_ffi_slice_delegate()
        {
            Interop.pattern_ffi_slice_delegate(delegate (Sliceu8 x0)
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

            Interop.pattern_ffi_slice_3(data, x0 =>
            {
                x0[1] = 100;
            });

            Assert.Equal(data[0], 1);
            Assert.Equal(data[1], 100);
        }


        [Fact]
        public void boolean_alignment()
        {
            const ulong BIT_PATTERN = 0x5555555555555555;

            for (var i = 0; i < 16; i++)
            {
                var x = new BooleanAlignment { is_valid = true, id = BIT_PATTERN, datum = BIT_PATTERN };

                x = Interop.boolean_alignment(x);
                Assert.Equal(x.is_valid, false);
                Assert.Equal(x.id, BIT_PATTERN);
                Assert.Equal(x.datum, BIT_PATTERN);

                x = Interop.boolean_alignment(x);
                Assert.Equal(x.is_valid, true);
                Assert.Equal(x.id, BIT_PATTERN);
                Assert.Equal(x.datum, BIT_PATTERN);

                x = Interop.boolean_alignment2(true);
                Assert.Equal(x.is_valid, true);

                x = Interop.boolean_alignment2(false);
                Assert.Equal(x.is_valid, false);
            }
        }

        [Fact]
        public void packed()
        {
            var p1 = new Packed1
            {
                x = 12,
                y = 34
            };

            var p2 = Interop.packed_to_packed1(p1);

            Assert.Equal(p1.x, p2.x);
            Assert.Equal(p1.y, p2.y);
        }

        [Fact]
        public void pattern_ffi_option_nullable()
        {
            var t = new Inner();
            OptionInner someOpt = OptionInner.FromNullable(t);
            Inner? nullableOpt = someOpt.ToNullable();
            Assert.True(nullableOpt.HasValue);

            OptionInner someOpt2 = OptionInner.FromNullable(null);
            Inner? nullableOpt2 = someOpt2.ToNullable();
            Assert.False(nullableOpt2.HasValue);
        }

        [Fact]
        public void pattern_ffi_callback_exception()
        {
            // This value will be changed to `6` before the callbacks are invoked, and
            // changed to `8` after callbacks return.
            var rval = 0;


            FFIError C1(int x, int y)
            {
                // This should see `6` here, and it does, because the function has set that value.
                Assert.Equal(rval, 6);

                // However, when we now throw we would expect .NET to unwind that exception back
                // into Rust (and therefore eventually observe rval to be `8`). When we use this
                // safe wrapper it does that, but only because we implemented special handling for
                // it for callbacks that return a FFIError.
                throw new Exception("We handled this");
            }

            void C2(int x, int y)
            {
                // This callback looks very similar. However, it does not come with special handling
                // support. When this throws, the .NET runtime will NOT(!!!) unwind back through
                // Rust but instead just return the stack up without ever calling Rust again. That means
                // in particular that any code that you would expect to run in Rust subsequent to the invocation
                // of this callback (esp. `drop` code and friends) will NOT fire, leading to unexpected
                // memory loss or worse.

                // throw new Exception("Unchecked callback which we didn't handle. Comment this out and see the test fail.");
            };

            try
            {
                Interop.pattern_callback_7_checked(C1, C2, 3, 7, out rval);
            }
            catch (Exception e)
            {
                // If everything works Rust code after invoking C1 and C2 is still executed,
                // setting this variable to `8`.
                Assert.Equal(rval, 8);
            }

        }

        [Fact]
        public void pattern_service_generated()
        {
            var simpleService = SimpleService.NewWith(123);
            var b = new byte[] { 1, 2, 3 } ;

            simpleService.MethodMutSelfFfiError(b);
            var s1 = simpleService.ReturnString();
            var s2 = simpleService.ReturnString();

            var sliceMut = simpleService.ReturnSliceMut();
            sliceMut[0] = 44;

            var slice = simpleService.ReturnSlice();
            Assert.Equal(slice.Count, 123);
            Assert.Equal((int) slice[0], 44);
            Assert.Equal((int) slice[1], 123);

            uint value = 123;
            var lt = SimpleServiceLifetime.NewWith(ref value);
            var s3 = lt.ReturnStringAcceptSlice(System.Array.Empty<byte>());
            var s4 = lt.ReturnStringAcceptSlice(System.Array.Empty<byte>());
        }

    }
}