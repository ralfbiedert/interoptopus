
You need to change the `ffi` proc macro how it is applied to types.

In addition to emitting a `TypeInfo` impl, it should also emit a `WireIO` impl.

The WireIO trait looks like so

```rust
pub trait WireIO {
    fn write(&self, out: &mut impl Write) -> Result<(), SerializationError>;
    fn read(input: &mut impl Read) -> Result<Self, SerializationError>
    where
        Self: Sized;
    fn live_size(&self) -> usize;
}
```

Implement it as follows:

When applied on enums each function should just invoke bad_wire!(), which panics. 

When applied on structs it works as follows:

- live_size should interrogate each field and sum up their live_sizes. 
- write should forward a write call to each field, in the order of declared fields
- read should instantiate each field via that field type read.

You can generally assume `WireIO` is implemented for each field, but you should emit a `const _:() { ... }` check for each field asserting `WireIO` is implemented for that type. When emitting that check make sure, as always, that any span of that checking code actually refers to the field's originally declared type.


## Misc

1) You should not use any external crates except the ones present (prettyplease, syn, proc_macro2, quote)
2) Keep the code nice and human readable. Files should not be too long, do not add excessive documentation. 
   First abstract the argument parsing into some struct. Then build a model of the elements to emit, then emit them. If possible, reuse part of the existing model. Create helper files inside `types/` (in the proc macro) if needed to compartmentalize the code. Keep the code inside `mod.rs` 
   minimal, instead, add logic to appropriately named files next to `mod.rs`. Your code should align with how the other macros work 
   already.
3) You should tracks spans properly and otherwise crate a "well behaved" proc macro. If the macro determines that something 
   will not work out it should emit a compiler error.
4) You can assume the `interoptopus` crate is in scope for emitting code. That said, be nice and prefix all items with 
   the fully qualified path, e.g., `::interoptopus::foo`, `::std::option::Option`, ...
 




