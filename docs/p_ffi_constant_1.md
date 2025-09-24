
Implement the `#[ffi]` attribute macro inside `proc_macros_impl` for constants. 
I already started an empty function `ffi` in `constant/mod.rs` for you.

Consts work like this

```rust
#[ffi]
const X: u8 = 123;
```

they should emit `Constant` 

```rust
pub trait ConstantInfo {
    fn id() -> ConstantId;
    fn constant() -> Constant;
    fn register(inventory: &mut Inventory);
}

/// The value of a constant.
#[derive(Clone, Debug, PartialOrd, PartialEq, Hash)]
pub enum ConstantValue {
    Primitive(PrimitiveValue),
}

pub struct Constant {
    pub name: String,
    pub visibility: Visibility,
    pub docs: Docs,
    pub emission: Emission,
    pub ty: TypeId,
    pub value: ConstantValue,
}
```
When registering a constant, you should also register its type.


Also 

1) You should not use any external crates except the ones present (prettyplease, syn, proc_macro2, quote)
2) Keep the code nice and human readable. Files should not be too long, do not add excessive documentation. 
   First abstract the argument parsing into some struct. Create
   helper files inside `constant/` (in the proc macro) if needed to compartmentalize the code. Keep the code inside `mod.rs` 
   minimal, instead, add logic to appropriately named files next to `mod.rs`.
3) You should tracks spans properly and otherwise crate a "well behaved" proc macro. If the macro determines that something 
   will not work out it should emit a compiler error.
4) Reuse as much code between enum and struct parsing as sensible.
5) You can assume the `interoptopus` crate is in scope for emitting code. That said, be nice and prefix all items with 
   the fully qualified path, e.g., `::interoptopus::foo`, `::std::option::Option`, ...

