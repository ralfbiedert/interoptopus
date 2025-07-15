public partial class WireOf{{type}}
{
    private fixed buf; // A pinned interop buffer...

    public void Ser({{type}})
    {}

    public {{type}} De()
    {}

    public usize WireSize()
    {
        // Return the same value as storage_size() on rs side
        256
    }
{#
  // need to support Ser and De functions for this type...
  // needs a reference to buffer slice in these functions or in the class itself?
  // e.g.
  // let rest = &buf;
  // let rest = self.t.ser(rest)?;
  // let rest = self.u.ser(rest)?;
  // etc
#}
}

public static class WireOf{{type}}Extensions
{
    public static WireOf{{type}} Wire(this {{type}} t) { return WireOf{{type}}.From(t); }
}
{#
// how do we wire Primitives? NATIVELY!
// is Wire<u32> a thing? Most probably not, useless. Wire wraps a struct.


// wired function wrapper:

// [LibraryImport(NativeLib, EntryPoint = "start_server")]
// [MethodImpl(MethodImplOptions.AggressiveOptimization)]
// private static partial WireOfReturn start_server(WireOfSomething server_name);

// [MethodImpl(MethodImplOptions.AggressiveOptimization)]
// public static Return StartServer(Something s) {
//     Span<byte> bytes = stackalloc byte[s.wire_size()];
//     fixed (byte* p = bytes)
//     {
//         WireOfSomething ws = s.Wire(p);
//         var wr_buf = start_server(ws); // returns a WireOfReturn constructed on the rs side?
//         // might have to convert wr_buf to Managed
//         return WireOfReturn.Unwire(wr_buf);
//     }
// }
#}
