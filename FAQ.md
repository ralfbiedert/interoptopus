# FAQ


## Misc

- **Why do I get `error[E0658]: macro attributes in #[derive] output are unstable`?**

  This happens when `#[ffi_type]` appears after `#derive[...]`. Just switch their order.


- **Why do I get `use of undeclared crate or module` in `inventory_function!()`?**

  You probably forgot to declare one of your items `#[ffi_function]` or `#[ffi_const]`.


- **Who's the target audience for this?**
  
  Anyone writing 'system libraries' that need to be consumable from multiple 
  environments at the same time. Our usage profile is:
  
  - several libraries with 100+ functions (think Vulkan, OpenXR), 
  - callable from Unity, Unreal, Python,
  - high "API uncertainty" in the beginning, requiring lots of iterations to get it right,  
  - predictable and "industry-grade" APIs in the end (final bindings ship to customers).


- **Where can I ask questions?**

  Use [Github Discussions](https://github.com/ralfbiedert/interoptopus/discussions). 


- **How does it work?**

  As  [answered by Alex Hirsekorn](https://www.quora.com/How-does-an-octopus-eat-a-crab-without-getting-cuts?share=1):
  - When a GPO [Giant Pacific Octopus] finds a crab it does something called a “flaring web-over” which uses the webbing between the arms to engulf the crab while simultaneously immobilizing the crab’s claws with its suckers.
  - With the crab in what amounts to a sealed bag the GPO spits one of its two types of saliva into that space. This first saliva is called cephalotoxin and acts as a sedative, rendering the crab unconscious but still alive. [If the crab is taken away from the GPO at this point it will wake up and run away.]
  - The GPO then spits the other kind of saliva into the crab; that saliva is a powerful digestive enzyme. Since the crab is still alive at this point it pumps that enzyme throughout its body and basically digests itself on the GPO’s behalf. The octopus typically takes a nap during this stage.
  - After some period of time (I’ve seen this vary from 1.5 to 3 hours) the GPO wakes up, disassembles the crab, and licks out what amounts to crab meat Jell-O with a tongue-like structure called a radula. As Kathleen said the GPO does the disassembly with its suckers but it doesn’t just open the carapace: It will also take the claws and legs apart at the various joints.
  - When the meal is finished and the shell parts tossed out the GPO will take another nap until it’s hungry again. [Studies have shown that a GPO spends as much as 70% of its time sleeping in its den.]

  Occasionally a GPO will get minor injuries from capturing the crab but, for the most part they are too careful and too skilled for that to be much of an issue.

  After the GPO has rested, FFI bindings are produced.


- **How can I add a new pattern?**
  
  Adding support for new patterns is best done via a PR. Patterns mimicking common Rust features 
  and improvements to existing patterns are welcome. As a rule of thumb they 
  should 
  - be idiomatic in Rust (e.g., options, slices),
  - have a safe and sound Rust implementation and 'reasonable' usability in other languages, 
  - be useful in more than one backend and come with a reference implementation,
  - _must_ fallback to C primitives (e.g., 'class methods' are functions with receivers).
  
  As an alternative, and discouraged for public backends, you might be able to get away using "tags".
  


## New Backends

**Quickstart**

1) start a crate
1) copy code of whatever backend comes closest (e.g, C)
1) from `Interop::write_to` produce some output, fix errors as they appear
1) create UI test against `interoptopus_reference_project` to ensure quality

**Some Tips**

Once you understand how Interoptopus abstracts APIs writing a backend is quite simple: 

- The `Library` is _the input_ to any backend, as in, it fully describes what you should generate. 
  It mainly consists of these elements:
  - Types
  - Functions
  - Constants
  - Patterns
- Any backend more or less just converts each of these things one-by-one. It usually writes all constants,
  then all (composite) types and enums, then all functions, then (optionally) all patterns. 
  
- Writing and converting types is usually the most tricky part, and might require that you sort types by 
  dependencies (e.g., for C) or handle types differently depending on where they appear (e.g., in C# an 
  `IntPtr` in a field might become a `ref T` in a function).

- Patterns are fully optional. You can always just implement their "fallback type" 
  (e.g, an AsciiPointer is just a `*const u8`) and call it a day. However, when exporting larger APIs 
  (like 100+ functions) producing _idiomatic_ pattern bindings will be a good investment. 

**How long will it take?**

Judging from creating the existing backends, and assuming you've done some FFI
calls from that language to a C library, I'd say:

- **1h** - browsing an existing backend and understanding how CTypes work
- **2h** - producing MVP output that can call a single `hello_world()`
- **4h** - generate bindings for arbitrary functions with primitive parameters
- **1d** - also produce `structs` and `enums`
- **2d** - support the entire C API surface
- **3-5d** - have clean, idiomatic wrappers for all patterns and run automated reference tests



## Safety, Soundness, Undefined Behavior

This library naturally does "unsafe" things and any journey into FFI-land is a little adventure.
That said, here are some assumptions and quality standards this project is based on:

- Safe Rust calling safe Rust code must always be sound, with soundness boundaries
on the module level, although smaller scopes are preferred. For example, creating a `FFISlice`
from Rust and directly using it from Rust must never cause UB.

- We must never willingly generate broken bindings. For _low level types_ we must never
generate bindings which "cannot be used correctly" (e.g., map a `u8` to a `float`), for
_patterns_ we must generate bindings that are "correct if used according to specification".

- There are situations where the (Rust) soundness of a binding invocation depends on conditions outside
our control. In these cases we trust foreign code will invoke the generated functions
correctly. For example, if a function is called with an `AsciiPointer` type we consider it _safe and sound_
to obtain a `str` from this pointer as `AsciiPointer`'s contract specifies it must point to
ASCII data.

- Related to the previous point we generally assume functions and types on both sides are used _appropriately_ w.r.t.
Rust's FFI requirements and we trust you do that, e.g., you must declare types `#[repr(C)]` (or similar)
and functions `extern "C"`.

- Any `unsafe` code in any abstraction we provide should be "well contained", properly documented
and reasonably be auditable.

- If unsound Rust types or bindings were ever needed (e.g., because of a lack of Rust specification,
like 'safely' mapping a trait's vtable) such bindings should be gated behind a feature flag
(e.g., `unsound`) and only enabled via an explicit opt-in. Right now there are none, but this is
to set expectations around discussions.


tl;dr: if it's fishy we probably want to fix it, but we rely on external code calling in 'according to documentation'. 


## Errors vs. Panics

Any Rust code ...

- ... likely used in applications **must not** panic, unless a panic is clearly user-requested 
  (e.g., having an `unwrap()` on an `FFIOption`). If a function can fail it must return a 
  `Result`. Allocations & co. are currently exempt if they lack a good `Result`-API; 
  although the plan is to replace them eventually.
  
- ... reasonably only used during compilation or unit tests (e.g., proc macro code or code generation 
  helpers) **should panic**, if panicking can lead to a better developer experience (e.g., 
  clearer error messages). 


## Licensing

A clarification how the license is meant to be applied:

- Our license only applies to code **in** this repository, not code generated **by** this repository.
- We do not claim copyright for code produced by backends included here; even if said code was based on a template in this repository.
- For the avoidance of doubt, anything produced by `Interop::write_to` or any item emitted by a proc macro is considered “generated by”.