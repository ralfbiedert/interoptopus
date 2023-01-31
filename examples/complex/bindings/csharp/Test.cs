using System;
using My.Company;
using Xunit;

namespace interop_test
{
    public class GeneralTests
    {
        [Fact]
        public void SimpleCallWorks()
        {
            Assert.Equal(InteropClass.example_api_version(), (uint) 0x00_01_00_00);
        }

        [Fact]
        public void CreateDestroyEngineWorks()
        {
            FFIError rval;
            IntPtr context_ptr = IntPtr.Zero;

            rval = InteropClass.example_create_context(ref context_ptr);
            Assert.Equal(rval, FFIError.Ok);

            rval = InteropClass.example_destroy_context(ref context_ptr);
            Assert.Equal(rval, FFIError.Ok);
        }


        [Fact]
        public void RunGameEngine()
        {
            var super1 = new SuperComplexEntity()
            {
                player_1 = new Vec3
                {
                    x = 2,
                    y = 4,
                    z = 6,
                },

                player_2 = new Vec3
                {
                    x = 2,
                    y = 4,
                    z = 6,
                },

                ammo = 10,
            };
            uint score = 0;
            var super2 = new SuperComplexEntity();

            IntPtr context_ptr = IntPtr.Zero;
            InteropClass.example_create_context(ref context_ptr);
            InteropClass.example_update_score_by_callback(context_ptr, x => 100);
            InteropClass.example_update_score_by_callback(context_ptr, x => x * 2);
            InteropClass.example_print_score(context_ptr);
            InteropClass.example_return_score(context_ptr, ref score);
            InteropClass.example_double_super_complex_entity(context_ptr, ref super1, ref super2);
            InteropClass.example_destroy_context(ref context_ptr);

            Assert.Equal(score, (uint) 200);
            Assert.Equal(super2.ammo, 2* super1.ammo);
        }
    }
}