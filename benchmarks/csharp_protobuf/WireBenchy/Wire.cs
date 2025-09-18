using System;
using Gen.Wire;
using Gen.Benchy;

namespace Benchy;

public class InteropWire
{
    /// Main benched function.
    public static unsafe WOutputs ExecuteRustClient(WInput input)
    {
        int bufferSize = input.WireSize();
        Span<byte> buffer = stackalloc byte[bufferSize]; // Stack allocation for small data, use heap for large payloads

        fixed (byte* bufferPtr = buffer)
        {
            var wireInput = input.WireWithBuffer(bufferPtr, bufferSize);
            var wireOutputs = Gen.Wire.Interop.WireRustClient(wireInput); // WireOfWOutputs
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
