// TODO: How to deal with nested helper types?
//
// public partial struct EnumPayload
// {
//      [StructLayout(LayoutKind.Sequential)]
//      internal unsafe struct UnmanagedB { ... }
//
//      [StructLayout(LayoutKind.Sequential)]
//      internal unsafe struct UnmanagedC { ... }
//
//      [StructLayout(LayoutKind.Explicit)]
//      public unsafe struct Unmanaged { ... }
// }
//
// Q:
// - Are `Unmanaged*` here individual structs in our taxonomy?
// - Would they be linked nodes?
// - Should they be omitted entirely since they are an impl detail?
//
// A?
// - It appears if they are always "derived" from something (like an `Unmanaged` is always derived
//   from the actual type) they should not be listed anywhere, since it's genuinely an implementation
//   detail. But then again we might be hardcoding knowledge of whether an `Unmanaged` exists for
//   something into our code.
// - Instead, types should probably have an `ImplementationDetail` enum or fields or so, where it's
//   indicated for each type it its intended to be generated, and with that enum definitively
//   declaring what other parts of the code can expect to exist.

pub enum TypeKind {
    // Primitive(Primitive),
    // Struct(Struct),
    // Class(Class),
    // Enum(Enum),
}
