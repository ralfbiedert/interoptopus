# Upgrade Instructions

Tips for solving non-trivial breaking changes when upgrading from previous versions.

### 0.15-alpha.17 -> 0.15 

Around -alpha.18 we did a massive update under the hood, which also changed some <alpha.17 behavior. If you upgrade 
from 0.14 you should read the `0.14 → 0.15-alpha.17` section first, then come here and see what else changed. 
If you far worked with some 0.15-alpha, check this list out: 

- `#[ffi_type]`, `#[ffi_function]`, ... all of these have been removed. You just do `#[ffi]` now.
- ffi functions do not check for panics anymore (unless asked, TBD)
- The entire `Inventory` internal machinery has been rewritten from scratch. If you relied on delicate inventory
  manipulation please have a look at the new `Inventory` APIs and file a issue / PR if something is missing.
- Checking of forbidden names now happens inside the macro at compile time with much better diagnostics.
- Many constructs are now forbidden that were previously allowed but 'dodgy'. In most cases you should get a clear
  error message if that happens.
- It's `Inventory::new()` now, and then `.validate()` instead of `.build()`
- Services now need #[ffi(service)] on the type, not `opaque`
- The "namespace" concept has been fully reworked. Under the hood it's called `emission` now, and should
  give much more fine grained control which symbol goes where when writing interop. In addition, there is now native
  support to tell it a symbol is "external" (aka, user-provided). In essence that will mean Interoptopus assumes
  the symbol is defined (and will try to use it) but it's up to you to provide the definition.
- Some #[ffi] attributes were renamed check the #[ffi] documentation for details



### 0.14 → 0.15-alpha.17

#### Core Library

- Exported functions (`#[ffi_function]`) don't need to specify `#[no_mangle]` or `extern "C"` anymore as these will be
  added automatically.
- Exported types (`#[ffi_type]`) must not specify `#[repr(...)]` anymore as we will handle that. If you need custom
  attributes you can, for example, do `#[ffi_type(transparent)]` or `#[ffi_type(packed)]`.
- In service definitions, providing a `prefix` is generally not needed anymore.
- Service methods that return void (`()`) can now be used without a `#[ffi_service_method]` annotation. On a Rust panic
  they will silently return. If this is not acceptable you must return `Result` or specify a different panic behavior
  via that attribute.
- `#[ffi_type(patterns(ffi_error))]` is now `#[ffi_type]` only.
- `#[ffi_service_ctor]` has been removed and is inferred.
- `AsciiPointer` is now called `CStrPointer`, since it can contain non-ASCII data (e.g., when called from C#).
- We fixed capitalization in some backends, e.g., a `Sliceu8` is now `SliceU8`.
- It's now `Inventory::new()` to create an `Inventory`, and you directly register on it. Call `validate()` when done.
- Skipping fields now works with `#[skip]`
- Services are now `#[ffi_type(service)]` and `#[ffi_service]`
- Surrogates now work through the `Surrogate<T, L>` type.
    - Previously you needed to specify `#[ffi_surrogates(some_field = "some_foreign_type")]`
    - Instead, you now make `some_field` of type `Surrogate<Foreign, Local>`
- Setting alignment on types is not supported anymore (for now). You should also stop using alignment on
  earlier versions as various backends didn't translate that properly.
- Backend-related testing functions were moved into an internal `tests` project, as the code was mostly specific to our
  project needs anyway.
- The core generation trait was renamed from `Interop` to `GenerateInterop`.
- Items were generally renamed from `FFIxxx` to `ffi::Xxx`
- The preferred way to return errors is now via `ffi:Result`
- We now support UTF-8 strings via `ffi::String`
- We support async-async calls (from C#)
- Enums can now carry data. For now only `E::A` and `E::B(T)` are supported.
- `ffi::Result` and `ffi::Option` are now based on enums
- Support for the old `FFIErrorEnum` style has been removed. Use `ffi::Result` instead. Error patterns are now much easier to implement.
- Likewise, `#[ffi_type(error)]` on enums has been removed.
- Some item names (e.g., `public`) are forbidden now as they might conflict with backend language-specific keywords, and
  will be flagged when invoking `.validate()`.


#### All Backends

- The type structure has been greatly simplified. Each backend now only has a single entry struct `InteropBuilder` you have to deal with.
- Documentation generators were generally renamed to `Markdown` since that's what they emit.

#### C# Backend

- Unity support was dropped due to a lack of bandwidth on my side and compat issues,
  compare [this issue](https://github.com/ralfbiedert/interoptopus/issues/133). If you need
  Unity support you should stick with 0.14, wait for Unity to support .NET 8.0, or fork the C# backend. For the latter
  option I happily help you get started, please post a comment in the linked issue.
- ~~If you used the C# `DotNet` or `Unity` overload writer, these helpers now take their own configuration
  where appropriate. If you previously only did `DotNet::new()` this became `DotNet::new_built()`.~~
- The concept of overload writers was removed entirely.
- The minimum supported C# version was bumped to 8.0
- We're using `LibraryImport` instead of `DLLImport` now, which should be significantly faster
- The concept of `Safe` vs `Unsafe` bindings was removed. Everything is `unsafe` now. This means in your
  `.csproj`
  you should set `<AllowUnsafeBlocks>true</AllowUnsafeBlocks>`. This greatly simplified some code, and if you haven't
  used unsafe before you'll get a massive speed boost. If you shipped safe bindings to customers without a `.csproj`
  file so far, you should now ship them with a `.csproj` file that enables that flag instead.
- Arrays are properly supported!
- Async calls are supported
- Exception in callbacks should be properly handled
- `String,` `Result`, `Option` and similar can now be deeply nested and generally work as expected.

#### Python / C Backend

- Contributors wanted! Right now I don't have much bandwidth to work on these backends, so they lack support for
  many of the new features.

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
