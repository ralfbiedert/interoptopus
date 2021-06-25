Generates CPython CFFI bindings for [Interoptopus](https://github.com/ralfbiedert/interoptopus).

### Usage

In your library or a support project add this:

```rust
use my_crate::ffi_inventory;

#[test]
fn generate_python_bindings() {
    use interoptopus::Interop;
    use interoptopus_backend_cpython_cffi::{Generator, PythonWriter, Config};

    // Converts an `ffi_inventory()` into Python interop definitions.
    Generator::new(Config::default(), ffi_inventory()).write_to("module.py")
}
```

And we might produce something like this:

```python
from cffi import FFI

api_definition = """
typedef struct Vec3f32
    {
    float x;
    float y;
    float z;
    } Vec2f32;

Vec3f32 my_game_function(Vec3f32* input);
"""


ffi = FFI()
ffi.cdef(api_definition)
_api = None


def init_api(dll):
    """Initializes this library, call with path to DLL."""
    global _api
    _api = ffi.dlopen(dll)


class raw:
    """Raw access to all exported functions."""

    def my_game_function(input):
    global _api
    return _api.my_game_function(input)
```
