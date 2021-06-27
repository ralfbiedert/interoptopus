using System;
using My.Company;

namespace Interoptopus
{

    static class Benchmark {
        
        const int Iterations = 100_000;
        
        
        static void Main(string[] args)
        {
            Interop.primitive_i8(123);
            Console.WriteLine("Hello World!");

            MeasureResult result;
            var writer = new MarkdownTableWriter();
            
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
            
            result = MeasureResult.Measure(Iterations, () => Interop.pattern_ascii_pointer_1("hello world"));
            writer.Add("pattern_ascii_pointer_1('hello world')", result);
            
            writer.Write("BENCHMARK_RESULTS.md");
        }
    }
}
