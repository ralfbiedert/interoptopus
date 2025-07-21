using System;
using Gen.ForCSharp;

namespace ForCSharp;

public class InteropWire
{
    /// Main benched function.
    public static unsafe Outputs ExecuteRustClient(Input input)
    {
        int bufferSize = input.WireSize();
        Span<byte> buffer = stackalloc byte[bufferSize]; // Stack allocation for small data, use heap for large payloads

        fixed (byte* bufferPtr = buffer)
        {
            var wireInput = input.WireWithBuffer(bufferPtr, bufferSize);
            var wireOutputs = Interop.WireRustClient(wireInput); // WireOfOutputs
            try
            {
                return wireOutputs.Unwire();
            }
            catch (Exception e)
            {
                throw new InvalidOperationException($"Unexpected Wire error in ExecuteRustClient: {e.Message}", e);
            }
            finally
            {
                wireOutputs.Dispose();
            }
        }
    }
}
