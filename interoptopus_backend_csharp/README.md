Generates C# bindings for [Interoptopus](https://github.com/ralfbiedert/interoptopus).


### Usage

In your library or a support project add this:

```rust
use my_crate::ffi_inventory;

#[test]
fn generate_csharp_bindings() {
    use interoptopus::Interop;
    use interoptopus_backend_csharp::{Generator, CSharpWriter, Config};

    // Converts an `ffi_inventory()` into C# interop definitions.
    Generator::new(Config::default(), ffi_inventory()).write_to("Interop.cs")
}
```

And we might produce something like this:

```cs
using System;
using System.Runtime.InteropServices;

namespace My.Company
{
    public static class InteropClass
    {
        public const string NativeLib = "hello_world";

        /// A function which does something with the vector.
        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "my_game_function")]
        public static extern Vec3f32 my_game_function(ref Vec3f32 input);
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct Vec3f32
    {
        public float x;
        public float y;
        public float z;
    }
}
```
