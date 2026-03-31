using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Runtime.InteropServices;
using System.Threading;
using System.Threading.Tasks;
using Interoptopus;
using My.Company;
using My.Company.Common;
using Interop = My.Company.Interop;

static class Benchmark {

    const int Iterations = 100_000;

    static void Main(string[] args)
    {
        Console.WriteLine("Running benchmarks ...");

        MeasureResult result;
        var writer = new MarkdownTableWriter();

        var short_vec = SliceVec3f32.From(new Vec3f32[10]);
        var short_byte = SliceByte.From(new byte[10]);
        var short_byte_mut = SliceMutByte.From(new byte[10]);
        var long_vec = SliceVec3f32.From(new Vec3f32[100_000]);
        var tupled = new Tupled();
        var callback_huge_prealloc = new CallbackHugeVecSlice(x => x[0]);
        var serviceAsync = ServiceAsyncBasic.Create();
        var hello_world = "hello world".Utf8();
        var deeply_nested = new DeeplyNestedWire1 {
            name = new string('x', 50),
            values = Enumerable.Range(0, 3).ToDictionary(
                i => (uint)i,
                i => new DeeplyNestedWire2 {
                    values = Enumerable.Range(0, 3).Select(j => new DeeplyNestedWire3 {
                        x = Enumerable.Range(0, 3).ToDictionary(k => (uint)k, k => new DeeplyNestedWire4 { a = (uint)k }),
                        y = new string('y', 50)
                    }).ToList()
                }
            )
        };

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

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_ffi_option_1(OptionInner.None));
        writer.Add("pattern_ffi_option_1(OptionInner.None)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_ffi_slice_delegate(x => x[0]));
        writer.Add("pattern_ffi_slice_delegate(x => x[0])", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_ffi_slice_delegate_huge(x => x[0]));
        writer.Add("pattern_ffi_slice_delegate_huge(x => x[0])", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_ffi_slice_delegate_huge(callback_huge_prealloc));
        writer.Add("pattern_ffi_slice_delegate_huge(callback_huge_prealloc)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_ffi_slice_2(short_vec, 0));
        writer.Add("pattern_ffi_slice_2(short_vec, 0)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_ffi_slice_2(long_vec, 0));
        writer.Add("pattern_ffi_slice_2(long_vec, 0)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_ffi_slice_4(short_byte, short_byte_mut));
        writer.Add("pattern_ffi_slice_4(short_byte, short_byte)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_ascii_pointer_1("hello world"));
        writer.Add("pattern_ascii_pointer_1('hello world')", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_string_10("hello world".Utf8()));
        writer.Add("pattern_string_10('hello world'.Utf8())", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_string_10(hello_world.Clone()));
        writer.Add("pattern_string_10(hello_world.Clone())", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_string_11(ref hello_world));
        writer.Add("pattern_string_11(ref hello_world)", result);

        result = MeasureResult.Measure(Iterations, () => "hello world".Utf8().Dispose());
        writer.Add("'hello world'.Utf8().Dispose()", result);

        result = MeasureResult.Measure(Iterations, () => hello_world.Clone().Dispose());
        writer.Add("hello_world.Clone().Dispose()", result);

        result = MeasureResult.Measure(Iterations, () => VecByte.Empty().Dispose());
        writer.Add("VecU8.Empty().Dispose()", result);

        result = MeasureResult.Measure(Iterations, () => VecUtf8String.Empty().Dispose());
        writer.Add("VecUtf8String.Empty().Dispose()", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_vec_1().Dispose());
        writer.Add("pattern_vec_u8_return().Dispose()", result);

        result = MeasureResult.MeasureParallel(Iterations, 1, async () => { var tcs = new TaskCompletionSource<int>(); tcs.SetResult(0); await tcs.Task; });
        writer.Add("await new TaskCompletionSource<int>().Task", result);

        result = MeasureResult.MeasureParallel(Iterations, 1, async () => { await serviceAsync.Call(); });
        writer.Add("await serviceAsync.Success() [1 parallel]", result);

        result = MeasureResult.MeasureParallel(Iterations, 16, async () => { await serviceAsync.Call(); });
        writer.Add("await serviceAsync.Success() [16 parallel]", result);

        result = MeasureNativeCallback(Iterations, cb => Interop.service_async_basic_call(serviceAsync.Context, cb));
        writer.Add("serviceAsync.Call() [native callback only]", result);

        result = MeasureResult.Measure(Iterations, () => Interop.wire_accept_string_1(WireOfString.From("hello world")));
        writer.Add("wire_accept_string_1()", result);

        result = MeasureResult.Measure(Iterations, () => Interop.wire_deeply_nested_1(deeply_nested.Wire()));
        writer.Add("wire_deeply_nested_1(deeply_nested.Wire())", result);

        writer.Write("RESULTS.md");
    }

    // Measures time from when the native call is issued until the completion callback fires —
    // pure native round-trip latency with no C# async scheduler overhead.
    static MeasureResult MeasureNativeCallback(uint n, Func<AsyncCallbackCommonNative, TaskHandle> nativeInvoke)
    {
        var mre = new ManualResetEventSlim(false);
        long callbackTick = 0;
        AsyncCallbackCommon cb = (_, __) => { callbackTick = Stopwatch.GetTimestamp(); mre.Set(); };
        var cbNative = new AsyncCallbackCommonNative
        {
            _ptr = Marshal.GetFunctionPointerForDelegate(cb),
            _ts = IntPtr.Zero,
        };

        // Warmup
        for (var i = 0; i < n; i++)
        {
            mre.Reset();
            var th = nativeInvoke(cbNative);
            mre.Wait();
            th.Dispose();
        }

        long sumTicks = 0;
        for (var i = 0; i < n; i++)
        {
            mre.Reset();
            var t0 = Stopwatch.GetTimestamp();
            var th = nativeInvoke(cbNative);
            mre.Wait();
            sumTicks += callbackTick - t0;
            th.Dispose();
        }

        return new MeasureResult(n, sumTicks);
    }
}
