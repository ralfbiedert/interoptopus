using System.Runtime.InteropServices;

namespace ForCSharp;

public class InteropFfi
{
    const string DllName = "proto_benchy.dll";

    [DllImport(DllName)]
    private static extern unsafe Ffi.Outputs FfiRustClient(Ffi.Input input);

    /// Main benched function.
    public static Ffi.Outputs ExecuteRustClient(Ffi.Input input)
    {
        try
        {
            unsafe
            {
                var outputs = FfiRustClient(input);
                return outputs;
            }
        }
        catch (Exception e)
        {
            throw new InvalidOperationException($"Unexpected FFI error in ExecuteRustClient: {e.Message}", e);
        }
    }
}
