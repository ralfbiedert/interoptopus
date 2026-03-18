using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Google.Protobuf;
using Interoptopus;
using My.Company;
using My.Company.Common;
using Interop = My.Company.Interop;

static partial class Benchmark {

    [LibraryImport("reference_project", EntryPoint = "protobuf_deeply_nested_1")]
    private static unsafe partial uint protobuf_deeply_nested_1(byte* data, nuint len);

    const int Iterations = 100_000;
    const int N = 5;
    const int S = 10000;

    static void Main(string[] args)
    {
        Console.WriteLine("Running benchmarks ...");

        MeasureResult result;
        var writer = new MarkdownTableWriter();

        long x = 0;
        var short_vec = SliceVec3f32.From(new Vec3f32[10]);
        var short_byte = SliceByte.From(new byte[10]);
        var short_byte_mut = SliceMutByte.From(new byte[10]);
        var long_vec = SliceVec3f32.From(new Vec3f32[100_000]);
        var tupled = new Tupled();
        var callback_huge_prealloc = new CallbackHugeVecSlice(x => x[0]);
        var serviceAsync = ServiceAsyncBasic.Create();
        var hello_world = "hello world".Utf8();
        var deeply_nested = new My.Company.DeeplyNestedWire1 {
            name = new string('x', S),
            values = Enumerable.Range(0, N).ToDictionary(
                i => (uint)i,
                i => new My.Company.DeeplyNestedWire2 {
                    values = Enumerable.Range(0, N).Select(j => new My.Company.DeeplyNestedWire3 {
                        x = Enumerable.Range(0, N).ToDictionary(k => (uint)k, k => new My.Company.DeeplyNestedWire4 { a = (uint)k }),
                        y = new string('y', S)
                    }).ToList()
                }
            )
        };

        var proto_nested = new ProtobufBench.DeeplyNestedWire1 { Name = new string('x', S) };
        foreach (var i in Enumerable.Range(0, N)) {
            var wire2 = new ProtobufBench.DeeplyNestedWire2();
            foreach (var j in Enumerable.Range(0, N)) {
                var wire3 = new ProtobufBench.DeeplyNestedWire3 { Y = new string('y', S) };
                foreach (var k in Enumerable.Range(0, N)) {
                    wire3.X.Add((uint)k, new ProtobufBench.DeeplyNestedWire4 { A = (uint)k });
                }
                wire2.Values.Add(wire3);
            }
            proto_nested.Values.Add((uint)i, wire2);
        }

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

        result = MeasureResult.Measure(Iterations, async () => { await new TaskCompletionSource().Task; });
        writer.Add("await new TaskCompletionSource().Task", result);

        result = MeasureResult.Measure(Iterations, async () => { await serviceAsync.Call(); });
        writer.Add("await serviceAsync.Success()", result);

        result = MeasureResult.Measure(Iterations, () => Interop.wire_accept_string_1(WireOfString.From("hello world")));
        writer.Add("wire_accept_string_1()", result);

        result = MeasureResult.Measure(Iterations, () => Interop.wire_deeply_nested_1(deeply_nested.Wire()));
        writer.Add("wire_deeply_nested_1(deeply_nested.Wire())", result);

        // Pre-allocate buffer for protobuf serialization (avoid alloc per iteration)
        var proto_buffer = new byte[proto_nested.CalculateSize()];

        result = MeasureResult.Measure(Iterations, () => {
            var bytes = proto_nested.ToByteArray();
            unsafe {
                fixed (byte* ptr = bytes) {
                    protobuf_deeply_nested_1(ptr, (nuint)bytes.Length);
                }
            }
        });
        writer.Add("protobuf_deeply_nested_1(ToByteArray())", result);

        result = MeasureResult.Measure(Iterations, () => {
            var span = new Span<byte>(proto_buffer);
            proto_nested.WriteTo(span);
            unsafe {
                fixed (byte* ptr = proto_buffer) {
                    protobuf_deeply_nested_1(ptr, (nuint)proto_buffer.Length);
                }
            }
        });
        writer.Add("protobuf_deeply_nested_1(prealloc)", result);

        writer.Write("RESULTS.md");
    }
}
