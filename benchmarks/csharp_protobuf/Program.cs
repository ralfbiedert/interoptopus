using BenchmarkDotNet.Attributes;
using BenchmarkDotNet.Running;
using System;
using Gen.ForCSharp;

namespace ForCSharp;

// Benchmark setup:
// This benchmark measures interop speed of a real-life-like structures
// through the C#->Rust->C# ipc path.
// TODO: Test Rust->C#->Rust path too?
//
// The actual processing only consists of validating some predefined values
// and filling in buffers with data of certain size - there are three
// settings, minimal, normal and extreme
// - in minimal, all the buffers are 0-sized,
// - in normal buffers are in 1000's of elements, and
// - in extreme buffers are huge - close to 1,000,000 elements each.
//
// Input is prepared on the C# side with
// input.configuration.host (String) = from "" to "verylonghostname" (4096 chars)
// input.configuration.response_size = 0, 1000 or 1000000 (size of the response)
// input.configuration.is_ok_response = true if populating Items in Outputs, false if populating Errors
// input.table.metadata.guid = typical guid
// input.table.metadata.prefix = "ordinary_prefix_"
// input.table.bytearray = from 0 bytes to 1Mb
// input.table.metadata.{row,column}_count = some predef values like 5 and 7
// input.context.things = from 0 strings to 1,000,000 strings "thingX"
// input.context.headers = from 0 headers to 1,000,000 "key"=>"value" pairs
//
// Output is prepared on Rust side in response to the input:
// validate input.table.metadata.{row,column}_count to have correct values
// read input.configuration.response_size
// read input.configuration.is_ok_response
// outputs.response.results = from 0 to 1,000,000 "Result" key value pairs
// outputs.data.items.items = from 0 to 1,000,000 "Message" key value pairs
// outputs.data.errors.error_messages = from 0 to 1,000,000 string values
//
public class BenchyBase
{
    protected const int SMALL = 0;
    protected const int MEDIUM = 1000;
    protected const int LARGE = 1000000;

    protected static Protobuf.Input populateProtobufInput(int n)
    {
        var input = new Protobuf.Input();
        input.Configuration = new Protobuf.Configuration();
        input.Value = new Protobuf.Table();
        input.Value.Metadata = new Protobuf.TableMetadata();
        input.Context = new Protobuf.Context();

        // input.configuration.host (String) = from "" to "verylonghostname" (4096 chars)
        input.Configuration.Host = "127.0.0.1";
        input.Configuration.ResponseSize = n;
        // input.configuration.is_ok_response = true if populating Items in Outputs, false if populating Errors
        input.Configuration.IsOkResponse = true;
        input.Value.Metadata.Guid = new Guid().ToString();
        input.Value.Metadata.Prefix = "ordinary_prefix_";
        input.Value.Metadata.RowCount = 5;
        input.Value.Metadata.ColumnCount = 7;
        input.Value.ByteArray = Google.Protobuf.ByteString.CopyFrom(new byte[n]); // from 0 bytes to 1Mb
        // input.context.things = from 0 strings to 1,000,000 strings "thingX"
        for (int i = 1; i <= n; i++)
        {
            input.Context.Things.Add($"Thing-{i}");
        }
        // input.context.headers = from 0 headers to 1,000,000 "key"=>"value" pairs
        /*for (int i = 1; i <= n; i++)
        {
            input.Context.Headers.Add($"Header-{i}", $"Value-{i}");
        }*/

        return input;
    }

    protected static Input populateFfiInput(int n)
    {
        var input = new Input();

        input.configuration = new Configuration();
        input.value = new Table();
        input.value.metadata = new TableMetadata();
        input.context = new Context();

        // input.configuration.host (String) = from "" to "verylonghostname" (4096 chars)
        input.configuration.host = "127.0.0.1".Utf8();
        input.configuration.response_size = (ulong)n;
        // input.configuration.is_ok_response = true if populating Items in Outputs, false if populating Errors
        input.configuration.is_ok_response = true;
        input.value.metadata.guid = new Guid().ToString().Utf8();
        input.value.metadata.prefix = "ordinary_prefix_".Utf8();
        input.value.metadata.row_count = 5;
        input.value.metadata.column_count = 7;
        fixed (var x = new byte[n]) // try with memory pinned from the start
        {
            input.value.byte_array = SliceU8.From(x, x.Length); // from 0 bytes to 1Mb}
        }
        // input.context.things = from 0 strings to 1,000,000 strings "thingX"
        var things = new Utf8String[n];
        for (int i = 0; i < n; i++)
        {
            things[i] = $"Thing-{i}".Utf8();
        }

        input.context.things = things.Slice();
        // NB: FFI does not support HashMaps interop
        // input.context.headers = from 0 headers to 1,000,000 "key"=>"value" pairs
        //for (int i = 0; i < n; i++)
        //{
        //    input.context.headers.Add($"Header-{i}", $"Value-{i}");
        //}

        return input;
    }

    // Wire.Input populateWireInput(int n)
    // {
    //     var input = new WireInput();
    //     return input;
    // }
}

