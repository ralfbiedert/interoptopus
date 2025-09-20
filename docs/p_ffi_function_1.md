
Implement the `#[ffi_function]` attribute macro inside `proc_macros`. I already started an empty function `ffi_function` in `function/mod.rs` for you.

You can see examples how the macro is used in the file `fn_basic.rs`, and the code must compile when you are done.

Requirements

1) Essentially, the macro must do two things:
   - Emit the function with slight modifications:
        - it has to be callable from C, by adding `extern "C"` 
        - it must be `#[unsafe(no_mangle)]` 
        - it must have an `#[unsafe(export_name = "...")]`
   - emit a companion type named exactly as the function, e.g., a `fn foo()` should have a `struct foo{}` type 
   - Properly implement `Function` for the newly generated type, compare below
2) The macro must support all valid functions that could be observed at an FFI boundary:
   - regular functions `fn foo() {}`
   - return types `fn foo() {} -> u32`
   - parameters w. generics `fn foo(x: Option<&u32>) {} -> Option<&u32>`
   - function with lifetimes `fn foo<'a>(x: Option<&'a u32>) {} -> Option<&'a u32>`
   - it does NOT have to support functions with regular generics `fn foo<T>(x: T)`, since such a function isn't real in the FFI sense. 
3) It must support a number of parameters (see the examples), these include
   - #[ffi_function], without parameter, it should emit the code as mentioned above
   - #[ffi_function(debug)], just before returning the resulting token stream, it should be pretty-printed to the console
   - #[ffi_function(export = unique)], although the function and type should be emitted as-is, the export_name should be the current function, with a 
      (pseudo)random suffix (e.g., `foo()` -> `foo_12313`)    
   - #[ffi_function(export = "abc")], the export_name should be `abc` instead
     #[ffi_function(module = "foo")], inside the Function struct the Emission::Module variant should be used
     #[ffi_function(module = common)], inside the Function struct the Emission::Common variant should be used
   
   It is possible to combine these attributes where sensible (e.g. #[ffi_function(debug, module = "abc")]) in any order. 
   Modules and export variants can only be specified once, and you should emit an error if they are not.
4) As mentioned you must also emit a `struct foo{}` type for a `foo()` function. This type:
    - must have the same visibility as the function
    - you must emit `FunctionInfo` for said type, see below
5) You should not use any external crates except the ones present (prettyplease, syn, proc_macro2, quote)
6) Keep the code nice and human readable. Files should not be too long, do not add excessive documentation. 
   First abstract the argument parsing into some struct. Then build a model of the elements to emit, then emit them. Create
   helper files inside `function/` (in the proc macro) if needed to compartmentalize the code. Keep the code inside `mod.rs` 
   minimal, instead, add logic to appropriately named files next to `mod.rs`. Your code should align with how the `types`
   macro works already.
7) You should tracks spans properly and otherwise crate a "well behaved" proc macro. If the macro determines that something 
   will not work out it should emit a compiler error.
8) You can assume the `interoptopus` crate is in scope for emitting code. That said, be nice and prefix all items with 
   the fully qualified path, e.g., `::interoptopus::foo`, `::std::option::Option`, ...
9) If the user has already specified `extern "..."` on the function or `#[no_mangle]` you should emit an error 

About emitting FunctionInfo:

Start by looking at the existing FunctionInfo trait. It should be, along with Function:

```rust
pub trait FunctionInfo {
    fn id() -> FunctionId;
    fn signature() -> Signature;
    fn function() -> Function;

    fn register(inventory: &mut Inventory);
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Argument {
    pub name: String,
    pub ty: TypeId,
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Signature {
    pub arguments: Vec<Argument>,
    pub rval: TypeId,
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Function {
    pub name: String,
    pub visibility: Visibility,
    pub docs: Docs,
    pub emission: Emission,
    pub signature: Signature,
}
```
When implementing this trait, consider this:
- id() should produce unique / random ID per function. You can use the `id!` helper macro to produce an id using the type you generated.  
- signature() should reflect the functions signature 
- For the `function()` you emit:
  - the name should match the `export_name` 
  - the visibility should match the declared one 
  - the docs should be the `///` documentation attached to the function,
  - emission as specified via `module`
- register() should register the function, and prior to that all parameter and return values present. 
  Functions without return values should always register `()`. 


