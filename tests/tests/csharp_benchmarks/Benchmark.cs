using System;
using Interoptopus;
using My.Company;

static class Benchmark {

    const int Iterations = 100_000;

    static void Main(string[] args)
    {
        Interop.primitive_i8(123);
        Console.WriteLine("Running benchmarks ...");

        MeasureResult result;
        var writer = new MarkdownTableWriter();

        long x = 0;
        var short_vec = new Vec3f32[10];
        var short_byte = new byte[10];
        var long_vec = new Vec3f32[100_000];
        var tupled = new Tupled();

        MeasureResult.Calibrate(Iterations, () => {});

        result = MeasureResult.Measure(Iterations, () => Interop.primitive_void());
        writer.Add("primitive_void()", result);

        result = MeasureResult.Measure(Iterations, () => Interop.primitive_u8(0));
        writer.Add("primitive_u8(0)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.primitive_u16(0));
        writer.Add("primitive_u16(0)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.primitive_u32(0));
        writer.Add("primitive_u32(0)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.primitive_u64(0));
        writer.Add("primitive_u64(0)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.many_args_5(0, 0, 0, 0, 0));
        writer.Add("many_args_5(0, 0, 0, 0, 0)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.many_args_10(0, 0, 0, 0, 0, 0, 0, 0, 0, 0));
        writer.Add("many_args_10(0, 0, 0, 0, 0, 0, 0, 0, 0, 0)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.ptr(ref x));
        writer.Add("ptr(x)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.ptr_mut(out x));
        writer.Add("ptr_mut(x)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.ref_simple(ref x));
        writer.Add("ref_simple(x)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.ref_option(ref x));
        writer.Add("ref_option(x)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.call_tupled(new Tupled()));
        writer.Add("tupled(new Tupled())", result);

        result = MeasureResult.Measure(Iterations, () => Interop.complex_args_1(new Vec3f32(), ref tupled));
        writer.Add("complex_args_1(new Vec3f32(), ref e)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.callback(x0 => x0, 0));
        writer.Add("callback(x => x, 0)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_ffi_option_1(new OptionInner()));
        writer.Add("pattern_ffi_option_1(new OptionInner())", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_ffi_slice_delegate(x => x[0]));
        writer.Add("pattern_ffi_slice_delegate(x => x[0])", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_ffi_slice_delegate(x => x.Copied[0]));
        writer.Add("pattern_ffi_slice_delegate(x => x.Copied[0])", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_ffi_slice_delegate_huge(x => x[0]));
        writer.Add("pattern_ffi_slice_delegate_huge(x => x[0])", result);

        result = MeasureResult.Measure(1000, () => Interop.pattern_ffi_slice_delegate_huge(x => x.Copied[0]));
        writer.Add("pattern_ffi_slice_delegate_huge(x => x.Copied[0])", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_ffi_slice_2(short_vec, 0));
        writer.Add("pattern_ffi_slice_2(short_vec, 0)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_ffi_slice_2(long_vec, 0));
        writer.Add("pattern_ffi_slice_2(long_vec, 0)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_ffi_slice_4(short_byte, short_byte));
        writer.Add("pattern_ffi_slice_4(short_byte, short_byte)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_ascii_pointer_1("hello world"));
        writer.Add("pattern_ascii_pointer_1('hello world')", result);

        writer.Write("RESULTS.md");
    }
}
