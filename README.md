[![Latest Version]][crates.io]
[![docs]][docs.rs]
![MIT]


## Interoptopus 

Create FFI bindings to your favorite language. Composable. Escape hatches included. 



## Overview

- you wrote an `extern "C"` API in Rust  
- the types at the FFI boundary are (mostly) owned by yourself
- you prefer to keep all your binding-related information (e.g., documentation) in Rust code 

Known limitations
- not used in production yet
- somewhat verbose if you don't own most of your types (still possible, just more work)
- if you target only a single language and don't care about your FFI layer other solutions might be better

## Supported Languages 

| Language | Crate | Comment |
| --- | --- | --- | 
| C# | `interoptopus_backend_csharp` |  Built-in. |
| C | `interoptopus_backend_c` | Built-in. |
| Python CFFI | `interoptopus_backend_cpython_cffi` | Built-in. |
| Your language | Write your own backend! | See existing backends for what to do.* |

(*) Ok, right now I don't really recommend writing a new backend just yet as lots of internals might change. That said, it should only take a few hours and feedback is more than welcome.  



## Example 

Slightly abridged, see `examples/hello_world` for full code:

```rust
use interoptopus::{ffi_function, ffi_type, IndentWriter};
use interoptopus_backend_csharp::Interop;

#[ffi_type]
#[repr(C)]
pub struct Vec2f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// A function which does something with the vector.
#[ffi_function]
#[no_mangle]
pub extern "C" fn my_game_function(input: Option<&Vec2f32>) -> Vec2f32 {
    Vec2f32 { x: 2.0, y: 4.0, z: 6.0 }
}

// This ultimately defines our FFI exports, all functions have to be listed here.
interoptopus::inventory_function!(ffi_inventory, [], [my_game_function]);

#[test]
fn generate_csharp_bindings() {
    let library = ffi_inventory();

    let config = interoptopus_backend_csharp::Config {
        namespace: "My.Company".to_string(),
        class: "InteropClass".to_string(),
        dll_name: "hello_world".to_string(),
        ..interoptopus_backend_csharp::Config::default()
    };

    let generator = interoptopus_backend_csharp::Generator::new(config, library);
    
    generator.write_to(my_file)?;
}
```

With a Cargo.toml:

```toml
[dependencies]
interoptopus = { version = "0.1", features = ["derive"] }
interoptopus_backend_csharp = "0.1"
```


Will produce:

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
        public static extern Vec2f32 my_game_function(ref Vec2f32 input);

    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct Vec2f32
    {
        public float x;
        public float y;
        public float z;
    }

}
```

For other languages (Python, C, ...) see the `examples` folder.


## Current Status

- June 13, 2021 - Pre-alpha. Has generated C#, C, Python-CFFI bindings at least once, many things missing, untested.



## FAQ

- **Why do I get `error[E0658]: macro attributes in `#[derive]` output are unstable`?**
  
  This happens when `#[ffi_type]` appears after `#derive[...]`. Just switch their order.


- **How do I support a new language?**

  1) create a new crate, like `my_language`
  1) check which existing backend comes closest, copy that code  
  1) start from trait `Interop::write_to` producing some output, fix errors as they appear 
  1) create a UI test against `interoptopus_reference_project` to ensure your bindings are stable  


## Contributing

PRs are welcome. If it's a major change consider filing an issue first. If you can, try running `scripts/test_all.py` (might need to install `dotnet`, `python`, ...). 


## License

MIT

[Latest Version]: https://img.shields.io/crates/v/interoptopus.svg
[crates.io]: https://crates.io/crates/interoptopus
[MIT]: https://img.shields.io/badge/license-MIT-blue.svg
[docs]: https://docs.rs/interoptopus/badge.svg
[docs.rs]: https://docs.rs/interoptopus/
