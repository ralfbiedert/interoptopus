using BenchmarkDotNet.Attributes;
using BenchmarkDotNet.Running;
using System.Threading;
using BenchmarkDotNet.Toolchains;
using ForCSharp;
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
[RPlotExporter]
[MemoryDiagnoser]
//[NativeMemoryProfiler]
public class Benchy
{
    const int SMALL = 0;
    const int MEDIUM = 1000;
    const int LARGE = 1000000;

    private Protobuf.Input smallProtobufInput = populateProtobufInput(SMALL);
    private Protobuf.Input mediumProtobufInput = populateProtobufInput(MEDIUM);
    private Protobuf.Input largeProtobufInput = populateProtobufInput(LARGE);

    private static Protobuf.Input populateProtobufInput(int n)
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
        for (int i = 1; i <= n; i++)
        {
            input.Context.Headers.Add($"Header-{i}", $"Value-{i}");
        }

        return input;
    }

    private Input smallFfiInput = populateFfiInput(SMALL);
    private Input mediumFfiInput = populateFfiInput(MEDIUM);
    private Input largeFfiInput = populateFfiInput(LARGE);

    private static Input populateFfiInput(int n)
    {
        var input = new Input();

        input.configuration = new Configuration();
        input.value = new Table();
        input.value.metadata = new TableMetadata();
        input.context = new Context();
        // input.configuration.host (String) = from "" to "verylonghostname" (4096 chars)
        input.configuration.host = Utf8String.From("127.0.0.1");
        input.configuration.response_size = (ulong)n;
        // input.configuration.is_ok_response = true if populating Items in Outputs, false if populating Errors
        input.configuration.is_ok_response = true;
        input.value.metadata.guid = Utf8String.From(new Guid().ToString());
        input.value.metadata.prefix = Utf8String.From("ordinary_prefix_");
        input.value.metadata.row_count = 5;
        input.value.metadata.column_count = 7;
        input.value.byte_array = VecU8.From(new byte[n]); // from 0 bytes to 1Mb
        // input.context.things = from 0 strings to 1,000,000 strings "thingX"
        var things = new Utf8String[n];
        for (int i = 1; i <= n; i++)
        {
            things[i] = Utf8String.From($"Thing-{i}");
        }
        input.context.things = VecUtf8String.From(things);
        // NB: FFI does not support HashMaps interop
        // input.context.headers = from 0 headers to 1,000,000 "key"=>"value" pairs
        //for (int i = 1; i <= n; i++)
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

    [Benchmark]
    public void Protobuf_0_cold()
    {
        var outputs = InteropProtobuf.ExecuteRustClient(smallProtobufInput);
    }

    [Benchmark]
    public void Protobuf_0_hot()
    {
        var outputs = InteropProtobuf.ExecuteRustClient(populateProtobufInput(SMALL));
    }

    [Benchmark]
    public void Protobuf_1k_cold()
    {
        var outputs = InteropProtobuf.ExecuteRustClient(mediumProtobufInput);
    }

    [Benchmark]
    public void Protobuf_1k_hot()
    {
        var outputs = InteropProtobuf.ExecuteRustClient(populateProtobufInput(MEDIUM));
    }

    [Benchmark]
    public void Protobuf_1kk_cold()
    {
        var outputs = InteropProtobuf.ExecuteRustClient(largeProtobufInput);
    }

    [Benchmark]
    public void Protobuf_1kk_hot()
    {
        var outputs = InteropProtobuf.ExecuteRustClient(populateProtobufInput(LARGE));
    }

    [Benchmark]
    public void Ffi_0_cold()
    {
        var outputs = InteropFfi.ExecuteRustClient(smallFfiInput);
    }

    [Benchmark]
    public void Ffi_0_hot()
    {
        var outputs = InteropFfi.ExecuteRustClient(populateFfiInput(SMALL));
    }

    [Benchmark]
    public void Ffi_1k_cold()
    {
        var outputs = InteropFfi.ExecuteRustClient(mediumFfiInput);
    }

    [Benchmark]
    public void Ffi_1k_hot()
    {
        var outputs = InteropFfi.ExecuteRustClient(populateFfiInput(MEDIUM));
    }

    [Benchmark]
    public void Ffi_1kk_cold()
    {
        var outputs = InteropFfi.ExecuteRustClient(largeFfiInput);
    }

    [Benchmark]
    public void Ffi_1kk_hot()
    {
        var outputs = InteropFfi.ExecuteRustClient(populateFfiInput(LARGE));
    }

    // [Benchmark]
    // public void WireInterop_0()
    // {
    //     var outputs = InteropWire.ExecuteRustClient(populateInput(SMALL));
    // }
}

public class Program
{
    public static void Main(string[] args)
    {
        var summary = BenchmarkRunner.Run<Benchy>();
    }
}
