
Implement the `#[ffi]` attribute macro inside `proc_macros` for constants. I already started an empty function `ffi_type` in `types/mod.rs` for you.

You can see examples how the macro is used in the file `ty_basic.rs`, and the code must compile when you are done.

Requirements

1) Essentially, the macro must do two things:
   - Transform a struct into a `#[repr(C)]` struct 
   - Transform an enum into a `#[repr(C)]` enum with a u32 variant selector 
   - Properly implement `TypeInfo` for the given type, compare below
2) The macro must support structs and enums, generic and lifetime parameters. If generic parameters are used, it 
   can assume the parameters themselves implement `TypeInfo` (e.g., the user will have specified T: TypeInfo)
3) It must support a number of parameters (see the examples), these include
   - #[ffi_type], without parameter, it should transform the type as above
   - #[ffi_type(packed)], the type should have packed alignment instead
   - #[ffi_type(name = "Foo")], the internal name as set in `Type` should be `Foo` 
   - #[ffi_type(module = "abc")], the module name as set in `Type` should be `abc`
     #[ffi_type(transparent)], the type should be repr transparent 
   - #[ffi_type] ... #[skip], if a `#[skip]` attribute is found on a field it should be ignored when emitting Type
   - #[ffi_type(service)], the type kind should be set as `Service`, no other attributes should be considered or registered 
   - #[ffi_type(opaque)], the type should be repr opaque
   - #[ffi_type(debug)], just before returning the resulting token stream, it should be pretty-printed to the console
   
   It is possible to combine these attributes where sensible (e.g. #[ffi_type(packed, name = "Abc")]) in any order.   

4) For each field (except ignored ones) the field type must also be registered by the macro.
5) You should not use any external crates except the ones present (prettyplease, syn, proc_macro2, quote)
6) Keep the code nice and human readable. Files should not be too long, do not add excessive documentation. 
   First abstract the argument parsing into some struct. Then build a model of the elements to emit, then emit them. Create
   helper files inside `types/` (in the proc macro) if needed to compartmentalize the code. Keep the code inside `mod.rs` 
   minimal, instead, add logic to appropriately named files next to `mod.rs`.
7) You should tracks spans properly and otherwise crate a "well behaved" proc macro. If the macro determins that something 
   will not work out it should emit a compiler error.
8) Reuse as much code between enum and struct parsing as sensible.
9) You can assume the `interoptopus` crate is in scope for emitting code. That said, be nice and prefix all items with 
   the fully qualified path, e.g., `::interoptopus::foo`, `::std::option::Option`, ...
10) Enums only need to support `A`, `B = 123` and `C(T)` style variants.
11) When emitting a type, do not re-build it from your model, but just emit it as is (possibly fixing the attached attributes as needed)

About emitting TypeInfo:

Start by looking at the existing TypeInfo trait. It should be, along with Type:

```rust
pub trait TypeInfo {
    const WIRE_SAFE: bool;
    const RAW_SAFE: bool;

    fn id() -> TypeId;
    fn kind() -> TypeKind;
    fn ty() -> Type;

    fn register(inventory: &mut Inventory);
}

pub struct Type {
   pub name: String,
   pub visibility: Visibility,
   pub docs: Docs,
   pub emission: Emission,
   pub kind: TypeKind,
}
```
When implementing this trait, consider this:
- WIRE_SAFE should be a big "AND" combination of all used fields or variants
- RAW_SAFE is the same
- id() should produce unique / random ID per type. You can use the `type_id!` helper macro 
- kind() should be a ::Enum, ::Struct, ::Opaque or ::Service, depending on what you emit
- For the Type `ty()` you emit:
  - the name should either be the name of the type, or the name attribute if given 
  - if type parameters are present you should concat them like so `Foo<T, U>` you should emit a name that would become `Foo{Tname}{Uname}`
  - the visibility should match the declared one of the type
  - the docs should be the `///` documentation attached to the type,
- register() should register the type, and prior to that all fields or variants if they have types.


