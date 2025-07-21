using System;
using Gen.ForCSharp;

namespace ForCSharp;

public class InteropFfi
{
    /// Main benched function.
    public static Outputs ExecuteRustClient(Input input)
    {
        try
        {
            // var outputs = Interop.FfiRustClient(input);
            // return outputs;
            return null;
        }
        catch (Exception e)
        {
            throw new InvalidOperationException($"Unexpected FFI error in ExecuteRustClient: {e.Message}", e);
        }
    }
}