public class JustTest : BenchyBase
{
    public void Ffi_0_hot()
    {
        var outputs = InteropFfi.ExecuteRustClient(populateFfiInput(SMALL));
    }
    public void Ffi_10_hot()
    {
        var outputs = InteropFfi.ExecuteRustClient(populateFfiInput(10));
    }
}

//[RPlotExporter]
[MemoryDiagnoser]
//[NativeMemoryProfiler]
[MinIterationCount(10)]
[MaxIterationCount(15)]
public class HotBenchy : BenchyBase
{
    [Benchmark]
    public void Protobuf_0_hot()
    {
        var outputs = InteropProtobuf.ExecuteRustClient(populateProtobufInput(SMALL));
    }

    [Benchmark]
    public void Protobuf_10_hot()
    {
        var outputs = InteropProtobuf.ExecuteRustClient(populateProtobufInput(10));
    }

    [Benchmark]
    public void Protobuf_50_hot()
    {
        var outputs = InteropProtobuf.ExecuteRustClient(populateProtobufInput(50));
    }

    [Benchmark]
    public void Protobuf_100_hot()
    {
        var outputs = InteropProtobuf.ExecuteRustClient(populateProtobufInput(100));
    }

    [Benchmark]
    public void Protobuf_500_hot()
    {
        var outputs = InteropProtobuf.ExecuteRustClient(populateProtobufInput(500));
    }

    [Benchmark]
    public void Protobuf_1k_hot()
    {
        var outputs = InteropProtobuf.ExecuteRustClient(populateProtobufInput(MEDIUM));
    }

    /*    [Benchmark]
        public void Protobuf_200k_hot()
        {
            var outputs = InteropProtobuf.ExecuteRustClient(populateProtobufInput(200000));
        }

        [Benchmark]
        public void Protobuf_1kk_hot()
        {
            var outputs = InteropProtobuf.ExecuteRustClient(populateProtobufInput(LARGE));
        }*/

    [Benchmark]
    public void Ffi_0_hot()
    {
        var outputs = InteropFfi.ExecuteRustClient(populateFfiInput(SMALL));
    }

    [Benchmark]
    public void Ffi_10_hot()
    {
        var outputs = InteropFfi.ExecuteRustClient(populateFfiInput(10));
    }

    [Benchmark]
    public void Ffi_50_hot()
    {
        var outputs = InteropFfi.ExecuteRustClient(populateFfiInput(50));
    }

    [Benchmark]
    public void Ffi_100_hot()
    {
        var outputs = InteropFfi.ExecuteRustClient(populateFfiInput(100));
    }

    [Benchmark]
    public void Ffi_500_hot()
    {
        var outputs = InteropFfi.ExecuteRustClient(populateFfiInput(500));
    }

    [Benchmark]
    public void Ffi_1k_hot()
    {
        var outputs = InteropFfi.ExecuteRustClient(populateFfiInput(MEDIUM));
    }
    /*
        [Benchmark]
        public void Ffi_200k_hot()
        {
            var outputs = InteropFfi.ExecuteRustClient(populateFfiInput(200000));
        }

        [Benchmark]
        public void Ffi_1kk_hot()
        {
            var outputs = InteropFfi.ExecuteRustClient(populateFfiInput(LARGE));
        }*/

    // [Benchmark]
    // public void WireInterop_0()
    // {
    //     var outputs = InteropWire.ExecuteRustClient(populateInput(SMALL));
    // }
}

[RPlotExporter]
[MemoryDiagnoser]
//[NativeMemoryProfiler]
public class ColdBenchy : BenchyBase
{
    private Protobuf.Input smallProtobufInput = populateProtobufInput(SMALL);
    private Protobuf.Input mediumProtobufInput = populateProtobufInput(MEDIUM);
    private Protobuf.Input largeProtobufInput = populateProtobufInput(LARGE);

    [Benchmark]
    public void Protobuf_0_cold()
    {
        var outputs = InteropProtobuf.ExecuteRustClient(smallProtobufInput);
    }

    [Benchmark]
    public void Protobuf_1k_cold()
    {
        var outputs = InteropProtobuf.ExecuteRustClient(mediumProtobufInput);
    }

    [Benchmark]
    public void Protobuf_1kk_cold()
    {
        var outputs = InteropProtobuf.ExecuteRustClient(largeProtobufInput);
    }

    private Input smallFfiInput = populateFfiInput(SMALL);
    private Input mediumFfiInput = populateFfiInput(MEDIUM);
    private Input largeFfiInput = populateFfiInput(LARGE);

    [Benchmark]
    public void Ffi_0_cold()
    {
        var input = smallFfiInput;
        var outputs = InteropFfi.ExecuteRustClient(input);
    }

    [Benchmark]
    public void Ffi_1k_cold()
    {
        var outputs = InteropFfi.ExecuteRustClient(mediumFfiInput);
    }
    [Benchmark]
    public void Ffi_1kk_cold()
    {
        var outputs = InteropFfi.ExecuteRustClient(largeFfiInput);
    }
}

public class Program
{
    public static void Main(string[] args)
    {
        //var tt = new Utf8String[1];
        var hot = BenchmarkRunner.Run<HotBenchy>();
        //var cold = BenchmarkRunner.Run<ColdBenchy>();

        //var benchy = new JustTest();
        //benchy.Ffi_0_hot();
    }
}
