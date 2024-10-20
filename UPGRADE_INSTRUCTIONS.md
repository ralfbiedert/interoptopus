# Upgrade Instructions

Tips for solving non-trivial breaking changes when upgrading from previous versions.

### 0.14 → 0.15

- Exported functions (`#[ffi_function]`) don't need to specify `#[no_mangle]` or `extern "C"` anymore as these will be
  added automatically.
- Exported types (`#[ffi_type]`) must not specify `#[repr(...)]` anymore as we will handle that. If you need custom
  attributes you can, for example, do `#[ffi_type(transparent)]` or `#[ffi_type(packed)]`.
- In service definitions, providing a `prefix` is generally not needed anymore.
- Service methods that return void (`()`) can now be used without a `#[ffi_service_method]` annotation. On a Rust panic
  they will silently return. If this is not acceptable you must return `Result` or specify a different panic behavior
  via that attribute.
- `#[ffi_type(patterns(ffi_error))]` is now `#[ffi_type(error)]`.
- If you used the C# `DotNet` or `Unity` overload writer, these helpers now take their own configuration
  where appropriate. If you previously only did `DotNet::new()` this became `DotNet::new_built()`.
- `AsciiPointer` is now called `CStrPointer`, since it can contain non-ASCII data (e.g., when called from C#).
- We fixed capitalization in some backends, e.g., a `Sliceu8` is now `SliceU8`.
- When using `InventoryBuilder` you should call `.validate().inventory()` now.
- To override visibility for all fields:
    - Previously you had to `#[ffi_type(visibility(_ = "public"))]`
    - Now you do `#[ffi_type(visibility(_all = "public"))]`
- Surrogates now work through the `Surrogate<T, L>` type.
    - Previously you needed to specify `#[ffi_surrogates(some_field = "some_foreign_type")]`
    - Instead, you now make `some_field` of type `Surrogate<Foreign, Local>`
- Setting alignment on types is not supported anymore (for now). You should also stop using alignment on
  earlier versions as various backends didn't translate that properly.
- Backend-related testing functions were moved into an internal `tests` project, as the code was mostly specific to our
  project needs anyway.

### 0.13 → 0.14

- Removed `inventory!` macro
    - You now just write a regular function returning an `Inventory`
    - See the reference project for details

### 0.12 → 0.13

- Deprecated Python CFFI backend, replace with Python CTypes backend.
    - Might require changing some invocations. Please see `reference_project.py`.
- Renamed attributes of `#[ffi_service_method]` once more, no behavior changed:
    - `wrap` is now `on_panic`
    - `direct` is `return_default`
    - `raw` is `undefined_behavior`

### 0.11 → 0.12

- Changed behavior of `#[ffi_service_method]`
    - `#[ffi_service_method(direct)]` is now `#[ffi_service_method(wrap = "direct")]`

### 0.10 → 0.11

- C# backend switched constructors to static methods
    - Wherever you used `new Service(x)` now use `Service.NewWith(x)` (or similar).

### 0.9 → 0.10

- C# backend split into `DotNet` and `Unity`. If methods are missing:
    - Add `.add_overload_writer(DotNet::new())` to `Generator`.
    - Consider adding `.add_overload_writer(Unity::new())` when targeting Unity

### 0.8 → 0.9

- Replaced most `pattern!` macros with `#[pattern]` attributes, see individual pattern documentation for details.
- Added type hints support, upgraded minimum supported Python version to 3.7 [no workaround]
