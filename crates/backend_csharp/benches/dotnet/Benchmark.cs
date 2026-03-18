using System;
using System.Collections.Generic;
using System.IO;
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

    [LibraryImport("reference_project", EntryPoint = "protobuf_vec_string")]
    private static unsafe partial void protobuf_vec_string(byte* data, nuint len);

    [LibraryImport("reference_project", EntryPoint = "protobuf_hashmap_string_string")]
    private static unsafe partial void protobuf_hashmap_string_string(byte* data, nuint len);

    const int Iterations = 1000;

    static (long wireNs, long protoPreallocNs) RunVecString(int n, int s)
    {
        var vec = Enumerable.Range(0, n).Select(i => new string((char)('a' + i % 26), s)).ToList();

        var proto = new ProtobufBench.VecString();
        foreach (var v in vec) proto.Values.Add(v);

        var wireResult = MeasureResult.Measure(Iterations, () => Interop.wire_accept_string_3(WireOfVecString.From(vec)));

        var protoResult = MeasureResult.Measure(Iterations, () => {
            var bytes = proto.ToByteArray();
            unsafe {
                fixed (byte* ptr = bytes) {
                    protobuf_vec_string(ptr, (nuint)bytes.Length);
                }
            }
        });

        return ((long)wireResult.MicroPer1000(), (long)protoResult.MicroPer1000());
    }

    static (long wireNs, long protoPreallocNs) RunHashMapStringString(int n, int s)
    {
        var map = Enumerable.Range(0, n).ToDictionary(
            i => $"key_{i:D6}" + new string('x', Math.Max(0, s - 10)),
            i => new string((char)('a' + i % 26), s));

        var proto = new ProtobufBench.HashMapStringString();
        foreach (var kv in map) proto.Values.Add(kv.Key, kv.Value);

        var wireResult = MeasureResult.Measure(Iterations, () => Interop.wire_accept_string_4(WireOfHashMapStringString.From(map)));

        var protoResult = MeasureResult.Measure(Iterations, () => {
            var bytes = proto.ToByteArray();
            unsafe {
                fixed (byte* ptr = bytes) {
                    protobuf_hashmap_string_string(ptr, (nuint)bytes.Length);
                }
            }
        });

        return ((long)wireResult.MicroPer1000(), (long)protoResult.MicroPer1000());
    }

    static (long wireNs, long protoAllocNs, long protoPreallocNs) RunWireVsProtobuf(int n, int s)
    {
        var deeply_nested = new My.Company.DeeplyNestedWire1 {
            name = new string('x', s),
            values = Enumerable.Range(0, n).ToDictionary(
                i => (uint)i,
                i => new My.Company.DeeplyNestedWire2 {
                    values = Enumerable.Range(0, n).Select(j => new My.Company.DeeplyNestedWire3 {
                        x = Enumerable.Range(0, n).ToDictionary(k => (uint)k, k => new My.Company.DeeplyNestedWire4 { a = (uint)k }),
                        y = new string('y', s)
                    }).ToList()
                }
            )
        };

        var proto_nested = new ProtobufBench.DeeplyNestedWire1 { Name = new string('x', s) };
        foreach (var i in Enumerable.Range(0, n)) {
            var wire2 = new ProtobufBench.DeeplyNestedWire2();
            foreach (var j in Enumerable.Range(0, n)) {
                var wire3 = new ProtobufBench.DeeplyNestedWire3 { Y = new string('y', s) };
                foreach (var k in Enumerable.Range(0, n)) {
                    wire3.X.Add((uint)k, new ProtobufBench.DeeplyNestedWire4 { A = (uint)k });
                }
                wire2.Values.Add(wire3);
            }
            proto_nested.Values.Add((uint)i, wire2);
        }

        var proto_buffer = new byte[proto_nested.CalculateSize()];

        var wireResult = MeasureResult.Measure(Iterations, () => Interop.wire_deeply_nested_1(deeply_nested.Wire()));

        var protoAllocResult = MeasureResult.Measure(Iterations, () => {
            var bytes = proto_nested.ToByteArray();
            unsafe {
                fixed (byte* ptr = bytes) {
                    protobuf_deeply_nested_1(ptr, (nuint)bytes.Length);
                }
            }
        });

        var protoPreallocResult = MeasureResult.Measure(Iterations, () => {
            var span = new Span<byte>(proto_buffer);
            proto_nested.WriteTo(span);
            unsafe {
                fixed (byte* ptr = proto_buffer) {
                    protobuf_deeply_nested_1(ptr, (nuint)proto_buffer.Length);
                }
            }
        });

        return (
            (long)wireResult.MicroPer1000(),
            (long)protoAllocResult.MicroPer1000(),
            (long)protoPreallocResult.MicroPer1000()
        );
    }

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

        var vec_strings = Enumerable.Range(0, 50).Select(i => new string((char)('a' + i % 26), 50)).ToList();
        var map_strings = Enumerable.Range(0, 50).ToDictionary(i => $"key_{i:D3}" + new string('x', 44), i => new string((char)('a' + i % 26), 50));

        result = MeasureResult.Measure(Iterations, () => Interop.wire_accept_string_3(WireOfVecString.From(vec_strings)));
        writer.Add("wire_accept_string_3(Vec<String> x50)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.wire_accept_string_4(WireOfHashMapStringString.From(map_strings)));
        writer.Add("wire_accept_string_4(HashMap<String,String> x50)", result);

        writer.Write("RESULTS.md");

        // Wire vs Protobuf parametric comparison
        int[] ns = { 1, 3, 5, 10, 50, 100, 300 };
        int[] ss = { 1, 100, 1000, 10000 };

        Console.WriteLine("\n--- Vec<String> ---");
        using (var csv = File.CreateText("wire_vs_protobuf_vec_string.csv")) {
            csv.WriteLine("N,S,wire_ns,proto_ns");
            foreach (var n in ns) {
                foreach (var s in ss) {
                    Console.Write($"  N={n}, S={s} ... ");
                    var (wireNs, protoNs) = RunVecString(n, s);
                    Console.WriteLine($"wire={wireNs}, proto={protoNs}");
                    csv.WriteLine($"{n},{s},{wireNs},{protoNs}");
                    csv.Flush();
                }
            }
        }

        Console.WriteLine("\n--- HashMap<String,String> ---");
        using (var csv = File.CreateText("wire_vs_protobuf_hashmap_string.csv")) {
            csv.WriteLine("N,S,wire_ns,proto_ns");
            foreach (var n in ns) {
                foreach (var s in ss) {
                    Console.Write($"  N={n}, S={s} ... ");
                    var (wireNs, protoNs) = RunHashMapStringString(n, s);
                    Console.WriteLine($"wire={wireNs}, proto={protoNs}");
                    csv.WriteLine($"{n},{s},{wireNs},{protoNs}");
                    csv.Flush();
                }
            }
        }

        Console.WriteLine("\nResults written to wire_vs_protobuf_vec_string.csv and wire_vs_protobuf_hashmap_string.csv");
    }
}
