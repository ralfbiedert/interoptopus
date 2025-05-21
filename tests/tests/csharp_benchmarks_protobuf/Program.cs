using BenchmarkDotNet.Attributes;
using BenchmarkDotNet.Running;
using System.Threading;

namespace ForCSharp;

[RPlotExporter]
public class Benchy
{
    [Benchmark]
    public void ProtobufInterop()
    {
        var input = new Input();
        // TODO: generate input based on benchmark parameters
        // (at least two extremes: very small payload, very large payload and maybe something in-between)
        var outputs = InteropProtobuf.ExecuteRustClient(input);
    }

    // [Benchmark]
    // public void WireInterop()
    // {
    //     // TODO: generate input based on benchmark parameters
    //     // (at least two extremes: very small payload, very large payload and maybe something in-between)
    //     var outputs = InteropWire.ExecuteRustClient(input);
    // }
}

public class Program
{
    public static void Main(string[] args)
    {
        var summary = BenchmarkRunner.Run<Benchy>();
    }
}
