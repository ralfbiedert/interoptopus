using System.Diagnostics;
using System.Runtime.InteropServices;
using Google.Protobuf;

namespace ForCSharp;

public class InteropProtobuf
{
    // [DllImport("proto_benchy.dll")]
    [DllImport("libproto_benchy.dylib")]
    private static extern unsafe void ProtoRustClient(
        byte[] structPointer, uint structLength, void** result, uint* resultLength);

    // [DllImport("proto_benchy.dll")]
    [DllImport("libproto_benchy.dylib")]
    private static extern unsafe void FreeRustResultMemory(byte* rustPtr, uint len);

    public static Outputs ExecuteRustClient(Input input)
    {
        try
        {
            unsafe
            {
                var inputBytes = input.ToByteString().ToByteArray();

                byte* buffer = null;
                uint length = 0;

                ProtoRustClient(inputBytes, (uint)inputBytes.Length, (void**)&buffer, &length);

                var result = CopyAndDeallocate(buffer, length);
                var output = Outputs.Parser.ParseFrom(result);

                return output;
            }
        }
        catch (Exception e)
        {
            throw new InvalidOperationException($"Unexpected error in ExecuteRustClient: {e.Message}", e);
        }
    }

    private static unsafe byte[] CopyAndDeallocate(byte* contentPtr, uint contentLength)
    {
        byte[] result = [];
        if (contentPtr == null) return result;

        try
        {
            result = new byte[contentLength];
            Marshal.Copy((IntPtr)contentPtr, result, 0, (int)contentLength);
        }
        finally
        {
            FreeRustResultMemory(contentPtr, contentLength);
        }

        return result;
    }
}
