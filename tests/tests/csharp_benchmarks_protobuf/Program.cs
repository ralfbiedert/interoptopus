using BenchmarkDotNet.Attributes;
using BenchmarkDotNet.Running;
using System.Threading;

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
public class Benchy
{
    const int ITERATIONS = 100_000; // TODO: run this many iters

    const int SMALL = 0;
    const int MEDIUM = 1000;
    const int LARGE = 1000000;

    Input populateInput(int n)
    {
        var input = new Input();
        return input;
    }

    [Benchmark]
    public void ProtobufInterop_0()
    {
        var outputs = InteropProtobuf.ExecuteRustClient(populateInput(SMALL));
    }

    [Benchmark]
    public void ProtobufInterop_1()
    {
        var outputs = InteropProtobuf.ExecuteRustClient(populateInput(MEDIUM));
    }

    [Benchmark]
    public void ProtobufInterop_2()
    {
        var outputs = InteropProtobuf.ExecuteRustClient(populateInput(LARGE));
    }

    [Benchmark]
    public void FfiInterop_0()
    {
        var outputs = InteropFfi.ExecuteRustClient(populateInput(SMALL));
    }

    [Benchmark]
    public void FfiInterop_1()
    {
        var outputs = InteropFfi.ExecuteRustClient(populateInput(MEDIUM));
    }

    [Benchmark]
    public void FfiInterop_2()
    {
        var outputs = InteropFfi.ExecuteRustClient(populateInput(LARGE));
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
