// using System.Runtime.InteropServices;
//
// namespace ForCSharp;
//
// public class InteropWire
// {
//     private static extern unsafe void WireRustClient(
//         byte[] structPointer, uint structLength, void** result, uint* resultLength);
//
//     private static extern unsafe void FreeRustResultMemory(byte* rustPtr, uint len);
//
//     /// Main benched function.
//     public static Outputs ExecuteRustClient(Input input)
//     {
//         try
//         {
//             unsafe
//             {
//                 var inputBytes = input.ToWire().ToByteArray();
//
//                 byte* buffer = null;
//                 uint length = 0;
//
//                 WireRustClient(inputBytes, (uint)inputBytes.Length, (void**)&buffer, &length);
//
//                 var result = CopyAndDeallocate(buffer, length);
//                 var output = Outputs.Parser.ParseFrom(result);
//
//                 return output;
//             }
//         }
//         catch (Exception e)
//         {
//             throw new InvalidOperationException($"Unexpected error in ExecuteRustClient: {e.Message}", e);
//         }
//     }
//
//     private static unsafe byte[] CopyAndDeallocate(byte* contentPtr, uint contentLength)
//     {
//         byte[] result = [];
//         if (contentPtr == null) return result;
//
//         try
//         {
//             result = new byte[contentLength];
//             Marshal.Copy((IntPtr)contentPtr, result, 0, (int)contentLength);
//         }
//         finally
//         {
//             FreeRustResultMemory(contentPtr, contentLength);
//         }
//
//         return result;
//     }
// }