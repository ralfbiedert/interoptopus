using System.Diagnostics;
using System.Runtime.InteropServices;
using Google.Protobuf;
using BenchmarkDotNet.Attributes;
using BenchmarkDotNet.Running;
using System.Threading;

namespace One.Two.ForCSharp;

// Pure protobuf-based interop between Rust and C#.
public class InteropProtobuf
{
    [DllImport("rust_bench.dll")]
    private static unsafe extern void ProtoRustClient(byte[] structPointer, uint structLength, void** result, uint* resultLength);

    [DllImport("rust_bench.dll")]
    private static unsafe extern void FreeRustResultMemory(byte* rustPtr, uint len);

    /// Main benched function.
    public static Outputs ExecuteRustClient(Input input)
    {
        try
        {
            unsafe
            {
                // Start the serialization timer
                Stopwatch ser = Stopwatch.StartNew();
                byte[] byteArray = input.ToByteString().ToByteArray();

                byte* resultPtr = null;
                uint resultBufLength = 0;
                ProtoRustClient(byteArray, (uint)byteArray.Length, (void**)&resultPtr, &resultBufLength);

                // Start the deserialization timer
                Stopwatch deser = Stopwatch.StartNew();

                byte[] result = CopyAndDeallocate(resultPtr, resultBufLength);

                Outputs output = Outputs.Parser.ParseFrom(result);

                // Stop the deserialization timer
                deser.Stop();

                var elapsed = deser.ElapsedMilliseconds;

                return output;
            }
        }
        catch (Exception e)
        {
            throw new InvalidOperationException($"Unexpected error in ExecuteRustClient: {e.Message}", e);
        }
    }

    /// <remarks>
    /// Since the memory is deallocated after copying, the contentPtr should not be accessed
    /// again after calling this method. Ensuring that the pointer
    /// is valid and that the given length correctly represents the allocated memory size when using this method.
    /// </remarks>
    /// <returns>The content at the pointer location. Null if the contentPtr is null.</returns>
    public static unsafe byte[] CopyAndDeallocate(byte* contentPtr, uint contentLength)
    {
        byte[] result = Array.Empty<byte>();

        if (contentPtr != null)
        {
            try
            {
                result = new byte[contentLength];
                Marshal.Copy((IntPtr)contentPtr, result, 0, (int)contentLength);
            }
            finally
            {
                //Free rust memory
                FreeRustResultMemory(contentPtr, contentLength);
            }
        }

        return result;
    }
}

// Interoptopus Wire<T>-based interop between Rust and C#.
public class InteropWire
{
    [DllImport("rust_bench.dll")]
    private static unsafe extern void WireRustClient(byte[] structPointer, uint structLength, void** result, uint* resultLength);

    [DllImport("rust_bench.dll")]
    private static unsafe extern void FreeRustResultMemory(byte* rustPtr, uint len);

    /// Main benched function.
    public static Outputs ExecuteRustClient(Input input)
    {
        try
        {
            unsafe
            {
                byte[] byteArray = input.ToWire().ToByteArray();

                byte* resultPtr = null;
                uint resultBufLength = 0;
                WireRustClient(byteArray, (uint)byteArray.Length, (void**)&resultPtr, &resultBufLength);

                byte[] result = CopyAndDeallocate(resultPtr, resultBufLength);

                Outputs output = Outputs.Parser.ParseFrom(result);

                return output;
            }
        }
        catch (Exception e)
        {
            throw new InvalidOperationException($"Unexpected error in ExecuteRustClient: {e.Message}", e);
        }
    }

    /// <remarks>
    /// Since the memory is deallocated after copying, the contentPtr should not be accessed
    /// again after calling this method. Ensuring that the pointer
    /// is valid and that the given length correctly represents the allocated memory size when using this method.
    /// </remarks>
    /// <returns>The content at the pointer location. Null if the contentPtr is null.</returns>
    public static unsafe byte[] CopyAndDeallocate(byte* contentPtr, uint contentLength)
    {
        byte[] result = Array.Empty<byte>();

        if (contentPtr != null)
        {
            try
            {
                result = new byte[contentLength];
                Marshal.Copy((IntPtr)contentPtr, result, 0, (int)contentLength);
            }
            finally
            {
                //Free rust memory
                FreeRustResultMemory(contentPtr, contentLength);
            }
        }

        return result;
    }
}

[RPlotExporter]
public class Benchy {
    [Benchmark]
    public void ProtobufInterop() {
        var outputs = InteropProtobuf.ExecuteRustClient(input);
    }

    [Benchmark]
    public void WireInterop() {
        var outputs = InteropWire.ExecuteRustClient(input);
    }
}

class Program
{
    static void Main(string[] args)
    {
        var summary = BenchmarkRunner.Run<Benchy>();
    }
}