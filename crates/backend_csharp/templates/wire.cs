public partial class WireOf{{type}}
{
    public void Ser()
    {}

    public WireOf{{type}} De()
    {}
  // need to support Ser and De functions for this type...
  // needs a reference to buffer slice in these functions or in the class itself?
  // e.g.
  // let rest = &buf;
  // let rest = self.t.ser(rest)?;
  // let rest = self.u.ser(rest)?;
  // etc
}

// how do we wire Primitives?
// is Wire<u32> a thing? Most probably not, useless. Wire wraps a struct.
