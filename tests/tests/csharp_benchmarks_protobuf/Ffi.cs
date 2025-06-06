using System;
using System.Runtime.InteropServices;
using Gen.ForCSharp;

namespace ForCSharp;

public class InteropFfi
{
    const string DllName = "proto_benchy";

    [DllImport(DllName)]
    private static extern unsafe Outputs FfiRustClient(Input input);

    /// Main benched function.
    public static Outputs ExecuteRustClient(Input input)
    {
        try
        {
            var outputs = FfiRustClient(input);
            return outputs;
        }
        catch (Exception e)
        {
            throw new InvalidOperationException($"Unexpected FFI error in ExecuteRustClient: {e.Message}", e);
        }
    }
}
