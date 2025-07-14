public partial class WireOf{{type}}
{
    private fixed buf; // A pinned interop buffer...

    public void Ser({{type}})
    {}

    public {{type}} De()
    {}
  // need to support Ser and De functions for this type...
  // needs a reference to buffer slice in these functions or in the class itself?
  // e.g.
  // let rest = &buf;
  // let rest = self.t.ser(rest)?;
  // let rest = self.u.ser(rest)?;
  // etc
}

public static class WireOf{{type}}Extensions
{
    public static WireOf{{type}} Wire(this {{type}} t) { return WireOf{{type}}.From(t); }
}

// how do we wire Primitives? NATIVELY!
// is Wire<u32> a thing? Most probably not, useless. Wire wraps a struct.
