using System.Diagnostics;
using System.Runtime.InteropServices;
using Google.Protobuf;

namespace One.Two.ForCSharp;

// Pure protobuf-based interop between Rust and C#.
public class Interop
{
    [DllImport("rust_bench.dll")]
    private static extern int DummyFunction(int numOne, int numTwo);

    [DllImport("rust_bench.dll")]
    private static unsafe extern void RustClient(byte[] structPointer, uint structLength, void** result, uint* resultLength);

    [DllImport("rust_bench.dll")]
    private static unsafe extern void FreeRustResultMemory(byte* rustPtr, uint len);

    /// Test basic interop.
    public static int ExecuteDummyFunction(int numOne, int numTwo)
    {
        return DummyFunction(numOne, numTwo);
    }

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
                RustClient(byteArray, (uint)byteArray.Length, (void**)&resultPtr, &resultBufLength);

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
