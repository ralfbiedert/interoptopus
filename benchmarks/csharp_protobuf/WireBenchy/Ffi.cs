using System;
using Gen.Ffi;

namespace Benchy;

public class InteropFfi
{
    /// Main benched function.
    public static FOutputs ExecuteRustClient(FInput input)
    {
        try
        {
            var outputs = Interop.FfiRustClient(input);
            return outputs;
        }
        catch (Exception e)
        {
            throw new InvalidOperationException($"Unexpected FFI error in ExecuteRustClient: {e.Message}", e);
        }
    }
}
