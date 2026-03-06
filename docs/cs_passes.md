

Help me implement the conversion logic in this file, it is part of a bigger converstion logic that should transform Rust `TypeKind::Struct` entries to a C# `TypeKind::Composite` ones.

This pass is meant to convert the field description.

For each `TypeKind::Struct` found, it should create and store an entry in a local hashmap `TypeId => Vec<Field>` hashmap 




# Background 

This project is called `interoptopus` and meant to generate FFI bindings in foreign languages for Rust code. It works as follows:

- Users author some Rust code and add `#[ffi]` attributes to functions or types, which will ensure they have the right `repr`, signatures, and perform various safety checks. These macros also implement helper traits.
- Users can then collect all generated items in an `interoptopus::inventory::Inventory`. We have two inventory types, `interoptopus::inventory::RustInventory` and `interoptopus::inventory::ForeignInventory` for "forward" and "reverse" interop respectively, but right now we only concern ourselves with the Rust inventory (that is, Rust functions exported to a foreign language).
- The inventory contains a list of `interoptopus::lang` items (types, functions), always in the form of `HashMap<SomeId, Item>`, for example `pub type Types = HashMap<TypeId, Type>;` and `pub type Functions = HashMap<FunctionId, Function>;`. The idea is that anything in an inventory / FFI boundary has some ID so we can "talk" about it, and link it from other items. For example, a `struct` type with a bunch of fields stores the target `TypeId` for each field which in turn can be obtained from that inventory / hashmap.
- Note that backends have their own models and `lang` type descriptors and ID space to prevent errors and only model what they know.
- This inventory is eventually passed to a backend, such as the `backend_csharp` crate. Each backend is meant to generate (for whatever language it targets) FFI bindings in that language for whatever was in the inventory. For example, the C# backend might create a `Foo()` and interop fluff function for a `foo()` that's contained in a Rust FFI library / inventory.
- Backends accomplish this via a transformation pipeline that is inspired by how compilers work. For example, the C# `backend_csharp::RustLibrary` pipeline primarily contains a bunch of 'passes'. The idea here being, each pass somehow transforms (or derives) parts of the inventory into some backend-specific model (possibly iteratively). Once all modelling passes are complete there should be a complete "output" model that describes how semantically an interop definition should be written. For example, the model might decide that some Rust struct becomes a non-blittable class, or some other Rust struct becomes a blittable C# struct (depending on the fields). Eventually, once that model is complete, a number of output passes transform this model to various strings, and again iteratively compose these strings to an ultimate `Interop.cs` file. 
- For each of these passes the idea is they should be small and mostly isolated. They might have dependencies on previous passes  (and are even allowed to modify a previous passes data via &mut references), but generally passes are where sanity and separation of concerns should be addressed, so that not everything clumps together in massive logic modules. Also, we might have other pipelines later which are based on different kinds of inventory, and therefore might have different stages, although some stages are probably shared, so passes are a bit careful about what types they accept and how they compose. 
- Model passes (but not output passes) can be called multiple times, inside one big loop (also see below). Effectively, all model passes are run once. If any model pass signaled that it changed anything inside all model passes will be run again (and again, and again ...) until all model passes signalled that nothing changed anymore. This allows the simplication of pass logic with nested data (e.g., some type depends on other types to be process, which in turn depends on ...) so they can just do some work, and then later do some more. 


# Passes

Passes always have something of this structure. The naming of `Config`, `Pass`, `new` and `process` is always the same. Higher up we reference them via their parent module, e.g., `foo:Pass` and `bar::Pass`. 

```rust
// This config allows external users to change how the pass works. It must exist, but may be empty.
#[derive(Default)]
pub struct Config {}

// Any pass-specific storage goes in here
pub struct Pass {}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self {}
    }

    // Process for model passes
    // - always is called `process`
    // - always takes &mut self
    // - can take any number of needed dependencies (mostly previous passes, or inventory fields)
    // - must return some `ModelResult` which is `Result<Outcome, Error>`. That outcome is an `enum Outcome { Unchanged, Changed }`.
    // - if the pass did not change (or is a one-shot pass that after the first time will never change) it must return `Unchanged`
    // - if the pass changed something inside (and / or thinks it should be called again) it must return `Changed`. This could happen if
    //   for example it tries to process some struct, but some fields aren't available yet. 
    pub fn process(&mut self, id_map: &mut model_id_maps::Pass, kinds: &mut model_type_kinds::Pass, rs_types: &interoptopus::inventory::Types) -> ModelResult {
        Ok(Unchanged)
    }
}

```


