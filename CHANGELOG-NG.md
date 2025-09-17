

# What
- Enable reverse Interoptopus (e.g., allow Rust app to load C# module)
  - C# backend will ship with `Runtime` type that can load some `plugin.dll` to fulfil an `#[ffi_plugin]`  
- Better support `Wire<T>` types what people today use `protobuf` for
- Better support "user provided external" types that Interoptopus will use, but won't emit 
- Better inventory manipulation (e.g., add, remove, modify types after-the-fact) 
- Better multi-file Interop support 
- Model-based ('smart') backend emission
  - Today all items are emitted 'ad-hoc', one part of code doesn't know what other part will emit, it's all implied 
    knowledge in the code. For examples, knowing what functions are overloaded in C# is not possible / painful.  
  - Instead, backends can now process and transform an inventory in multiple stages, 'enriching' type and functions
    with metadata, and use that to emit code in a coherent manner.

# How

WIP
- [ ] Totally rewritten `Inventory`
  - Type information is now flattened and unique (no more massively nested `Type` structs) 
- [ ] Totally rewritten `lang` meta hierarchy 
  - Lightweight & clean
  - Meta is in orthogonal POROs now, all public fields, no fluff accessor methods
  - 
TODO (vaguely in this order)
- [ ] Change proc macros to emit new-style traits
- [ ] Clean up proc macros 
- [ ] Introduce helper functions to work with new style `Inventory` (query, filter, replace)
- [ ] Change C# backend to use new style `Inventory` and switch to model-based approach
- [ ] Rework C# backend to emit items needed by Runtime `#[ffi_plugin]` use
- [ ] Write `#[ffi_plugin]` macro  
- [ ] Write plugin code in C# backend 
- [ ] (Re-)implement `Runtime` for C# backend using our own logic  
 

# Where
- `inventory` branch