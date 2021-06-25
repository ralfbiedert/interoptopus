Generates C bindings for [Interoptopus](https://github.com/ralfbiedert/interoptopus).

### Usage

In your library or a support project add this:

```rust
use my_crate::ffi_inventory;

#[test]
fn generate_c_bindings() {
    use interoptopus::Interop;
    use interoptopus_backend_c::{Generator, CWriter, Config};

    // Converts an `ffi_inventory()` into Python interop definitions.
    Generator::new(Config::default(), ffi_inventory()).write_to("module.h")
}
```

And we might produce something like this:

```c

#ifndef module
#define module

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>

typedef struct Vec3f32
{
    float x;
    float y;
    float z;
} Vec3f32;

Vec3f32 my_game_function(Vec3f32* input);

#ifdef __cplusplus
}
#endif

#endif /* module */

```
